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

use adr_core::{Effect, RuntimeState};
use crate::policy::CompiledPolicy;
use crate::types::{
    ExecClass, ExecutionPlan, IntentNode, NodeId, ResolverResult, SafetyRule, SafetyViolation,
    Severity,
};
use crate::policy_engine::PolicyEngine; 

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

impl From<RuntimeState> for RuntimeStateSnapshot {
    fn from(s: RuntimeState) -> Self {
        match s {
            RuntimeState::Running => RuntimeStateSnapshot::Running,
            RuntimeState::Stopping => RuntimeStateSnapshot::Stopping,
            RuntimeState::Halted => RuntimeStateSnapshot::Halted,
            RuntimeState::Frozen => RuntimeStateSnapshot::Frozen,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdrNodeMeta {
    pub id: NodeId,
    pub effect: Effect,
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
    pub nodes: Vec<AdrNodeMeta>,
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
        graph: &AdrGraph,
        _policy: &CompiledPolicy,
        context: &RuntimeContext,
    ) -> ResolverResult {
        // Safety must be checked before any policy logic.
        if context.runtime_state != RuntimeStateSnapshot::Running {
            return ResolverResult {
                plan: None,
                confidence_semantic: 0.0,
                confidence_safety: 0.0,
                open_human_gates: vec![],
                rejected_plans: vec![],
                safety_violations: vec![SafetyViolation {
                    node_id: intent.id,
                    rule: SafetyRule::PolicyConstraintViolated("runtime_not_running".to_string()),
                    severity: Severity::Critical,
                }],
            };
        }

        // Phase 16 skeleton: resolver-side policy filter.
        // Currently empty policy = allow all.
		let policy_engine = PolicyEngine::from_compiled_policy(_policy);
		if graph.nodes.is_empty() {
			return ResolverResult {
				plan: None,
				confidence_semantic: 0.0,
				confidence_safety: 0.0,
				open_human_gates: vec![],
				rejected_plans: vec![],
				safety_violations: vec![SafetyViolation {
					node_id: intent.id,
					rule: SafetyRule::PolicyConstraintViolated("empty_graph".to_string()),
					severity: Severity::Error,
				}],
			};
		}
		



		let mut allowed_ids = Vec::new();
		let mut policy_violations = Vec::new();

		for node in &graph.nodes {
			if !policy_engine.allows_with_effect(intent, &node.effect) {
				policy_violations.push(SafetyViolation {
					node_id: node.id,
					rule: SafetyRule::PolicyConstraintViolated(
						"effect_not_allowed_by_policy".to_string(),
					),
					severity: Severity::Error,
				});
				continue;
			}

			allowed_ids.push(node.id);
		}

		if allowed_ids.is_empty() {
			return ResolverResult {
				plan: None,
				confidence_semantic: 0.0,
				confidence_safety: 0.0,
				open_human_gates: vec![],
				rejected_plans: vec![],
				safety_violations: policy_violations,
			};
		}

		let plan = ExecutionPlan {
			nodes: allowed_ids,
			parallel: vec![],
			checkpoints: vec![],
		};





		ResolverResult {
			plan: Some(plan),
			confidence_semantic: 1.0,
			confidence_safety: 1.0,
			open_human_gates: vec![],
			rejected_plans: vec![],
			safety_violations: policy_violations,
		}

    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
	use crate::types::TrustTier;
	use crate::policy::{AuditConfig, KillSwitchConfig, LogLevel, MerkleRootHolder, TimeSource};

    fn make_context(state: RuntimeStateSnapshot) -> RuntimeContext {
        RuntimeContext {
            active_capabilities: vec![],
            runtime_state: state,
            scheduler_class: ExecClass::Orchestrated,
        }
    }

    fn make_intent() -> IntentNode {
        IntentNode {
            id: Uuid::new_v4(),
            goal: "test".to_string(),
            constraints: vec![],
            trust_tier: TrustTier::AiAutonomous,
            capabilities: vec![],
        }
    }

    /// Minimal compiled policy for tests – does not represent a real domain.
    fn stub_policy() -> CompiledPolicy {
        use crate::policy::{
            AuditConfig, KillSwitchConfig, LogLevel, MerkleRootHolder, TimeSource,			
        };

        CompiledPolicy {
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
			allowed_capabilities: vec![],
			minimum_trust_tier: None,
			allowed_effects: None,
        }
    }

    #[test]
    fn resolver_blocks_when_runtime_not_running() {
        let resolver = RuleBasedResolver;
        let intent = make_intent();
		let graph = AdrGraph {
			node_ids: vec![],
			nodes: vec![],
		};
        let policy = stub_policy();
        let context = make_context(RuntimeStateSnapshot::Frozen);

        let result = resolver.resolve(&intent, &graph, &policy, &context);
        assert_eq!(result.confidence_safety, 0.0);
        assert!(!result.safety_violations.is_empty());
    }

    #[test]
    fn resolver_returns_no_plan_when_graph_empty() {
        let resolver = RuleBasedResolver;
        let intent = make_intent();
		let graph = AdrGraph {
			node_ids: vec![],
			nodes: vec![],
		};
        let policy = stub_policy();
        let context = make_context(RuntimeStateSnapshot::Running);

        let result = resolver.resolve(&intent, &graph, &policy, &context);
        assert_eq!(result.confidence_safety, 0.0);
        assert!(result.plan.is_none());
        assert!(!result.safety_violations.is_empty());
    }

    #[test]
    fn resolver_picks_first_node_when_running() {
        let resolver = RuleBasedResolver;
        let intent = make_intent();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
		let graph = AdrGraph {
			node_ids: vec![id1, id2],
			nodes: vec![
				AdrNodeMeta { id: id1, effect: Effect::None },
				AdrNodeMeta { id: id2, effect: Effect::None },
			],
		};

        let policy = stub_policy();
        let context = make_context(RuntimeStateSnapshot::Running);

        let result = resolver.resolve(&intent, &graph, &policy, &context);
        assert_eq!(result.confidence_safety, 1.0);
        assert!(result.safety_violations.is_empty());
        assert!(result.plan.is_some());
        assert_eq!(result.plan.unwrap().nodes, vec![id1, id2]);
    }
	
	#[test]
	fn resolver_blocks_plan_when_effect_not_allowed_by_policy() {
		let resolver = RuleBasedResolver;
		let intent = make_intent();

		let id1 = Uuid::new_v4();
		let graph = AdrGraph {
			node_ids: vec![id1],
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::FsWrite,
				},
			],
		};

		let policy = CompiledPolicy {
			domain: "test".to_string(),
			version: "0.0.1".to_string(),
			policy_hash: "stub".to_string(),
			allowed_capabilities: vec![],
			minimum_trust_tier: None,
			allowed_effects: Some(vec![Effect::None]),
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
		};

		let context = make_context(RuntimeStateSnapshot::Running);

		let result = resolver.resolve(&intent, &graph, &policy, &context);

		assert!(result.plan.is_none());
		assert_eq!(result.confidence_safety, 0.0);
		assert!(!result.safety_violations.is_empty());
		
		match &result.safety_violations[0].rule {
			SafetyRule::PolicyConstraintViolated(msg) => {
				assert_eq!(msg, "effect_not_allowed_by_policy");
			}
			other => panic!("unexpected safety rule: {:?}", other),
		}


		
						
	}
	
	#[test]
	fn resolver_collects_policy_violations_for_disallowed_nodes() {
		let resolver = RuleBasedResolver;
		let intent = make_intent();

		let id1 = Uuid::new_v4();
		let id2 = Uuid::new_v4();

		let graph = AdrGraph {
			node_ids: vec![id1, id2],
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::None,
				},
				AdrNodeMeta {
					id: id2,
					effect: Effect::FsWrite,
				},
			],
		};

		let policy = CompiledPolicy {
			domain: "test".to_string(),
			version: "0.0.1".to_string(),
			policy_hash: "stub".to_string(),
			allowed_capabilities: vec![],
			minimum_trust_tier: None,
			allowed_effects: Some(vec![Effect::None]),
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
		};

		let context = make_context(RuntimeStateSnapshot::Running);

		let result = resolver.resolve(&intent, &graph, &policy, &context);

		assert!(result.plan.is_some());
		assert_eq!(result.plan.as_ref().unwrap().nodes, vec![id1]);
		assert_eq!(result.safety_violations.len(), 1);

		match &result.safety_violations[0].rule {
			SafetyRule::PolicyConstraintViolated(msg) => {
				assert_eq!(msg, "effect_not_allowed_by_policy");
			}
			other => panic!("unexpected safety rule: {:?}", other),
		}
	}
	
	
}

