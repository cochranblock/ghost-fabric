mod config;
mod inference;
mod mesh;
mod radio;
mod sensor;

use clap::{Parser, Subcommand};

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

/// f1=start — start the mesh node
fn f1() {
    let cfg = config::f4();
    let cfg = match cfg {
        Some(c) => c,
        None => {
            eprintln!("No node config found. Run `ghost-fabric init` first.");
            std::process::exit(1);
        }
    };
    println!("Starting node: {}", cfg.node_id);
    println!("Radio: {} MHz, SF{}", cfg.frequency_mhz, cfg.spreading_factor);

    // Module status
    println!("\nSubsystems:");
    println!("  radio:     {}", radio::f5());
    println!("  mesh:      {}", mesh::f6());
    println!("  inference: {}", inference::f7());
    println!("  sensor:    {}", sensor::f8());
    println!("\nNode ready. Waiting for implementation.");
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
