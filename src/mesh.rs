//! Mesh subsystem — cognitive mesh routing and peer discovery
//!
//! T2=MeshNetwork trait, T3=Peer struct, T9=PeerTable in-memory store.
//! Route scoring: RSSI + battery + last_seen recency.

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

/// Max peers held in T9. New peers past the cap evict the LRU entry by `last_seen`.
/// Bounds memory under flood; 64 is plenty for a sub-GHz LoRa neighborhood.
pub const PEER_CAP: usize = 64;

/// Max remembered (src, seq) pairs for duplicate suppression.
/// FIFO eviction once full.
pub const SEEN_CAP: usize = 256;

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
    seen: HashSet<(String, u16)>,
    seen_order: VecDeque<(String, u16)>,
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
            seen: HashSet::new(),
            seen_order: VecDeque::new(),
        }
    }

    /// Remove peers not seen for `max_age_secs`
    pub fn evict_stale(&mut self, max_age_secs: u64) -> usize {
        let cutoff = now_secs().saturating_sub(max_age_secs);
        let before = self.peers.len();
        self.peers.retain(|_, p| p.last_seen >= cutoff);
        before - self.peers.len()
    }

    /// f27=mark_seen — record (src, seq). Returns `true` if this pair was already
    /// present (caller should drop the frame), `false` if newly recorded.
    /// Bounded by `SEEN_CAP` with FIFO eviction.
    pub fn f27(&mut self, src: &str, seq: u16) -> bool {
        let key = (src.to_string(), seq);
        if !self.seen.insert(key.clone()) {
            return true;
        }
        self.seen_order.push_back(key);
        if self.seen_order.len() > SEEN_CAP
            && let Some(oldest) = self.seen_order.pop_front()
        {
            self.seen.remove(&oldest);
        }
        false
    }

    /// Evict the peer with the oldest `last_seen` if the table is at `PEER_CAP`.
    /// No-op if under cap. Used before inserting a *new* peer.
    fn evict_lru(&mut self) {
        if self.peers.len() < PEER_CAP {
            return;
        }
        if let Some(victim) = self
            .peers
            .values()
            .min_by_key(|p| p.last_seen)
            .map(|p| p.node_id.clone())
        {
            self.peers.remove(&victim);
        }
    }
}

impl T2 for T9 {
    fn add_peer(&mut self, peer: T3) {
        if !self.peers.contains_key(&peer.node_id) {
            self.evict_lru();
        }
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
            self.evict_lru();
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

    // --- duplicate suppression (f27) ---

    #[test]
    fn f27_first_time_returns_false() {
        let mut table = T9::new();
        assert!(!table.f27("gf-a", 1));
    }

    #[test]
    fn f27_duplicate_returns_true() {
        let mut table = T9::new();
        assert!(!table.f27("gf-a", 1));
        assert!(table.f27("gf-a", 1));
    }

    #[test]
    fn f27_different_seq_not_duplicate() {
        let mut table = T9::new();
        assert!(!table.f27("gf-a", 1));
        assert!(!table.f27("gf-a", 2));
    }

    #[test]
    fn f27_different_src_not_duplicate() {
        let mut table = T9::new();
        assert!(!table.f27("gf-a", 1));
        assert!(!table.f27("gf-b", 1));
    }

    #[test]
    fn f27_fifo_evicts_oldest_after_cap() {
        let mut table = T9::new();
        // Fill exactly to cap with unique (src, seq)
        for i in 0..(SEEN_CAP as u16) {
            assert!(!table.f27("gf-x", i));
        }
        // One more — pushes (gf-x, 0) out
        assert!(!table.f27("gf-x", SEEN_CAP as u16));
        // Oldest is now forgotten, so re-inserting it returns false (treated as new)
        assert!(!table.f27("gf-x", 0));
        // But the most-recent ones are still remembered
        assert!(table.f27("gf-x", SEEN_CAP as u16));
    }

    // --- peer table cap (PEER_CAP) ---

    #[test]
    fn peer_cap_evicts_oldest_on_overflow() {
        let mut table = T9::new();
        // Fill table to capacity with ascending last_seen so the first peer is the LRU.
        for i in 0..PEER_CAP {
            let mut p = T3::new(&format!("gf-{:03}", i), -70, 80);
            p.last_seen = 1000 + i as u64;
            table.add_peer(p);
        }
        assert_eq!(table.peer_count(), PEER_CAP);
        assert!(table.get_peer("gf-000").is_some());

        // One more new peer — evicts the LRU (gf-000)
        let mut newcomer = T3::new("gf-new", -70, 80);
        newcomer.last_seen = 1000 + PEER_CAP as u64;
        table.add_peer(newcomer);

        assert_eq!(table.peer_count(), PEER_CAP);
        assert!(table.get_peer("gf-000").is_none());
        assert!(table.get_peer("gf-new").is_some());
    }

    #[test]
    fn peer_cap_update_existing_does_not_evict() {
        let mut table = T9::new();
        for i in 0..PEER_CAP {
            let mut p = T3::new(&format!("gf-{:03}", i), -70, 80);
            p.last_seen = 1000 + i as u64;
            table.add_peer(p);
        }
        assert_eq!(table.peer_count(), PEER_CAP);

        // Re-insert an existing peer with updated values — count stays, no eviction.
        table.add_peer(T3::new("gf-000", -50, 95));
        assert_eq!(table.peer_count(), PEER_CAP);
        let p = table.get_peer("gf-000").unwrap();
        assert_eq!(p.rssi, -50);
        assert_eq!(p.battery_pct, 95);
    }

    #[test]
    fn f25_respects_peer_cap() {
        let mut table = T9::new();
        for i in 0..PEER_CAP {
            let mut p = T3::new(&format!("gf-{:03}", i), -70, 80);
            p.last_seen = 1000 + i as u64;
            table.add_peer(p);
        }
        // Sync entries that would push us over cap
        let entries: Vec<crate::packet::T16> = (0..10)
            .map(|i| crate::packet::T16 {
                id: format!("gf-sync-{}", i),
                rssi: -75,
                battery: 70,
                hops: 1,
            })
            .collect();
        let added = table.f25(&entries, "gf-sender");
        assert_eq!(added, 10);
        // Cap holds — older peers were evicted
        assert_eq!(table.peer_count(), PEER_CAP);
    }
}
