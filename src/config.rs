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

    #[test]
    fn two_defaults_differ() {
        let a = T0::default();
        let b = T0::default();
        assert_ne!(a.node_id, b.node_id);
    }

    #[test]
    fn node_id_format() {
        let cfg = T0::default();
        assert!(cfg.node_id.starts_with("gf-"));
        assert_eq!(cfg.node_id.len(), 19); // "gf-" + 16 hex chars
        assert!(cfg.node_id[3..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn validate_sf_boundary_low() {
        let mut cfg = T0::default();
        cfg.spreading_factor = 6;
        f17(&cfg).unwrap();
        cfg.spreading_factor = 5;
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_sf_boundary_high() {
        let mut cfg = T0::default();
        cfg.spreading_factor = 12;
        f17(&cfg).unwrap();
        cfg.spreading_factor = 13;
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_sf_zero() {
        let mut cfg = T0::default();
        cfg.spreading_factor = 0;
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_all_valid_bandwidths() {
        let valid = [7, 10, 15, 20, 31, 41, 62, 125, 250, 500];
        for bw in valid {
            let mut cfg = T0::default();
            cfg.bandwidth_khz = bw;
            f17(&cfg).unwrap_or_else(|e| panic!("bw {} should be valid: {}", bw, e));
        }
    }

    #[test]
    fn validate_invalid_bandwidths() {
        let invalid = [0, 1, 8, 11, 16, 50, 63, 100, 124, 126, 251, 499, 501, 1000];
        for bw in invalid {
            let mut cfg = T0::default();
            cfg.bandwidth_khz = bw;
            assert!(f17(&cfg).is_err(), "bw {} should be invalid", bw);
        }
    }

    #[test]
    fn validate_freq_boundary_low() {
        let mut cfg = T0::default();
        cfg.frequency_mhz = 150;
        f17(&cfg).unwrap();
        cfg.frequency_mhz = 149;
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_freq_boundary_high() {
        let mut cfg = T0::default();
        cfg.frequency_mhz = 960;
        f17(&cfg).unwrap();
        cfg.frequency_mhz = 961;
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_freq_zero() {
        let mut cfg = T0::default();
        cfg.frequency_mhz = 0;
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_node_id_too_short() {
        let mut cfg = T0::default();
        cfg.node_id = "gf-a".into(); // 4 chars, need 5
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_node_id_min_length() {
        let mut cfg = T0::default();
        cfg.node_id = "gf-ab".into(); // exactly 5 chars
        f17(&cfg).unwrap();
    }

    #[test]
    fn validate_node_id_wrong_prefix() {
        let mut cfg = T0::default();
        cfg.node_id = "xx-abcdef".into();
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn validate_node_id_empty() {
        let mut cfg = T0::default();
        cfg.node_id = String::new();
        assert!(f17(&cfg).is_err());
    }

    #[test]
    fn config_with_peers() {
        let mut cfg = T0::default();
        cfg.peers = vec!["gf-peer1".into(), "gf-peer2".into()];
        let json = serde_json::to_string(&cfg).unwrap();
        let loaded: T0 = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.peers.len(), 2);
        assert_eq!(loaded.peers[0], "gf-peer1");
    }

    #[test]
    fn config_all_fields_round_trip() {
        let mut cfg = T0::default();
        cfg.frequency_mhz = 868;
        cfg.spreading_factor = 12;
        cfg.bandwidth_khz = 250;
        cfg.peers = vec!["gf-a".into()];
        let json = serde_json::to_string(&cfg).unwrap();
        let loaded: T0 = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.frequency_mhz, 868);
        assert_eq!(loaded.spreading_factor, 12);
        assert_eq!(loaded.bandwidth_khz, 250);
        assert_eq!(loaded.peers, vec!["gf-a"]);
    }

    #[test]
    fn config_invalid_json() {
        let result: Result<T0, _> = serde_json::from_str("not json");
        assert!(result.is_err());
    }

    #[test]
    fn config_missing_required_field() {
        let json = r#"{"node_id":"gf-test"}"#;
        let result: Result<T0, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn config_path_contains_ghost_fabric() {
        let path = f9();
        assert!(path.to_str().unwrap().contains("ghost-fabric"));
        assert!(path.to_str().unwrap().ends_with("node.json"));
    }

    #[test]
    fn config_clone() {
        let cfg = T0::default();
        let cloned = cfg.clone();
        assert_eq!(cfg.node_id, cloned.node_id);
        assert_eq!(cfg.frequency_mhz, cloned.frequency_mhz);
    }

    #[test]
    fn config_debug_format() {
        let cfg = T0::default();
        let debug = format!("{:?}", cfg);
        assert!(debug.contains("node_id"));
        assert!(debug.contains("gf-"));
    }

    #[test]
    fn validate_error_messages_contain_value() {
        let mut cfg = T0::default();
        cfg.spreading_factor = 99;
        let err = f17(&cfg).unwrap_err();
        assert!(err.contains("99"), "error should contain the bad value: {}", err);

        cfg.spreading_factor = 7;
        cfg.bandwidth_khz = 42;
        let err = f17(&cfg).unwrap_err();
        assert!(err.contains("42"), "error should contain the bad value: {}", err);

        cfg.bandwidth_khz = 125;
        cfg.frequency_mhz = 2400;
        let err = f17(&cfg).unwrap_err();
        assert!(err.contains("2400"), "error should contain the bad value: {}", err);
    }
}
