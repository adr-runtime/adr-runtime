// =============================================================================
// ADR – Agent-Oriented Declarative Runtime
// Layer 2: Intent Resolver
//
// The IntentResolver selects an ExecutionPlan that satisfies an IntentNode.
// Phase 7: Trait definition + stub implementation only.
// No algorithm yet – that comes in Phase 8.
//
// Design principles:
//   - Rule-based, not ML-based (deterministic, auditable)
//   - Resolver proposes, never decides (trust tiers + human gates decide)
//   - confidence_safety is BINARY (1.0 or 0.0, never in between)
//
// Authors: ADR Runtime Contributors
// Version: 0.1.0 – Phase 7 Skeleton
// License: MIT
// =============================================================================

use crate::policy::CompiledPolicy;
use crate::types::{
    ExecClass, IntentNode, NodeId, ResolverResult, SafetyRule, SafetyViolation, Severity,
};

// -----------------------------------------------------------------------------
// Runtime Context
// Snapshot of the runtime state visible to the resolver.
// The resolver is read-only: it observes context, never modifies it.
// -----------------------------------------------------------------------------

/// Read-only snapshot of Layer 1 runtime state, passed to the resolver.
pub struct RuntimeContext {
    /// Currently granted capabilities (from Layer 1 CapabilitySet)
    pub active_capabilities: Vec<String>,
    /// Current runtime state (must be Running for resolution to proceed)
    pub runtime_state: RuntimeStateSnapshot,
    /// Scheduler class active in the current execution context
    pub scheduler_class: ExecClass,
}

/// Snapshot of the runtime state – mirrored from Layer 1.
/// The resolver must not accept work if state is not Running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeStateSnapshot {
    Running,
    Stopping,
    Halted,
    Frozen,
}

// -----------------------------------------------------------------------------
// Graph abstraction (stub – will reference adr-core types in Phase 8)
// -----------------------------------------------------------------------------

/// Minimal graph representation visible to Layer 2.
/// Full Graph-IR types live in adr-core (Layer 1).
/// This stub will be replaced by a proper reference in Phase 8.
pub struct AdrGraph {
    /// Node IDs available for planning
    pub node_ids: Vec<NodeId>,
    // Full node data will be fetched from adr-core via a read-only interface
    // node_store: &'a dyn NodeStore,
}

// -----------------------------------------------------------------------------
// IntentResolver Trait
// The central contract for all resolver implementations.
// -----------------------------------------------------------------------------

/// Resolves an IntentNode into an ExecutionPlan.
///
/// Implementations must be:
///   - Deterministic: same inputs -> same output
///   - Read-only: no side effects, no runtime state changes
///   - Fast: resolution happens before execution, must not block
pub trait IntentResolver {
    fn resolve(
        &self,
        intent: &IntentNode,
        graph: &AdrGraph,
        policy: &CompiledPolicy,
        context: &RuntimeContext,
    ) -> ResolverResult;
}

// -----------------------------------------------------------------------------
// RuleBasedResolver – Phase 7 Stub
// Returns a placeholder result. Real algorithm comes in Phase 8.
// -----------------------------------------------------------------------------

/// Stub implementation of the rule-based resolver.
/// Phase 7: compiles and returns safe default values.
/// Phase 8: implements the 5-step selection algorithm.
pub struct RuleBasedResolver;

impl IntentResolver for RuleBasedResolver {
    fn resolve(
        &self,
        intent: &IntentNode,
        _graph: &AdrGraph,
        _policy: &CompiledPolicy,
        context: &RuntimeContext,
    ) -> ResolverResult {
        // Safety check: resolver must not operate if runtime is not Running
        if context.runtime_state != RuntimeStateSnapshot::Running {
            return ResolverResult {
                plan: None,
                confidence_semantic: 0.0,
                confidence_safety: 0.0,
                open_human_gates: vec![],
                rejected_plans: vec![],
                safety_violations: vec![SafetyViolation {
                    node_id: intent.id,
                    rule: SafetyRule::PolicyConstraintViolated(
                        "Runtime is not in Running state".to_string(),
                    ),
                    severity: Severity::Critical,
                }],
            };
        }

        // Phase 7 stub: returns empty plan with placeholder confidence.
        // TODO Phase 8: implement 5-step rule-based selection algorithm:
        //   Step 1 – Filter nodes with undeclared effects or capabilities
        //   Step 2 – Filter nodes with insufficient trust tier
        //   Step 3 – Filter nodes with exec_class conflict
        //   Step 4 – Sort remaining paths by: min human gates, min caps, shortest path
        //   Step 5 – Select best path, compute confidence from fulfilled contracts
        ResolverResult {
            plan: None,
            confidence_semantic: 0.0,
            confidence_safety: 1.0, // No violations found (empty plan)
            open_human_gates: vec![],
            rejected_plans: vec![],
            safety_violations: vec![],
        }
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TrustTier;
    use uuid::Uuid;

    fn make_context(state: RuntimeStateSnapshot) -> RuntimeContext {
        RuntimeContext {
            active_capabilities: vec![],
            runtime_state: state,
            scheduler_class: ExecClass::Orchestrated,
        }
    }

    fn make_intent() -> IntentNode {
        use crate::types::IntentNode;
        IntentNode {
            id: Uuid::new_v4(),
            goal: "Test intent".to_string(),
            constraints: vec![],
            trust_tier: TrustTier::AiAutonomous,
            capabilities: vec![],
        }
    }

    #[test]
    fn resolver_blocks_when_runtime_not_running() {
        let resolver = RuleBasedResolver;
        let intent = make_intent();
        let graph = AdrGraph { node_ids: vec![] };
        let context = make_context(RuntimeStateSnapshot::Frozen);

        let result = resolver.resolve(&intent, &graph, &stub_policy(), &context);
        assert_eq!(result.confidence_safety, 0.0);
        assert!(!result.safety_violations.is_empty());
    }

    #[test]
    fn resolver_returns_safe_when_running() {
        let resolver = RuleBasedResolver;
        let intent = make_intent();
        let graph = AdrGraph { node_ids: vec![] };
        let context = make_context(RuntimeStateSnapshot::Running);

        let result = resolver.resolve(&intent, &graph, &stub_policy(), &context);
        assert_eq!(result.confidence_safety, 1.0);
        assert!(result.safety_violations.is_empty());
    }

    /// Minimal compiled policy for tests – does not represent a real domain.
    fn stub_policy() -> crate::policy::CompiledPolicy {
        use crate::policy::{
            AuditConfig, KillSwitchConfig, LogLevel, MerkleRootHolder, TimeSource,
        };
        crate::policy::CompiledPolicy {
            domain: "test".to_string(),
            version: "0.0.1".to_string(),
            policy_hash: "stub".to_string(),
            trust_overrides: vec![],
            freeze_triggers: vec![],
            audit: AuditConfig {
                log_level: LogLevel::Minimal,
                merkle_root_holder: MerkleRootHolder::Local,
                merkle_anchor_interval: std::time::Duration::from_secs(300),
                tamper_evident: false,
                time_source: TimeSource::LocalClock,
            },
            kill_switch: KillSwitchConfig {
                require_physical_channel: false,
                channels: vec![],
                watchdog_timer: None,
                offline_capable: false,
            },
        }
    }
}
