use crate::capability::CapabilitySet;
use crate::graph::{Effect, ExecClass, Graph, Node};
use crate::killswitch::{KillSwitchChannel, StopSignal};
use crate::runtime_state::RuntimeState;

#[derive(Debug)]
pub enum AdrRuntimeError {
    StateBlocked(RuntimeState),
    RealtimeViolation,
    CapabilityNotGranted(u64),
    PlanNodeMissing(crate::graph::NodeId),
}


pub struct AdrRuntime<C: KillSwitchChannel> {
    state: RuntimeState,
    kill: C,
    caps: CapabilitySet,
}

impl<C: KillSwitchChannel> AdrRuntime<C> {
	pub fn new(kill: C) -> Self {
		Self {
			state: RuntimeState::Running,
			kill,
			caps: CapabilitySet::new(),
		}
	}

    pub fn state(&self) -> RuntimeState {
        self.state
    }

    pub fn set_state(&mut self, s: RuntimeState) {
        self.state = s;
    }
	
	pub fn capabilities(&self) -> &CapabilitySet {
		&self.caps
	}

	pub fn capabilities_mut(&self) -> &CapabilitySet {
		&self.caps
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
		
		
		// Phase 14: capability enforcement in executor
		for cap_mask in &node.capabilities {
			if !self.caps.has_mask(*cap_mask) {
				return Err(AdrRuntimeError::CapabilityNotGranted(*cap_mask));
			}
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
	
	
	pub fn execute_plan(
		&mut self,
		plan: &crate::graph::ExecutionPlan,
		graph: &Graph,
	) -> Result<Vec<crate::graph::NodeId>, AdrRuntimeError> {
		let mut executed = Vec::new();

		for node_id in &plan.nodes {
			// Kill switch must be checked before each node in the plan.
			self.poll_kill_switch();

			// For plan execution, only the Running state may start a new node.
			if self.state != RuntimeState::Running {
				return Err(AdrRuntimeError::StateBlocked(self.state));
			}

			let Some(node) = graph.nodes.iter().find(|n| &n.id == node_id) else {
				return Err(AdrRuntimeError::PlanNodeMissing(*node_id));
			};

			self.execute_node(node)?;
			executed.push(*node_id);
		}

		Ok(executed)
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