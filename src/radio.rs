//! Radio subsystem — LoRa 915MHz interface
//!
//! Future: sx127x or sx126x driver integration,
//! TX/RX packet framing, duty cycle management.

/// f5=radio_status — report radio subsystem state
pub fn f5() -> &'static str {
    "not connected (driver not implemented)"
}
