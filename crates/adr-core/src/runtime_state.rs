use core::cmp::Ordering;

/// Safety priority order:
/// Frozen > Halted > Stopping > Running
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RuntimeState {
    Running,
    Stopping,
    Halted,
    Frozen,
}

impl Ord for RuntimeState {
    fn cmp(&self, other: &Self) -> Ordering {
        use RuntimeState::*;
        fn rank(s: RuntimeState) -> u8 {
            match s {
                Running => 0,
                Stopping => 1,
                Halted => 2,
                Frozen => 3,
            }
        }
        rank(*self).cmp(&rank(*other))
    }
}

impl PartialOrd for RuntimeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}