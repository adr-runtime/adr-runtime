use core::sync::atomic::{AtomicU64, Ordering};

/// Minimal deterministic CapabilitySet (Phase 8 skeleton).
#[derive(Debug)]
pub struct CapabilitySet {
    bits: AtomicU64,
}

impl CapabilitySet {
    pub fn new() -> Self {
        Self { bits: AtomicU64::new(0) }
    }

    pub fn allow_mask(&self, mask: u64) {
        self.bits.fetch_or(mask, Ordering::SeqCst);
    }

    pub fn has_mask(&self, mask: u64) -> bool {
        (self.bits.load(Ordering::SeqCst) & mask) == mask
    }

    /// P8 requirement: atomic revocation.
    pub fn revoke_all(&self) {
        self.bits.store(0, Ordering::SeqCst);
    }
}