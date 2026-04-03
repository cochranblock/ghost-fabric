//! Radio subsystem — LoRa 915MHz interface
//!
//! T1=RadioDriver trait for hardware abstraction.
//! Future: sx127x or sx126x driver integration,
//! TX/RX packet framing, duty cycle management.

/// T1=RadioDriver — hardware abstraction for LoRa radios
pub trait T1 {
    /// Initialize radio hardware at given frequency/SF/BW
    fn init(&mut self, freq_mhz: u32, sf: u8, bw_khz: u32) -> Result<(), String>;
    /// Send raw packet (max 255 bytes for LoRa)
    fn send(&mut self, data: &[u8]) -> Result<(), String>;
    /// Receive packet with timeout in ms. Returns None on timeout.
    fn recv(&mut self, timeout_ms: u64) -> Result<Option<Vec<u8>>, String>;
    /// Current radio status string
    fn status(&self) -> &str;
    /// RSSI of last received packet (dBm)
    fn last_rssi(&self) -> Option<i16>;
}

/// f5=radio_status — report radio subsystem state (legacy compat)
pub fn f5() -> &'static str {
    "not connected (driver not implemented)"
}

/// T8=MockRadio — in-memory radio for testing. Packets go into a buffer.
pub struct T8 {
    initialized: bool,
    freq_mhz: u32,
    sf: u8,
    bw_khz: u32,
    tx_buf: Vec<Vec<u8>>,
    rx_buf: Vec<Vec<u8>>,
    rssi: Option<i16>,
}

impl Default for T8 {
    fn default() -> Self {
        Self::new()
    }
}

impl T8 {
    pub fn new() -> Self {
        Self {
            initialized: false,
            freq_mhz: 0,
            sf: 0,
            bw_khz: 0,
            tx_buf: Vec::new(),
            rx_buf: Vec::new(),
            rssi: None,
        }
    }

    /// Inject a packet into the receive buffer (simulates incoming radio)
    pub fn inject_rx(&mut self, data: Vec<u8>, rssi: i16) {
        self.rx_buf.push(data);
        self.rssi = Some(rssi);
    }

    /// Read all transmitted packets (for test assertions)
    pub fn drain_tx(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.tx_buf)
    }
}

impl T1 for T8 {
    fn init(&mut self, freq_mhz: u32, sf: u8, bw_khz: u32) -> Result<(), String> {
        if !(6..=12).contains(&sf) {
            return Err(format!("invalid spreading factor: {} (must be 6-12)", sf));
        }
        self.freq_mhz = freq_mhz;
        self.sf = sf;
        self.bw_khz = bw_khz;
        self.initialized = true;
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<(), String> {
        if !self.initialized {
            return Err("radio not initialized".into());
        }
        if data.len() > 255 {
            return Err(format!("packet too large: {} bytes (max 255)", data.len()));
        }
        self.tx_buf.push(data.to_vec());
        Ok(())
    }

    fn recv(&mut self, _timeout_ms: u64) -> Result<Option<Vec<u8>>, String> {
        if !self.initialized {
            return Err("radio not initialized".into());
        }
        if self.rx_buf.is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.rx_buf.remove(0)))
        }
    }

    fn status(&self) -> &str {
        if self.initialized {
            "connected (mock)"
        } else {
            "not initialized"
        }
    }

    fn last_rssi(&self) -> Option<i16> {
        self.rssi
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_radio_init() {
        let mut radio = T8::new();
        assert_eq!(radio.status(), "not initialized");
        radio.init(915, 7, 125).unwrap();
        assert_eq!(radio.status(), "connected (mock)");
    }

    #[test]
    fn mock_radio_invalid_sf() {
        let mut radio = T8::new();
        assert!(radio.init(915, 5, 125).is_err());
        assert!(radio.init(915, 13, 125).is_err());
    }

    #[test]
    fn mock_radio_send_recv() {
        let mut radio = T8::new();
        radio.init(915, 7, 125).unwrap();

        radio.send(b"hello mesh").unwrap();
        let sent = radio.drain_tx();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0], b"hello mesh");

        radio.inject_rx(b"reply".to_vec(), -85);
        let pkt = radio.recv(1000).unwrap();
        assert_eq!(pkt, Some(b"reply".to_vec()));
        assert_eq!(radio.last_rssi(), Some(-85));
    }

    #[test]
    fn mock_radio_recv_empty() {
        let mut radio = T8::new();
        radio.init(915, 7, 125).unwrap();
        assert_eq!(radio.recv(100).unwrap(), None);
    }

    #[test]
    fn mock_radio_send_before_init() {
        let mut radio = T8::new();
        assert!(radio.send(b"fail").is_err());
    }

    #[test]
    fn mock_radio_packet_too_large() {
        let mut radio = T8::new();
        radio.init(915, 7, 125).unwrap();
        let big = vec![0u8; 256];
        assert!(radio.send(&big).is_err());
    }

    #[test]
    fn legacy_status() {
        assert_eq!(f5(), "not connected (driver not implemented)");
    }

    #[test]
    fn init_sf_boundary_6() {
        let mut r = T8::new();
        r.init(915, 6, 125).unwrap();
        assert_eq!(r.status(), "connected (mock)");
    }

    #[test]
    fn init_sf_boundary_12() {
        let mut r = T8::new();
        r.init(915, 12, 125).unwrap();
    }

    #[test]
    fn init_sf_zero() {
        let mut r = T8::new();
        assert!(r.init(915, 0, 125).is_err());
    }

    #[test]
    fn send_exact_255_bytes() {
        let mut r = T8::new();
        r.init(915, 7, 125).unwrap();
        let data = vec![0xABu8; 255];
        r.send(&data).unwrap();
        let sent = r.drain_tx();
        assert_eq!(sent[0].len(), 255);
    }

    #[test]
    fn send_empty() {
        let mut r = T8::new();
        r.init(915, 7, 125).unwrap();
        r.send(&[]).unwrap();
        let sent = r.drain_tx();
        assert_eq!(sent[0].len(), 0);
    }

    #[test]
    fn multiple_tx_accumulate() {
        let mut r = T8::new();
        r.init(915, 7, 125).unwrap();
        r.send(b"one").unwrap();
        r.send(b"two").unwrap();
        r.send(b"three").unwrap();
        let sent = r.drain_tx();
        assert_eq!(sent.len(), 3);
        assert_eq!(sent[0], b"one");
        assert_eq!(sent[1], b"two");
        assert_eq!(sent[2], b"three");
    }

    #[test]
    fn drain_tx_clears_buffer() {
        let mut r = T8::new();
        r.init(915, 7, 125).unwrap();
        r.send(b"data").unwrap();
        let first = r.drain_tx();
        assert_eq!(first.len(), 1);
        let second = r.drain_tx();
        assert!(second.is_empty());
    }

    #[test]
    fn recv_drains_in_order() {
        let mut r = T8::new();
        r.init(915, 7, 125).unwrap();
        r.inject_rx(b"first".to_vec(), -80);
        r.inject_rx(b"second".to_vec(), -90);
        r.inject_rx(b"third".to_vec(), -70);

        assert_eq!(r.recv(0).unwrap(), Some(b"first".to_vec()));
        assert_eq!(r.recv(0).unwrap(), Some(b"second".to_vec()));
        assert_eq!(r.recv(0).unwrap(), Some(b"third".to_vec()));
        assert_eq!(r.recv(0).unwrap(), None);
    }

    #[test]
    fn rssi_none_before_inject() {
        let r = T8::new();
        assert_eq!(r.last_rssi(), None);
    }

    #[test]
    fn rssi_updates_on_inject() {
        let mut r = T8::new();
        r.inject_rx(b"a".to_vec(), -50);
        assert_eq!(r.last_rssi(), Some(-50));
        r.inject_rx(b"b".to_vec(), -99);
        assert_eq!(r.last_rssi(), Some(-99));
    }

    #[test]
    fn recv_before_init_error() {
        let mut r = T8::new();
        assert!(r.recv(100).is_err());
    }

    #[test]
    fn send_after_recv() {
        let mut r = T8::new();
        r.init(915, 7, 125).unwrap();
        r.inject_rx(b"in".to_vec(), -70);
        r.recv(0).unwrap();
        r.send(b"out").unwrap();
        assert_eq!(r.drain_tx().len(), 1);
    }

    #[test]
    fn default_impl() {
        let r = T8::default();
        assert_eq!(r.status(), "not initialized");
        assert_eq!(r.last_rssi(), None);
    }

    #[test]
    fn init_stores_params() {
        let mut r = T8::new();
        r.init(868, 10, 250).unwrap();
        assert!(r.initialized);
        assert_eq!(r.freq_mhz, 868);
        assert_eq!(r.sf, 10);
        assert_eq!(r.bw_khz, 250);
    }

    #[test]
    fn packet_size_error_message() {
        let mut r = T8::new();
        r.init(915, 7, 125).unwrap();
        let big = vec![0u8; 300];
        let err = r.send(&big).unwrap_err();
        assert!(err.contains("300"), "error should contain actual size");
        assert!(err.contains("255"), "error should contain max size");
    }
}
