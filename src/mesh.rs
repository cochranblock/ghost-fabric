//! Mesh subsystem — cognitive mesh routing and peer discovery
//!
//! T2=MeshNetwork trait, T3=Peer struct, T9=PeerTable in-memory store.
//! Route scoring: RSSI + battery + last_seen recency.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// T3=Peer — a known neighbor node
#[derive(Debug, Clone)]
pub struct T3 {
    pub node_id: String,
    pub rssi: i16,
    pub last_seen: u64,
    pub battery_pct: u8,
    pub hop_count: u8,
}

impl T3 {
    pub fn new(node_id: &str, rssi: i16, battery_pct: u8) -> Self {
        Self {
            node_id: node_id.to_string(),
            rssi,
            last_seen: now_secs(),
            battery_pct,
            hop_count: 1,
        }
    }

    /// Route score: higher is better. Prefers strong signal, high battery, recent contact.
    pub fn route_score(&self) -> i32 {
        let rssi_score = (self.rssi + 120) as i32; // -120dBm=0, -40dBm=80
        let battery_score = self.battery_pct as i32 / 5; // 0-20
        let age = now_secs().saturating_sub(self.last_seen);
        let freshness = 20i32.saturating_sub(age as i32 / 30); // decays over 10 min
        let hop_penalty = self.hop_count as i32 * 10;
        rssi_score + battery_score + freshness - hop_penalty
    }
}

/// T2=MeshNetwork — trait for mesh peer management and routing
pub trait T2 {
    fn add_peer(&mut self, peer: T3);
    fn remove_peer(&mut self, node_id: &str) -> bool;
    fn get_peer(&self, node_id: &str) -> Option<&T3>;
    fn peers(&self) -> Vec<&T3>;
    fn best_route(&self, dest: &str) -> Option<&T3>;
    fn peer_count(&self) -> usize;
    fn status(&self) -> String;
}

/// T9=PeerTable — in-memory peer table
pub struct T9 {
    peers: HashMap<String, T3>,
}

impl Default for T9 {
    fn default() -> Self {
        Self::new()
    }
}

impl T9 {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    /// Remove peers not seen for `max_age_secs`
    pub fn evict_stale(&mut self, max_age_secs: u64) -> usize {
        let cutoff = now_secs().saturating_sub(max_age_secs);
        let before = self.peers.len();
        self.peers.retain(|_, p| p.last_seen >= cutoff);
        before - self.peers.len()
    }
}

impl T2 for T9 {
    fn add_peer(&mut self, peer: T3) {
        self.peers.insert(peer.node_id.clone(), peer);
    }

    fn remove_peer(&mut self, node_id: &str) -> bool {
        self.peers.remove(node_id).is_some()
    }

    fn get_peer(&self, node_id: &str) -> Option<&T3> {
        self.peers.get(node_id)
    }

    fn peers(&self) -> Vec<&T3> {
        self.peers.values().collect()
    }

    fn best_route(&self, dest: &str) -> Option<&T3> {
        // Direct peer match first
        if let Some(p) = self.peers.get(dest) {
            return Some(p);
        }
        // Otherwise pick highest-scoring peer as relay
        self.peers.values().max_by_key(|p| p.route_score())
    }

    fn peer_count(&self) -> usize {
        self.peers.len()
    }

    fn status(&self) -> String {
        let n = self.peers.len();
        if n == 0 {
            "offline (no peers)".to_string()
        } else {
            format!("online ({} peer{})", n, if n == 1 { "" } else { "s" })
        }
    }
}

impl T9 {
    /// f24=export_sync — export peer table as sync entries for broadcast
    pub fn f24(&self) -> Vec<crate::packet::T16> {
        self.peers
            .values()
            .map(|p| crate::packet::T16 {
                id: p.node_id.clone(),
                rssi: p.rssi,
                battery: p.battery_pct,
                hops: p.hop_count,
            })
            .collect()
    }

    /// f25=import_sync — merge sync entries from a remote peer, incrementing hop count
    pub fn f25(&mut self, entries: &[crate::packet::T16], _from_node: &str) -> usize {
        let mut added = 0;
        for entry in entries {
            // Don't add ourselves or the sender (we already know them)
            if self.peers.contains_key(&entry.id) {
                continue;
            }
            let mut peer = T3::new(&entry.id, entry.rssi, entry.battery);
            peer.hop_count = entry.hops.saturating_add(1);
            self.peers.insert(entry.id.clone(), peer);
            added += 1;
        }
        added
    }
}

/// f6=mesh_status — report mesh subsystem state (legacy compat)
pub fn f6() -> &'static str {
    "offline (no peers)"
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_peer() {
        let mut table = T9::new();
        table.add_peer(T3::new("gf-0001", -75, 90));
        assert_eq!(table.peer_count(), 1);
        let p = table.get_peer("gf-0001").unwrap();
        assert_eq!(p.rssi, -75);
        assert_eq!(p.battery_pct, 90);
    }

    #[test]
    fn remove_peer() {
        let mut table = T9::new();
        table.add_peer(T3::new("gf-0001", -75, 90));
        assert!(table.remove_peer("gf-0001"));
        assert!(!table.remove_peer("gf-0001"));
        assert_eq!(table.peer_count(), 0);
    }

    #[test]
    fn status_reflects_peers() {
        let mut table = T9::new();
        assert_eq!(table.status(), "offline (no peers)");
        table.add_peer(T3::new("gf-0001", -75, 90));
        assert_eq!(table.status(), "online (1 peer)");
        table.add_peer(T3::new("gf-0002", -80, 50));
        assert_eq!(table.status(), "online (2 peers)");
    }

    #[test]
    fn best_route_direct() {
        let mut table = T9::new();
        table.add_peer(T3::new("gf-dest", -60, 100));
        table.add_peer(T3::new("gf-relay", -50, 100));
        let best = table.best_route("gf-dest").unwrap();
        assert_eq!(best.node_id, "gf-dest");
    }

    #[test]
    fn best_route_relay() {
        let mut table = T9::new();
        table.add_peer(T3::new("gf-strong", -50, 100));
        table.add_peer(T3::new("gf-weak", -110, 10));
        // dest not in table — pick best relay
        let best = table.best_route("gf-unknown").unwrap();
        assert_eq!(best.node_id, "gf-strong");
    }

    #[test]
    fn best_route_empty() {
        let table = T9::new();
        assert!(table.best_route("gf-any").is_none());
    }

    #[test]
    fn route_score_prefers_strong_signal() {
        let strong = T3::new("a", -50, 80);
        let weak = T3::new("b", -110, 80);
        assert!(strong.route_score() > weak.route_score());
    }

    #[test]
    fn route_score_prefers_high_battery() {
        let full = T3::new("a", -80, 100);
        let low = T3::new("b", -80, 10);
        assert!(full.route_score() > low.route_score());
    }

    #[test]
    fn evict_stale_peers() {
        let mut table = T9::new();
        let mut old = T3::new("gf-old", -70, 80);
        old.last_seen = 1000; // ancient
        table.add_peer(old);
        table.add_peer(T3::new("gf-fresh", -70, 80));
        let evicted = table.evict_stale(300);
        assert_eq!(evicted, 1);
        assert_eq!(table.peer_count(), 1);
        assert!(table.get_peer("gf-fresh").is_some());
    }

    #[test]
    fn update_existing_peer() {
        let mut table = T9::new();
        table.add_peer(T3::new("gf-0001", -90, 50));
        table.add_peer(T3::new("gf-0001", -60, 95));
        assert_eq!(table.peer_count(), 1);
        let p = table.get_peer("gf-0001").unwrap();
        assert_eq!(p.rssi, -60);
        assert_eq!(p.battery_pct, 95);
    }

    #[test]
    fn legacy_status() {
        assert_eq!(f6(), "offline (no peers)");
    }

    #[test]
    fn export_sync_contains_all_peers() {
        let mut table = T9::new();
        table.add_peer(T3::new("gf-a", -60, 90));
        table.add_peer(T3::new("gf-b", -80, 70));

        let entries = table.f24();
        assert_eq!(entries.len(), 2);
        let ids: Vec<&str> = entries.iter().map(|e| e.id.as_str()).collect();
        assert!(ids.contains(&"gf-a"));
        assert!(ids.contains(&"gf-b"));
    }

    #[test]
    fn export_sync_empty_table() {
        let table = T9::new();
        assert!(table.f24().is_empty());
    }

    #[test]
    fn import_sync_adds_new_peers() {
        let mut table = T9::new();
        let entries = vec![
            crate::packet::T16 { id: "gf-new1".into(), rssi: -70, battery: 80, hops: 1 },
            crate::packet::T16 { id: "gf-new2".into(), rssi: -85, battery: 60, hops: 2 },
        ];
        let added = table.f25(&entries, "gf-sender");
        assert_eq!(added, 2);
        assert_eq!(table.peer_count(), 2);
    }

    #[test]
    fn import_sync_increments_hops() {
        let mut table = T9::new();
        let entries = vec![
            crate::packet::T16 { id: "gf-distant".into(), rssi: -90, battery: 50, hops: 2 },
        ];
        table.f25(&entries, "gf-relay");
        let peer = table.get_peer("gf-distant").unwrap();
        assert_eq!(peer.hop_count, 3); // 2 + 1
    }

    #[test]
    fn import_sync_skips_known_peers() {
        let mut table = T9::new();
        table.add_peer(T3::new("gf-known", -60, 90));

        let entries = vec![
            crate::packet::T16 { id: "gf-known".into(), rssi: -70, battery: 80, hops: 1 },
            crate::packet::T16 { id: "gf-new".into(), rssi: -75, battery: 70, hops: 1 },
        ];
        let added = table.f25(&entries, "gf-sender");
        assert_eq!(added, 1); // only gf-new added
        assert_eq!(table.peer_count(), 2);
        // gf-known should not be overwritten
        let peer = table.get_peer("gf-known").unwrap();
        assert_eq!(peer.rssi, -60); // original value preserved
    }

    #[test]
    fn import_sync_hop_overflow_saturates() {
        let mut table = T9::new();
        let entries = vec![
            crate::packet::T16 { id: "gf-far".into(), rssi: -90, battery: 10, hops: 255 },
        ];
        table.f25(&entries, "gf-relay");
        let peer = table.get_peer("gf-far").unwrap();
        assert_eq!(peer.hop_count, 255); // saturating_add(1) on 255 stays 255
    }
}
