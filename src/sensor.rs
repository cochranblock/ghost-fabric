//! Sensor subsystem — local sensor I/O drivers
//!
//! Future: GPIO/I2C/SPI sensor reads, data normalization,
//! local fusion before inference.

/// f8=sensor_status — report sensor subsystem state
pub fn f8() -> &'static str {
    "no sensors detected"
}
