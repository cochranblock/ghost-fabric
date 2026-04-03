use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// T0=NodeConfig — node identity and radio parameters
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct T0 {
    pub node_id: String,
    pub frequency_mhz: u32,
    pub spreading_factor: u8,
    pub bandwidth_khz: u32,
    #[serde(default)]
    pub peers: Vec<String>,
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
            peers: Vec::new(),
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

/// f17=validate — check config values are within LoRa spec
pub fn f17(cfg: &T0) -> Result<(), String> {
    if cfg.spreading_factor < 6 || cfg.spreading_factor > 12 {
        return Err(format!("SF {} out of range (6-12)", cfg.spreading_factor));
    }
    let valid_bw = [7, 10, 15, 20, 31, 41, 62, 125, 250, 500];
    if !valid_bw.contains(&cfg.bandwidth_khz) {
        return Err(format!("bandwidth {} kHz not a valid LoRa BW", cfg.bandwidth_khz));
    }
    if cfg.frequency_mhz < 150 || cfg.frequency_mhz > 960 {
        return Err(format!("frequency {} MHz out of range (150-960)", cfg.frequency_mhz));
    }
    if !cfg.node_id.starts_with("gf-") || cfg.node_id.len() < 5 {
        return Err("node_id must start with 'gf-' and be at least 5 chars".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let cfg = T0::default();
        f17(&cfg).unwrap();
        assert!(cfg.node_id.starts_with("gf-"));
        assert_eq!(cfg.frequency_mhz, 915);
        assert_eq!(cfg.spreading_factor, 7);
        assert_eq!(cfg.bandwidth_khz, 125);
        assert!(cfg.peers.is_empty());
    }

    #[test]
    fn config_round_trip() {
        let cfg = T0::default();
        let json = serde_json::to_string(&cfg).unwrap();
        let loaded: T0 = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg.node_id, loaded.node_id);
        assert_eq!(cfg.frequency_mhz, loaded.frequency_mhz);
    }

    #[test]
    fn config_backward_compat() {
        // Old configs without peers field should still deserialize
        let json = r#"{"node_id":"gf-test","frequency_mhz":915,"spreading_factor":7,"bandwidth_khz":125}"#;
        let cfg: T0 = serde_json::from_str(json).unwrap();
        assert!(cfg.peers.is_empty());
    }

    #[test]
    fn validate_bad_sf() {
        let mut cfg = T0::default();
        cfg.spreading_factor = 13;
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_bad_bw() {
        let mut cfg = T0::default();
        cfg.bandwidth_khz = 100;
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_bad_freq() {
        let mut cfg = T0::default();
        cfg.frequency_mhz = 5000;
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_bad_node_id() {
        let mut cfg = T0::default();
        cfg.node_id = "bad".into();
        assert!(f17(&cfg).is_err());
    }
}
