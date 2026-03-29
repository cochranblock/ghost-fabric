//! Mesh subsystem — cognitive mesh routing and peer discovery
//!
//! Future: peer table, route scoring (RSSI + battery + priority),
//! compressed state sync, graceful degradation.

/// f6=mesh_status — report mesh subsystem state
pub fn f6() -> &'static str {
    "offline (no peers)"
}
