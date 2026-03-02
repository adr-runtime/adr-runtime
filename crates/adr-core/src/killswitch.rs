#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StopSignal {
    SoftStop,
    HardStop,
    Freeze,
}

/// Core interface: platform adapters implement this.
pub trait KillSwitchChannel: Send + Sync {
    fn poll(&self) -> Option<StopSignal>;
}