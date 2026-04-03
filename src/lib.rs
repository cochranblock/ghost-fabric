pub mod config;
pub mod inference;
pub mod lifecycle;
pub mod mesh;
pub mod packet;
pub mod radio;
pub mod sensor;
#[cfg(unix)]
pub mod uds_radio;

/// f10=node_status_string — returns full node status as a string
pub fn f10() -> String {
    let mut out = String::new();
    out.push_str(&format!("ghost-fabric v{}\n", env!("CARGO_PKG_VERSION")));

    match config::f4() {
        Some(cfg) => {
            out.push_str(&format!("Node ID:    {}\n", cfg.node_id));
            out.push_str(&format!("Frequency:  {} MHz\n", cfg.frequency_mhz));
            out.push_str(&format!("SF:         {}\n", cfg.spreading_factor));
            out.push_str(&format!("Bandwidth:  {} kHz\n", cfg.bandwidth_khz));
            out.push_str("\nSubsystems:\n");
            out.push_str(&format!("  radio:     {}\n", radio::f5()));
            out.push_str(&format!("  mesh:      {}\n", mesh::f6()));
            out.push_str(&format!("  inference: {}\n", inference::f7()));
            out.push_str(&format!("  sensor:    {}\n", sensor::f8()));
        }
        None => {
            out.push_str("No node config. Run init to generate.\n");
        }
    }
    out
}

/// f11=init_and_report — initialize node and return status string
pub fn f11(data_dir: Option<std::path::PathBuf>) -> String {
    let path = match data_dir {
        Some(dir) => {
            let p = dir.join("ghost-fabric").join("node.json");
            if let Some(parent) = p.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            p
        }
        None => config::f9(),
    };

    if !path.exists() {
        let cfg = config::T0::default();
        let json = serde_json::to_string_pretty(&cfg).expect("serialize");
        std::fs::write(&path, &json).expect("write config");
        format!("Initialized node: {}", cfg.node_id)
    } else {
        match std::fs::read_to_string(&path)
            .ok()
            .and_then(|d| serde_json::from_str::<config::T0>(&d).ok())
        {
            Some(cfg) => format!("Node already initialized: {}", cfg.node_id),
            None => "Config exists but unreadable".to_string(),
        }
    }
}
