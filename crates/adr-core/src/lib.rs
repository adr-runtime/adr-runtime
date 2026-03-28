//! ADR Layer 1: deterministic safety/runtime core (skeleton)

pub mod runtime_state;
pub mod capability;
pub mod killswitch;
pub mod graph;
pub mod runtime;
pub mod audit;
pub mod capability_ids;
pub mod effect_handler;


pub use runtime::{AdrRuntime, AdrRuntimeError};
pub use runtime_state::RuntimeState;

pub use graph::{Effect, ExecClass, ExecutionPlan, Graph, GraphHeader, Node, NodeId};
pub use audit::{ActionKind, ActionLogEntry, Evidence};

pub use capability_ids::{
    capability_name_to_mask,
    CAP_ACTUATOR_CONTROL,
    CAP_FS_WRITE,
    CAP_NET_EXTERNAL,
};
pub use effect_handler::EffectHandler;
