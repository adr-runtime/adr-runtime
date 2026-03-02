use crate::killswitch::{KillSwitchChannel, StopSignal};
use crate::runtime_state::RuntimeState;

#[derive(Debug)]
pub enum AdrRuntimeError {
    StateBlocked(RuntimeState),
}

pub struct AdrRuntime<C: KillSwitchChannel> {
    state: RuntimeState,
    kill: C,
}

impl<C: KillSwitchChannel> AdrRuntime<C> {
    pub fn new(kill: C) -> Self {
        Self { state: RuntimeState::Running, kill }
    }

    pub fn state(&self) -> RuntimeState {
        self.state
    }

    /// Phase 8: noop execution to prove state gating and kill switch priority.
    pub fn execute_noop(&mut self) -> Result<(), AdrRuntimeError> {
        if let Some(sig) = self.kill.poll() {
            match sig {
                StopSignal::Freeze => self.state = RuntimeState::Frozen,
                StopSignal::HardStop => self.state = RuntimeState::Halted,
                StopSignal::SoftStop => self.state = RuntimeState::Stopping,
            }
        }

        if self.state >= RuntimeState::Halted {
            return Err(AdrRuntimeError::StateBlocked(self.state));
        }

        Ok(())
    }
}