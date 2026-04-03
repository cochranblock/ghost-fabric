use clap::{Parser, Subcommand};
use ghost_fabric_core::{config, lifecycle, radio, mesh, inference, sensor};
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

    println!("\nSubsystems:");
    println!("  radio:     {}", radio::f5());
    println!("  mesh:      {}", mesh::f6());
    println!("  inference: {}", inference::f7());
    println!("  sensor:    {}", sensor::f8());

    // Main loop — runs until SIGINT (Ctrl+C)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc_handler(r);

    println!("\nNode running. Press Ctrl+C to stop.");
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("\nShutting down...");
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
