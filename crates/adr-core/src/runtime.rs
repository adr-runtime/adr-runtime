use crate::graph::{Effect, ExecClass, Node};
use crate::killswitch::{KillSwitchChannel, StopSignal};
use crate::runtime_state::RuntimeState;

#[derive(Debug)]
pub enum AdrRuntimeError {
    StateBlocked(RuntimeState),
    RealtimeViolation,
}

pub struct AdrRuntime<C: KillSwitchChannel> {
    state: RuntimeState,
    kill: C,
}

impl<C: KillSwitchChannel> AdrRuntime<C> {
    pub fn new(kill: C) -> Self {
        Self {
            state: RuntimeState::Running,
            kill,
        }
    }

    pub fn state(&self) -> RuntimeState {
        self.state
    }

    pub fn set_state(&mut self, s: RuntimeState) {
        self.state = s;
    }

    /// Phase 8/9: noop execution to prove state gating and kill switch priority.
    pub fn execute_noop(&mut self) -> Result<(), AdrRuntimeError> {
        self.poll_kill_switch();

        if self.state >= RuntimeState::Halted {
            return Err(AdrRuntimeError::StateBlocked(self.state));
        }

        Ok(())
    }

    /// Phase 13: minimal executor for a single node.
    /// No real IO yet - only deterministic dispatch rules.
    pub fn execute_node(&mut self, node: &Node) -> Result<(), AdrRuntimeError> {
        self.poll_kill_switch();

        if self.state >= RuntimeState::Halted {
            return Err(AdrRuntimeError::StateBlocked(self.state));
        }

        match node.exec_class {
            ExecClass::RealtimeSafe => match node.effect {
                Effect::None => Ok(()),
                _ => Err(AdrRuntimeError::RealtimeViolation),
            },
            ExecClass::Orchestrated => match node.effect {
                Effect::None | Effect::FsWrite | Effect::NetExternal => Ok(()),
            },
        }
    }

    fn poll_kill_switch(&mut self) {
        if let Some(sig) = self.kill.poll() {
            match sig {
                StopSignal::Freeze => self.state = RuntimeState::Frozen,
                StopSignal::HardStop => self.state = RuntimeState::Halted,
                StopSignal::SoftStop => self.state = RuntimeState::Stopping,
            }
        }
    }
}