//! Sensor subsystem — local sensor I/O drivers
//!
//! T4=SensorDriver trait, T5=SensorReading value type.
//! Future: GPIO/I2C/SPI sensor reads, data normalization,
//! local fusion before inference.

/// T5=SensorReading — a single sensor measurement
#[derive(Debug, Clone)]
pub struct T5 {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: u64,
}

/// T4=SensorDriver — trait for sensor hardware abstraction
pub trait T4 {
    /// Sensor name (e.g. "bme280", "gps", "imu")
    fn name(&self) -> &str;
    /// Take a reading. Returns None if sensor unavailable.
    fn read(&mut self) -> Option<T5>;
    /// Current sensor status
    fn status(&self) -> &str;
}

/// T10=MockSensor — test sensor that returns canned values
pub struct T10 {
    sensor_name: String,
    readings: Vec<f64>,
    unit: String,
    idx: usize,
}

impl T10 {
    pub fn new(name: &str, unit: &str, readings: Vec<f64>) -> Self {
        Self {
            sensor_name: name.to_string(),
            readings,
            unit: unit.to_string(),
            idx: 0,
        }
    }
}

impl T4 for T10 {
    fn name(&self) -> &str {
        &self.sensor_name
    }

    fn read(&mut self) -> Option<T5> {
        if self.readings.is_empty() {
            return None;
        }
        let val = self.readings[self.idx % self.readings.len()];
        self.idx += 1;
        Some(T5 {
            name: self.sensor_name.clone(),
            value: val,
            unit: self.unit.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    fn status(&self) -> &str {
        if self.readings.is_empty() {
            "no data"
        } else {
            "active (mock)"
        }
    }
}

/// f8=sensor_status — report sensor subsystem state (legacy compat)
pub fn f8() -> &'static str {
    "no sensors detected"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_sensor_reads() {
        let mut s = T10::new("temp", "C", vec![22.5, 23.0, 22.8]);
        assert_eq!(s.name(), "temp");
        assert_eq!(s.status(), "active (mock)");

        let r = s.read().unwrap();
        assert_eq!(r.name, "temp");
        assert!((r.value - 22.5).abs() < f64::EPSILON);
        assert_eq!(r.unit, "C");
        assert!(r.timestamp > 0);

        let r2 = s.read().unwrap();
        assert!((r2.value - 23.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mock_sensor_wraps() {
        let mut s = T10::new("hum", "%", vec![60.0]);
        s.read().unwrap();
        let r = s.read().unwrap();
        assert!((r.value - 60.0).abs() < f64::EPSILON);
    }

    #[test]
    fn mock_sensor_empty() {
        let mut s = T10::new("broken", "?", vec![]);
        assert_eq!(s.status(), "no data");
        assert!(s.read().is_none());
    }

    #[test]
    fn legacy_status() {
        assert_eq!(f8(), "no sensors detected");
    }
}
