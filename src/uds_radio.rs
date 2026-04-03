//! UDS radio driver — T1 implementation over Unix domain sockets.
//! Enables multi-process mesh testing without hardware.
//!
//! T15=UdsRadio. Each node binds a UDS at /tmp/gf-{node_id}.sock.
//! Broadcast sends to all .sock files in /tmp/gf-*.sock.
//! Point-to-point sends to /tmp/gf-{dst}.sock.

use crate::radio::T1;
use std::io;
use std::os::unix::net::UnixDatagram;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// T15=UdsRadio — Unix domain socket radio for local mesh testing
pub struct T15 {
    sock: Option<UnixDatagram>,
    path: PathBuf,
    initialized: bool,
    last_rssi: Option<i16>,
}

impl T15 {
    pub fn new(node_id: &str) -> Self {
        let path = sock_path(node_id);
        Self {
            sock: None,
            path,
            initialized: false,
            last_rssi: None,
        }
    }

    /// Send raw bytes to a specific node's socket
    pub fn send_to(&self, data: &[u8], dst_id: &str) -> Result<(), String> {
        let sock = self.sock.as_ref().ok_or("not initialized")?;
        let dst_path = sock_path(dst_id);
        if !dst_path.exists() {
            return Err(format!("peer socket not found: {}", dst_path.display()));
        }
        sock.send_to(data, &dst_path)
            .map_err(|e| format!("send_to {}: {}", dst_id, e))?;
        Ok(())
    }

    /// Broadcast to all gf-*.sock peers in /tmp
    pub fn broadcast(&self, data: &[u8]) -> Result<usize, String> {
        let sock = self.sock.as_ref().ok_or("not initialized")?;
        let mut sent = 0;
        for entry in std::fs::read_dir("/tmp").map_err(|e| e.to_string())? {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("sock")
                && path != self.path
                && path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("gf-"))
                && sock.send_to(data, &path).is_ok()
            {
                sent += 1;
            }
        }
        Ok(sent)
    }
}

impl Drop for T15 {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

impl T1 for T15 {
    fn init(&mut self, _freq_mhz: u32, _sf: u8, _bw_khz: u32) -> Result<(), String> {
        // Remove stale socket
        if self.path.exists() {
            std::fs::remove_file(&self.path).map_err(|e| e.to_string())?;
        }

        let sock = UnixDatagram::bind(&self.path).map_err(|e| {
            format!("bind {}: {}", self.path.display(), e)
        })?;
        sock.set_nonblocking(true).map_err(|e| e.to_string())?;
        self.sock = Some(sock);
        self.initialized = true;
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<(), String> {
        if !self.initialized {
            return Err("not initialized".into());
        }
        if data.len() > 255 {
            return Err(format!("packet too large: {} bytes (max 255)", data.len()));
        }
        // UDS "send" broadcasts to all peers
        self.broadcast(data)?;
        Ok(())
    }

    fn recv(&mut self, timeout_ms: u64) -> Result<Option<Vec<u8>>, String> {
        let sock = self.sock.as_ref().ok_or("not initialized")?;

        if timeout_ms > 0 {
            sock.set_read_timeout(Some(Duration::from_millis(timeout_ms)))
                .map_err(|e| e.to_string())?;
        } else {
            sock.set_nonblocking(true).map_err(|e| e.to_string())?;
        }

        let mut buf = [0u8; 256];
        match sock.recv(&mut buf) {
            Ok(n) => {
                // Simulate RSSI: UDS is local, so always strong signal
                self.last_rssi = Some(-30);
                Ok(Some(buf[..n].to_vec()))
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(format!("recv: {}", e)),
        }
    }

    fn status(&self) -> &str {
        if self.initialized {
            "connected (UDS)"
        } else {
            "not initialized"
        }
    }

    fn last_rssi(&self) -> Option<i16> {
        self.last_rssi
    }
}

fn sock_path(node_id: &str) -> PathBuf {
    Path::new("/tmp").join(format!("{}.sock", node_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uds_init_and_cleanup() {
        let mut radio = T15::new("gf-test-init");
        assert_eq!(radio.status(), "not initialized");
        radio.init(915, 7, 125).unwrap();
        assert_eq!(radio.status(), "connected (UDS)");
        assert!(radio.path.exists());

        // Cleanup on drop
        let path = radio.path.clone();
        drop(radio);
        assert!(!path.exists());
    }

    #[test]
    fn uds_send_recv_between_nodes() {
        let id = std::process::id();
        let name_a = format!("gf-test-sr-{}-a", id);
        let name_b = format!("gf-test-sr-{}-b", id);
        let mut node_a = T15::new(&name_a);
        let mut node_b = T15::new(&name_b);
        node_a.init(915, 7, 125).unwrap();
        node_b.init(915, 7, 125).unwrap();

        // A sends directly to B (broadcast hits other test sockets)
        node_a.send_to(b"hello from A", &name_b).unwrap();

        // B should receive
        let data = node_b.recv(100).unwrap();
        assert_eq!(data, Some(b"hello from A".to_vec()));

        // A should not have received anything
        let self_data = node_a.recv(50).unwrap();
        assert_eq!(self_data, None);
    }

    #[test]
    fn uds_send_to_specific_peer() {
        let id = std::process::id();
        let name_a = format!("gf-test-p2p-{}-a", id);
        let name_b = format!("gf-test-p2p-{}-b", id);
        let name_c = format!("gf-test-p2p-{}-c", id);
        let mut node_a = T15::new(&name_a);
        let mut node_b = T15::new(&name_b);
        let mut node_c = T15::new(&name_c);
        node_a.init(915, 7, 125).unwrap();
        node_b.init(915, 7, 125).unwrap();
        node_c.init(915, 7, 125).unwrap();

        // A sends directly to B
        node_a.send_to(b"just for B", &name_b).unwrap();

        // B gets it
        let data_b = node_b.recv(100).unwrap();
        assert_eq!(data_b, Some(b"just for B".to_vec()));

        // C should not get it
        let data_c = node_c.recv(50).unwrap();
        assert_eq!(data_c, None);
    }

    #[test]
    fn uds_recv_empty() {
        let mut radio = T15::new("gf-test-empty");
        radio.init(915, 7, 125).unwrap();
        assert_eq!(radio.recv(10).unwrap(), None);
    }

    #[test]
    fn uds_send_before_init() {
        let mut radio = T15::new("gf-test-noinit");
        assert!(radio.send(b"fail").is_err());
    }

    #[test]
    fn uds_cbor_frame_round_trip() {
        use crate::packet;

        let id = std::process::id();
        let name_a = format!("gf-test-cbor-{}-a", id);
        let name_b = format!("gf-test-cbor-{}-b", id);
        let mut node_a = T15::new(&name_a);
        let mut node_b = T15::new(&name_b);
        node_a.init(915, 7, 125).unwrap();
        node_b.init(915, 7, 125).unwrap();

        // A sends directly to B
        let beacon = packet::T12::f20(&name_a, 90, 1, 1);
        let bytes = packet::f18(&beacon).unwrap();
        node_a.send_to(&bytes, &name_b).unwrap();

        // B receives and decodes
        let data = node_b.recv(100).unwrap().unwrap();
        let frame = packet::f19(&data).unwrap();
        assert_eq!(frame.src, name_a);
        assert_eq!(frame.kind, packet::T13::Beacon);
    }
}
