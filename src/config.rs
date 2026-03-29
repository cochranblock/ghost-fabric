use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// T0=NodeConfig — node identity and radio parameters
#[derive(Serialize, Deserialize)]
pub struct T0 {
    pub node_id: String,
    pub frequency_mhz: u32,
    pub spreading_factor: u8,
    pub bandwidth_khz: u32,
}

impl Default for T0 {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let id: u64 = rng.r#gen();
        Self {
            node_id: format!("gf-{:016x}", id),
            frequency_mhz: 915,
            spreading_factor: 7,
            bandwidth_khz: 125,
        }
    }
}

/// f9=config_path — returns the config file path
pub fn f9() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ghost-fabric")
        .join("node.json")
}

/// f3=init — generate node config and write to disk
pub fn f3() {
    let path = f9();
    if path.exists() {
        println!("Config already exists: {}", path.display());
        if let Some(cfg) = f4() {
            println!("Node ID: {}", cfg.node_id);
        }
        return;
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create config directory");
    }

    let cfg = T0::default();
    let json = serde_json::to_string_pretty(&cfg).expect("failed to serialize config");
    fs::write(&path, &json).expect("failed to write config");

    println!("Initialized node config: {}", path.display());
    println!("Node ID: {}", cfg.node_id);
}

/// f4=load — load node config from disk
pub fn f4() -> Option<T0> {
    let path = f9();
    let data = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&data).ok()
}
