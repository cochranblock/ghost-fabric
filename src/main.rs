use clap::{Parser, Subcommand};
use ghost_fabric_core::{config, lifecycle, mesh, packet, radio};
use ghost_fabric_core::mesh::T2 as _;
use ghost_fabric_core::radio::T1 as _;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Ghost Fabric — sovereign edge intelligence over sub-GHz cognitive mesh
#[derive(Parser)]
#[command(name = "ghost-fabric", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the mesh node
    Start,
    /// Show node status and identity
    Status,
    /// Initialize node config (generates node ID on first run)
    Init,
}

/// f0=main — entry point
fn f0() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => config::f3(),
        Some(Commands::Start) => f1(),
        Some(Commands::Status) => f2(),
        None => {
            println!("ghost-fabric v{}", env!("CARGO_PKG_VERSION"));
            println!("Run `ghost-fabric --help` for usage.");
        }
    }
}

/// f1=start — start the mesh node with hot reload lifecycle
fn f1() {
    let cfg = config::f4();
    let cfg = match cfg {
        Some(c) => c,
        None => {
            eprintln!("No node config found. Run `ghost-fabric init` first.");
            std::process::exit(1);
        }
    };

    if let Err(e) = config::f17(&cfg) {
        eprintln!("Invalid config: {}", e);
        std::process::exit(1);
    }

    // Hot reload: acquire PID lock, SIGTERM old instance
    lifecycle::f14();
    println!("PID {} acquired lock at {}", std::process::id(), lifecycle::f13().display());

    println!("Starting node: {}", cfg.node_id);
    println!("Radio: {} MHz, SF{}", cfg.frequency_mhz, cfg.spreading_factor);

    // Initialize subsystems
    let mut mock_radio = radio::T8::new();
    if let Err(e) = mock_radio.init(cfg.frequency_mhz, cfg.spreading_factor, cfg.bandwidth_khz) {
        eprintln!("Radio init failed: {}", e);
        std::process::exit(1);
    }

    let mut peer_table = mesh::T9::new();
    let mut seq: u16 = 0;

    println!("\nSubsystems:");
    println!("  radio:     {}", mock_radio.status());
    println!("  mesh:      {}", peer_table.status());

    // Main loop — runs until SIGINT (Ctrl+C)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc_handler(r);

    println!("\nNode running. Press Ctrl+C to stop.");
    let mut tick: u64 = 0;
    while running.load(Ordering::SeqCst) {
        // Every 10s: broadcast beacon
        if tick.is_multiple_of(10) {
            let beacon = packet::T12::f20(&cfg.node_id, 100, 1, seq);
            if let Ok(bytes) = packet::f18(&beacon) {
                let _ = mock_radio.send(&bytes);
                seq = seq.wrapping_add(1);
            }
        }

        // Poll radio for incoming packets
        if let Ok(Some(data)) = mock_radio.recv(0)
            && let Ok(frame) = packet::f19(&data)
        {
            f22(&frame, &cfg.node_id, &mut peer_table, &mut mock_radio, &mut seq);
        }

        // Every 60s: evict stale peers
        if tick.is_multiple_of(60) && tick > 0 {
            let evicted = peer_table.evict_stale(300);
            if evicted > 0 {
                eprintln!("[mesh] evicted {} stale peer(s)", evicted);
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        tick += 1;
    }

    println!("\nShutting down...");
    println!("  peers seen: {}", peer_table.peer_count());
    println!("  packets tx: {}", seq);
    lifecycle::f16();
    println!("Node {} stopped.", cfg.node_id);
}

fn ctrlc_handler(running: Arc<AtomicBool>) {
    #[cfg(unix)]
    {
        // Use libc directly — no extra deps
        unsafe {
            libc::signal(libc::SIGINT, sigint_handler as *const () as libc::sighandler_t);
        }
        // Store the flag in a static for the signal handler
        RUNNING_FLAG.store(
            Arc::into_raw(running) as usize,
            Ordering::SeqCst,
        );
    }
    #[cfg(not(unix))]
    {
        let _ = running;
    }
}

#[cfg(unix)]
static RUNNING_FLAG: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[cfg(unix)]
extern "C" fn sigint_handler(_sig: libc::c_int) {
    let ptr = RUNNING_FLAG.load(Ordering::SeqCst);
    if ptr != 0 {
        let flag = unsafe { Arc::from_raw(ptr as *const AtomicBool) };
        flag.store(false, Ordering::SeqCst);
        // Leak it back so the main thread can still read it
        let _ = Arc::into_raw(flag);
    }
}

/// f22=handle_frame — process an incoming mesh frame
fn f22(
    frame: &packet::T12,
    my_id: &str,
    peers: &mut mesh::T9,
    radio: &mut radio::T8,
    seq: &mut u16,
) {
    // Ignore our own frames
    if frame.src == my_id {
        return;
    }

    match frame.kind {
        packet::T13::Beacon => {
            if let packet::T14::Beacon {
                battery_pct,
                hop_count,
            } = &frame.payload
            {
                let mut peer = mesh::T3::new(&frame.src, -70, *battery_pct);
                peer.hop_count = *hop_count;
                peers.add_peer(peer);
                eprintln!("[mesh] beacon from {} (battery {}%)", frame.src, battery_pct);
            }
        }
        packet::T13::Ping => {
            let pong = packet::T12::pong(
                my_id,
                &frame.src,
                100,
                peers.peer_count() as u8,
                *seq,
            );
            if let Ok(bytes) = packet::f18(&pong) {
                let _ = radio.send(&bytes);
                *seq = seq.wrapping_add(1);
            }
        }
        packet::T13::Pong => {
            if let packet::T14::Pong {
                battery_pct,
                peer_count: _,
            } = &frame.payload
            {
                let peer = mesh::T3::new(&frame.src, -70, *battery_pct);
                peers.add_peer(peer);
                eprintln!("[mesh] pong from {} (battery {}%)", frame.src, battery_pct);
            }
        }
        packet::T13::Data => {
            if frame.dst == my_id || frame.dst == "*" {
                eprintln!("[data] from {}: {:?}", frame.src, frame.payload);
            }
        }
        packet::T13::Ack => {
            eprintln!("[ack] from {} for seq {:?}", frame.src, frame.payload);
        }
    }

    // Relay broadcast frames with TTL
    if frame.is_broadcast() && frame.should_relay() {
        let mut relay = frame.clone();
        if relay.relay_hop()
            && let Ok(bytes) = packet::f18(&relay)
        {
            let _ = radio.send(&bytes);
        }
    }
}

/// f2=status — display node identity and config
fn f2() {
    match config::f4() {
        Some(cfg) => {
            println!("Node ID:    {}", cfg.node_id);
            println!("Frequency:  {} MHz", cfg.frequency_mhz);
            println!("SF:         {}", cfg.spreading_factor);
            println!("Bandwidth:  {} kHz", cfg.bandwidth_khz);
            println!("Config:     {}", config::f9().display());
        }
        None => {
            eprintln!("No node config. Run `ghost-fabric init` first.");
            std::process::exit(1);
        }
    }
}

fn main() {
    f0();
}
