//! ADR Layer 1: deterministic safety/runtime core (skeleton)

pub mod runtime_state;
pub mod capability;
pub mod killswitch;
pub mod graph;
pub mod runtime;
pub mod audit;

pub use runtime::{AdrRuntime, AdrRuntimeError};
pub use runtime_state::RuntimeState;

pub use graph::{Effect, ExecClass, ExecutionPlan, Graph, GraphHeader, Node, NodeId};
pub use audit::{ActionKind, ActionLogEntry, Evidence};
