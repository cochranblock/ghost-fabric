//! Mesh packet types — CBOR-encoded frames for LoRa (<255 bytes).
//!
//! T12=Frame (wire format), T13=FrameKind (packet type tag),
//! T14=Payload (typed payload variants).
//! f18=encode, f19=decode, f20=beacon, f21=data_frame, f26=derive_key.
//!
//! Wire format: [CBOR bytes][16-byte truncated HMAC-SHA256]
//! f18 appends MAC; f19 verifies before decoding.

use hmac::{Hmac, Mac};
use hkdf::Hkdf;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Total LoRa payload budget (255 - 4 radio overhead).
pub const MAX_FRAME_BYTES: usize = 251;
/// HMAC-SHA256 truncated to 128 bits — appended to every encoded frame.
pub const MAC_BYTES: usize = 16;
/// Max CBOR payload after reserving space for the MAC.
pub const MAX_CBOR_BYTES: usize = MAX_FRAME_BYTES - MAC_BYTES;

/// T13=FrameKind — packet type discriminator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum T13 {
    /// Node beacon: "I'm here, this is my state"
    Beacon,
    /// Sensor/inference data payload
    Data,
    /// Acknowledgment of a received frame
    Ack,
    /// Peer discovery request
    Ping,
    /// Peer discovery response
    Pong,
    /// Neighbor table sync — share known peers on reconnection
    Sync,
}

/// T14=Payload — typed payload content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum T14 {
    /// Beacon: battery %, hop count
    Beacon { battery_pct: u8, hop_count: u8 },
    /// Data: sensor name, value, unit
    Data {
        name: String,
        value: f64,
        unit: String,
    },
    /// Ack: sequence number being acknowledged
    Ack { ack_seq: u16 },
    /// Ping: empty (just the frame header matters)
    Ping,
    /// Pong: battery %, peer count
    Pong { battery_pct: u8, peer_count: u8 },
    /// Sync: compressed neighbor table entries
    Sync { peers: Vec<T16> },
}

/// T16=SyncEntry — compressed peer info for state sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct T16 {
    pub id: String,
    pub rssi: i16,
    pub battery: u8,
    pub hops: u8,
}

/// T12=Frame — wire-format mesh packet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct T12 {
    /// Source node ID (e.g. "gf-a1b2c3d4e5f6g7h8")
    pub src: String,
    /// Destination node ID or "*" for broadcast
    pub dst: String,
    /// Packet type
    pub kind: T13,
    /// Monotonic sequence number (per-sender)
    pub seq: u16,
    /// Time-to-live: decremented on each relay hop
    pub ttl: u8,
    /// Typed payload
    pub payload: T14,
}

impl T12 {
    /// f20=beacon — create a beacon frame
    pub fn f20(src: &str, battery_pct: u8, hop_count: u8, seq: u16) -> Self {
        Self {
            src: src.to_string(),
            dst: "*".to_string(),
            kind: T13::Beacon,
            seq,
            ttl: 3,
            payload: T14::Beacon {
                battery_pct,
                hop_count,
            },
        }
    }

    /// f21=data_frame — create a data frame for a specific destination
    pub fn f21(src: &str, dst: &str, name: &str, value: f64, unit: &str, seq: u16) -> Self {
        Self {
            src: src.to_string(),
            dst: dst.to_string(),
            kind: T13::Data,
            seq,
            ttl: 5,
            payload: T14::Data {
                name: name.to_string(),
                value,
                unit: unit.to_string(),
            },
        }
    }

    /// Create a ping frame
    pub fn ping(src: &str, seq: u16) -> Self {
        Self {
            src: src.to_string(),
            dst: "*".to_string(),
            kind: T13::Ping,
            seq,
            ttl: 1,
            payload: T14::Ping,
        }
    }

    /// Create a pong response
    pub fn pong(src: &str, dst: &str, battery_pct: u8, peer_count: u8, seq: u16) -> Self {
        Self {
            src: src.to_string(),
            dst: dst.to_string(),
            kind: T13::Pong,
            seq,
            ttl: 1,
            payload: T14::Pong {
                battery_pct,
                peer_count,
            },
        }
    }

    /// Create an ack frame
    pub fn ack(src: &str, dst: &str, ack_seq: u16, seq: u16) -> Self {
        Self {
            src: src.to_string(),
            dst: dst.to_string(),
            kind: T13::Ack,
            seq,
            ttl: 1,
            payload: T14::Ack { ack_seq },
        }
    }

    /// f23=sync_frame — create a state sync frame with neighbor table
    pub fn f23(src: &str, peers: Vec<T16>, seq: u16) -> Self {
        Self {
            src: src.to_string(),
            dst: "*".to_string(),
            kind: T13::Sync,
            seq,
            ttl: 2,
            payload: T14::Sync { peers },
        }
    }

    /// Is this a broadcast frame?
    pub fn is_broadcast(&self) -> bool {
        self.dst == "*"
    }

    /// Should this frame be relayed? (TTL > 0 after decrement)
    pub fn should_relay(&self) -> bool {
        self.ttl > 1
    }

    /// Decrement TTL for relay. Returns false if TTL expired.
    pub fn relay_hop(&mut self) -> bool {
        if self.ttl == 0 {
            return false;
        }
        self.ttl -= 1;
        self.ttl > 0
    }
}

/// f26=derive_key — derive 32-byte mesh key from passphrase via HKDF-SHA256.
/// All nodes sharing the same secret will accept each other's frames.
pub fn f26(secret: &[u8]) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(None, secret);
    let mut key = [0u8; 32];
    hk.expand(b"ghost-fabric mesh v1", &mut key).expect("hkdf expand");
    key
}

/// f18=encode — serialize frame to CBOR then append 16-byte HMAC-SHA256.
/// Total wire size: CBOR bytes + MAC_BYTES (≤ MAX_FRAME_BYTES).
pub fn f18(frame: &T12, key: &[u8; 32]) -> Result<Vec<u8>, String> {
    let mut cbor = Vec::new();
    ciborium::into_writer(frame, &mut cbor).map_err(|e| format!("cbor encode: {}", e))?;
    if cbor.len() > MAX_CBOR_BYTES {
        return Err(format!(
            "frame too large: {} bytes (max {})",
            cbor.len(),
            MAX_CBOR_BYTES
        ));
    }
    let mut mac = HmacSha256::new_from_slice(key).expect("hmac key");
    mac.update(&cbor);
    let tag = mac.finalize().into_bytes();
    cbor.extend_from_slice(&tag[..MAC_BYTES]);
    Ok(cbor)
}

/// f19=decode — verify 16-byte HMAC then deserialize CBOR frame.
/// Returns Err on MAC mismatch (tampered or wrong key) or bad CBOR.
pub fn f19(data: &[u8], key: &[u8; 32]) -> Result<T12, String> {
    if data.len() < MAC_BYTES {
        return Err(format!(
            "frame too short: {} bytes (min {})",
            data.len(),
            MAC_BYTES
        ));
    }
    let (cbor, tag) = data.split_at(data.len() - MAC_BYTES);
    let mut mac = HmacSha256::new_from_slice(key).expect("hmac key");
    mac.update(cbor);
    // Tag is truncated to the first MAC_BYTES of the full HMAC-SHA256 output.
    mac.verify_truncated_left(tag)
        .map_err(|_| "MAC verification failed".to_string())?;
    ciborium::from_reader(cbor).map_err(|e| format!("cbor decode: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_KEY: [u8; 32] = [0x42u8; 32];
    const OTHER_KEY: [u8; 32] = [0xFFu8; 32];

    #[test]
    fn beacon_round_trip() {
        let frame = T12::f20("gf-0001", 85, 1, 42);
        assert_eq!(frame.kind, T13::Beacon);
        assert!(frame.is_broadcast());
        assert_eq!(frame.ttl, 3);

        let bytes = f18(&frame, &TEST_KEY).unwrap();
        assert!(bytes.len() < MAX_FRAME_BYTES);
        // Wire bytes = CBOR + 16-byte MAC
        assert!(bytes.len() >= MAC_BYTES);

        let decoded = f19(&bytes, &TEST_KEY).unwrap();
        assert_eq!(decoded.src, "gf-0001");
        assert_eq!(decoded.seq, 42);
        if let T14::Beacon { battery_pct, hop_count } = decoded.payload {
            assert_eq!(battery_pct, 85);
            assert_eq!(hop_count, 1);
        } else {
            panic!("wrong payload type");
        }
    }

    #[test]
    fn data_frame_round_trip() {
        let frame = T12::f21("gf-0001", "gf-0002", "temp", 22.5, "C", 7);
        assert!(!frame.is_broadcast());
        assert_eq!(frame.kind, T13::Data);

        let bytes = f18(&frame, &TEST_KEY).unwrap();
        let decoded = f19(&bytes, &TEST_KEY).unwrap();
        assert_eq!(decoded.dst, "gf-0002");
        if let T14::Data { name, value, unit } = decoded.payload {
            assert_eq!(name, "temp");
            assert!((value - 22.5).abs() < f64::EPSILON);
            assert_eq!(unit, "C");
        } else {
            panic!("wrong payload type");
        }
    }

    #[test]
    fn ping_pong_round_trip() {
        let ping = T12::ping("gf-0001", 1);
        assert!(ping.is_broadcast());
        let bytes = f18(&ping, &TEST_KEY).unwrap();
        let decoded = f19(&bytes, &TEST_KEY).unwrap();
        assert_eq!(decoded.kind, T13::Ping);

        let pong = T12::pong("gf-0002", "gf-0001", 90, 3, 2);
        let bytes = f18(&pong, &TEST_KEY).unwrap();
        let decoded = f19(&bytes, &TEST_KEY).unwrap();
        assert_eq!(decoded.kind, T13::Pong);
        if let T14::Pong { battery_pct, peer_count } = decoded.payload {
            assert_eq!(battery_pct, 90);
            assert_eq!(peer_count, 3);
        } else {
            panic!("wrong payload type");
        }
    }

    #[test]
    fn ack_round_trip() {
        let ack = T12::ack("gf-0002", "gf-0001", 42, 3);
        assert_eq!(ack.kind, T13::Ack);
        let bytes = f18(&ack, &TEST_KEY).unwrap();
        let decoded = f19(&bytes, &TEST_KEY).unwrap();
        if let T14::Ack { ack_seq } = decoded.payload {
            assert_eq!(ack_seq, 42);
        } else {
            panic!("wrong payload type");
        }
    }

    #[test]
    fn frame_size_under_limit() {
        let frame = T12::f20("gf-a1b2c3d4e5f6g7h8", 100, 1, 65535);
        let bytes = f18(&frame, &TEST_KEY).unwrap();
        // CBOR ~50 bytes + 16-byte MAC — well under 251
        assert!(bytes.len() < 120, "beacon too large: {} bytes", bytes.len());
        assert!(bytes.len() <= MAX_FRAME_BYTES);
    }

    #[test]
    fn data_frame_size() {
        let frame = T12::f21(
            "gf-a1b2c3d4e5f6g7h8",
            "gf-b2c3d4e5f6g7h8a1",
            "temperature",
            -40.123456,
            "celsius",
            999,
        );
        let bytes = f18(&frame, &TEST_KEY).unwrap();
        assert!(bytes.len() < 176, "data frame too large: {} bytes", bytes.len());
        assert!(bytes.len() <= MAX_FRAME_BYTES);
    }

    #[test]
    fn ttl_relay() {
        let mut frame = T12::f20("gf-0001", 85, 1, 1);
        assert_eq!(frame.ttl, 3);
        assert!(frame.should_relay());
        assert!(frame.relay_hop()); // 3 -> 2
        assert!(frame.relay_hop()); // 2 -> 1
        assert!(!frame.relay_hop()); // 1 -> 0, returns false
        assert!(!frame.should_relay());
    }

    // --- authentication tests ---

    #[test]
    fn wrong_key_rejected() {
        let frame = T12::f20("gf-0001", 80, 1, 1);
        let bytes = f18(&frame, &TEST_KEY).unwrap();
        let err = f19(&bytes, &OTHER_KEY).unwrap_err();
        assert_eq!(err, "MAC verification failed");
    }

    #[test]
    fn tampered_payload_rejected() {
        let frame = T12::f20("gf-0001", 80, 1, 1);
        let mut bytes = f18(&frame, &TEST_KEY).unwrap();
        // Flip a byte in the CBOR region (not the MAC)
        let cbor_mid = (bytes.len() - MAC_BYTES) / 2;
        bytes[cbor_mid] ^= 0xFF;
        assert!(f19(&bytes, &TEST_KEY).is_err());
    }

    #[test]
    fn tampered_mac_rejected() {
        let frame = T12::f20("gf-0001", 80, 1, 1);
        let mut bytes = f18(&frame, &TEST_KEY).unwrap();
        // Flip a byte in the MAC region
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;
        let err = f19(&bytes, &TEST_KEY).unwrap_err();
        assert_eq!(err, "MAC verification failed");
    }

    #[test]
    fn truncated_frame_rejected() {
        let frame = T12::f20("gf-0001", 80, 1, 1);
        let bytes = f18(&frame, &TEST_KEY).unwrap();
        // Strip all but 10 bytes — less than MAC_BYTES
        assert!(f19(&bytes[..10], &TEST_KEY).is_err());
    }

    #[test]
    fn decode_garbage() {
        // Garbage shorter than MAC_BYTES
        assert!(f19(&[0xFF, 0x00, 0x42], &TEST_KEY).is_err());
    }

    #[test]
    fn decode_empty() {
        assert!(f19(&[], &TEST_KEY).is_err());
    }

    #[test]
    fn mac_is_appended_not_prepended() {
        let frame = T12::f20("gf-0001", 80, 1, 1);
        let bytes = f18(&frame, &TEST_KEY).unwrap();
        // CBOR starts with a CBOR map marker (0xA? or similar), not random bytes
        // Verify that stripping the last MAC_BYTES gives valid-looking CBOR
        let cbor_part = &bytes[..bytes.len() - MAC_BYTES];
        assert!(!cbor_part.is_empty());
        // The full bytes should NOT decode without key (wrong slice)
        assert!(f19(cbor_part, &TEST_KEY).is_err()); // no MAC appended — too short or bad MAC
    }

    #[test]
    fn different_secrets_produce_different_keys() {
        let k1 = f26(b"secret-a");
        let k2 = f26(b"secret-b");
        assert_ne!(k1, k2);
    }

    #[test]
    fn same_secret_produces_same_key() {
        let k1 = f26(b"my-mesh-secret");
        let k2 = f26(b"my-mesh-secret");
        assert_eq!(k1, k2);
    }

    #[test]
    fn empty_secret_produces_valid_key() {
        let k = f26(b"");
        assert_ne!(k, [0u8; 32]); // HKDF with empty IKM is defined, not zero
        // Frames encoded with this key should round-trip
        let frame = T12::f20("gf-0001", 80, 1, 1);
        let bytes = f18(&frame, &k).unwrap();
        f19(&bytes, &k).unwrap();
    }

    #[test]
    fn sync_frame_round_trip() {
        let entries = vec![
            T16 { id: "gf-a".into(), rssi: -60, battery: 90, hops: 1 },
            T16 { id: "gf-b".into(), rssi: -80, battery: 70, hops: 2 },
        ];
        let frame = T12::f23("gf-origin", entries, 5);
        assert_eq!(frame.kind, T13::Sync);
        assert!(frame.is_broadcast());
        assert_eq!(frame.ttl, 2);
        assert_eq!(frame.seq, 5);

        let bytes = f18(&frame, &TEST_KEY).unwrap();
        assert!(bytes.len() < MAX_FRAME_BYTES);

        let decoded = f19(&bytes, &TEST_KEY).unwrap();
        assert_eq!(decoded.src, "gf-origin");
        if let T14::Sync { peers } = decoded.payload {
            assert_eq!(peers.len(), 2);
            assert_eq!(peers[0].id, "gf-a");
            assert_eq!(peers[1].hops, 2);
        } else {
            panic!("wrong payload type");
        }
    }

    #[test]
    fn sync_frame_empty_peers() {
        let frame = T12::f23("gf-origin", vec![], 0);
        let bytes = f18(&frame, &TEST_KEY).unwrap();
        let decoded = f19(&bytes, &TEST_KEY).unwrap();
        if let T14::Sync { peers } = decoded.payload {
            assert!(peers.is_empty());
        } else {
            panic!("wrong payload type");
        }
    }
}
