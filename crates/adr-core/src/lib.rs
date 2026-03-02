//! ADR Layer 1: deterministic safety/runtime core (skeleton)

pub mod runtime_state;
pub mod capability;
pub mod killswitch;
pub mod graph;
pub mod runtime;

pub use runtime::{AdrRuntime, AdrRuntimeError};
pub use runtime_state::RuntimeState;
