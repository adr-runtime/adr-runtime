// =============================================================================
// ADR – Agent-Oriented Declarative Runtime
// Layer 2: Library Entry Point
//
// Layer 2 = Intent + Policy + Resolver (declarative logic)
// Layer 1 = Safety Engine (adr-core, ChatGPT's deliverable)
//
// "Layer 1: deterministische Safety Engine.
//  Layer 2: erklärbare Entscheidungslogik."
//
// Authors: Claude (Anthropic) & ChatGPT (OpenAI)
// Version: 0.1.0 – Phase 7 Skeleton
// License: MIT
// Repository: https://github.com/adr-runtime/adr-runtime
// =============================================================================

pub mod policy;
pub mod resolver;
pub mod types;

// Re-export the most commonly used items for convenience
pub use policy::CompiledPolicy;
pub use resolver::{IntentResolver, RuleBasedResolver, RuntimeContext, RuntimeStateSnapshot};
pub use types::{
    Capability, ExecutionDecision, ExecutionPlan, ExecClass, IntentNode,
    NodeId, NodeType, RejectedPlan, RejectionReason, ResolverResult,
    SafetyRule, SafetyViolation, Severity, Thresholds, TrustTier,
    should_execute,
};
