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

use std::collections::{HashMap, HashSet, VecDeque};

use adr_core::{Effect, RuntimeState};
use adr_core::capability_name_to_mask;
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
	
	/// Snapshot of runtime capability masks at resolve time.
	/// The executor re-enforces capability checks independently.
	/// These masks may diverge if capabilities are revoked between
	/// resolve and execute.
	pub active_capability_masks: Vec<u64>,

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
    pub dependencies: Vec<NodeId>,
}

// -----------------------------------------------------------------------------
// Graph abstraction (stub – will reference adr-core types in Phase 8)
// -----------------------------------------------------------------------------

/// Minimal graph representation visible to Layer 2.
/// Full Graph-IR types live in adr-core (Layer 1).
/// This stub will be replaced by a proper reference in Phase 8.
pub struct AdrGraph {
    pub nodes: Vec<AdrNodeMeta>,
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

fn validate_graph_integrity(graph: &AdrGraph) -> Result<(), NodeId> {
    let mut seen = HashSet::new();

    for node in &graph.nodes {
        if !seen.insert(node.id) {
            return Err(node.id);
        }
    }

    Ok(())
}

fn node_participates_in_cycle(start: NodeId, remaining: &[AdrNodeMeta]) -> bool {
    let remaining_ids: HashSet<NodeId> = remaining.iter().map(|n| n.id).collect();

    fn visit(
        node_id: NodeId,
        remaining: &[AdrNodeMeta],
        remaining_ids: &HashSet<NodeId>,
        visiting: &mut HashSet<NodeId>,
        visited: &mut HashSet<NodeId>,
    ) -> bool {
        if visited.contains(&node_id) {
            return false;
        }

        if !visiting.insert(node_id) {
            return true;
        }

        let node = remaining.iter().find(|n| n.id == node_id);

        if let Some(node) = node {
            for dep in &node.dependencies {
                if remaining_ids.contains(dep)
                    && visit(*dep, remaining, remaining_ids, visiting, visited)
                {
                    return true;
                }
            }
        }

        visiting.remove(&node_id);
        visited.insert(node_id);
        false
    }

    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();

    visit(
        start,
        remaining,
        &remaining_ids,
        &mut visiting,
        &mut visited,
    )
}

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
		
		for cap in &intent.capabilities {
			let Some(mask) = capability_name_to_mask(&cap.0) else {
				return ResolverResult {
					plan: None,
					confidence_semantic: 0.0,
					confidence_safety: 0.0,
					open_human_gates: vec![],
					rejected_plans: vec![],
					safety_violations: vec![SafetyViolation {
						node_id: intent.id,
						rule: SafetyRule::CapabilityOutOfScope,
						severity: Severity::Error,
					}],
				};
			};

			if !context.active_capability_masks.contains(&mask) {
				return ResolverResult {
					plan: None,
					confidence_semantic: 0.0,
					confidence_safety: 0.0,
					open_human_gates: vec![],
					rejected_plans: vec![],
					safety_violations: vec![SafetyViolation {
						node_id: intent.id,
						rule: SafetyRule::CapabilityOutOfScope,
						severity: Severity::Error,
					}],
				};
			}
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

		if let Err(node_id) = validate_graph_integrity(graph) {
			return ResolverResult {
				plan: None,
				confidence_semantic: 0.0,
				confidence_safety: 0.0,
				open_human_gates: vec![],
				rejected_plans: vec![],
				safety_violations: vec![SafetyViolation {
					node_id,
					rule: SafetyRule::DuplicateNodeId(node_id),
					severity: Severity::Error,
				}],
			};
		}
		

		let mut allowed_ids = Vec::new();
		let mut policy_violations = Vec::new();
		let allowed_nodes: Vec<AdrNodeMeta> = graph
			.nodes
			.iter()
			.filter_map(|node| {
				if !policy_engine.allows_with_effect(intent, &node.effect) {
					policy_violations.push(SafetyViolation {
						node_id: node.id,
						rule: SafetyRule::PolicyConstraintViolated(
							"effect_not_allowed_by_policy".to_string(),
						),
						severity: Severity::Error,
					});
					return None;
				}

				Some(node.clone())
			})
			.collect();

		let allowed_id_set: HashSet<NodeId> = allowed_nodes.iter().map(|node| node.id).collect();
		let mut remaining_dependency_counts: HashMap<NodeId, usize> = HashMap::new();
		let mut dependents: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
		let mut blocked_by_missing: HashSet<NodeId> = HashSet::new();

		for node in &allowed_nodes {
			let mut internal_dependency_count = 0;

			for dep in &node.dependencies {
				if allowed_id_set.contains(dep) {
					internal_dependency_count += 1;
					dependents.entry(*dep).or_default().push(node.id);
				} else {
					blocked_by_missing.insert(node.id);
				}
			}

			remaining_dependency_counts.insert(node.id, internal_dependency_count);
		}

		let mut ready = VecDeque::new();
		for node in &allowed_nodes {
			if !blocked_by_missing.contains(&node.id)
				&& remaining_dependency_counts.get(&node.id) == Some(&0)
			{
				ready.push_back(node.id);
			}
		}

		let mut parallel_groups = Vec::new();
		while !ready.is_empty() {
			let current_layer_size = ready.len();
			let mut current_layer = Vec::with_capacity(current_layer_size);
			let mut next_ready = Vec::new();

			for _ in 0..current_layer_size {
				let node_id = ready
					.pop_front()
					.expect("ready queue length was captured before draining");
				allowed_ids.push(node_id);
				current_layer.push(node_id);

				if let Some(children) = dependents.get(&node_id) {
					for child_id in children {
						let count = remaining_dependency_counts
							.get_mut(child_id)
							.expect("dependency counts must exist for allowed nodes");
						*count -= 1;

						if *count == 0 && !blocked_by_missing.contains(child_id) {
							next_ready.push(*child_id);
						}
					}
				}
			}

			parallel_groups.push(current_layer);
			for node_id in next_ready {
				ready.push_back(node_id);
			}
		}

		if allowed_ids.len() != allowed_nodes.len() {
			let unresolved_nodes: Vec<AdrNodeMeta> = allowed_nodes
				.iter()
				.filter(|node| !allowed_ids.contains(&node.id))
				.cloned()
				.collect();

			let resolved_id_set: HashSet<NodeId> = allowed_ids.iter().copied().collect();

			for node in unresolved_nodes {
				let rule = if node_participates_in_cycle(node.id, &allowed_nodes) {
					SafetyRule::CycleDetected(node.id)
				} else {
					let missing_dep = node
						.dependencies
						.iter()
						.find(|dep| !resolved_id_set.contains(dep))
						.copied()
						.unwrap_or(node.id);
					SafetyRule::DependencyNotSatisfied(missing_dep)
				};

				policy_violations.push(SafetyViolation {
					node_id: node.id,
					rule,
					severity: Severity::Error,
				});
			}
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
			parallel: parallel_groups,
			checkpoints: vec![],
		};





		ResolverResult {
			plan: Some(plan),
			confidence_semantic: 1.0,
			confidence_safety: if policy_violations.is_empty() { 1.0 } else { 0.0 },
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
	use crate::types::{Capability, TrustTier};
	use crate::policy::{AuditConfig, KillSwitchConfig, LogLevel, MerkleRootHolder, TimeSource};


    fn make_context(state: RuntimeStateSnapshot) -> RuntimeContext {
        RuntimeContext {
            active_capabilities: vec![],
            runtime_state: state,
            scheduler_class: ExecClass::Orchestrated,
			active_capability_masks: vec![],
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
			nodes: vec![
				AdrNodeMeta { id: id1, effect: Effect::None, dependencies: vec![],},
				AdrNodeMeta { id: id2, effect: Effect::None, dependencies: vec![],},
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
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::FsWrite,
					dependencies: vec![],
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
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::None,
					dependencies: vec![],
				},
				AdrNodeMeta {
					id: id2,
					effect: Effect::FsWrite,
					dependencies: vec![],
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
		assert_eq!(result.confidence_safety, 0.0);
		assert_eq!(result.plan.as_ref().unwrap().nodes, vec![id1]);
		assert_eq!(result.safety_violations.len(), 1);

		match &result.safety_violations[0].rule {
			SafetyRule::PolicyConstraintViolated(msg) => {
				assert_eq!(msg, "effect_not_allowed_by_policy");
			}
			other => panic!("unexpected safety rule: {:?}", other),
		}
	}
	
	
	#[test]
	fn resolver_blocks_unknown_capability_with_explicit_violation() {
		let resolver = RuleBasedResolver;

		let intent = IntentNode {
			id: Uuid::new_v4(),
			goal: "test".to_string(),
			constraints: vec![],
			trust_tier: TrustTier::AiAutonomous,
			capabilities: vec![Capability("unknown_cap".to_string())],
		};

		let id1 = Uuid::new_v4();
		let graph = AdrGraph {
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::None,
					dependencies: vec![],
				},
			],
		};

		let policy = stub_policy();
		let context = make_context(RuntimeStateSnapshot::Running);

		let result = resolver.resolve(&intent, &graph, &policy, &context);

		assert!(result.plan.is_none());
		assert_eq!(result.confidence_safety, 0.0);
		assert_eq!(result.safety_violations.len(), 1);

		match result.safety_violations[0].rule {
			SafetyRule::CapabilityOutOfScope => {}
			ref other => panic!("expected CapabilityOutOfScope, got {:?}", other),
		}
	}	
	
	#[test]
	fn resolver_blocks_capability_not_present_in_runtime_context() {
		let resolver = RuleBasedResolver;

		let intent = IntentNode {
			id: Uuid::new_v4(),
			goal: "test".to_string(),
			constraints: vec![],
			trust_tier: TrustTier::AiAutonomous,
			capabilities: vec![Capability("fs_write".to_string())],
		};

		let id1 = Uuid::new_v4();
		let graph = AdrGraph {
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::None,
					dependencies: vec![],
				},
			],
		};

		let policy = stub_policy();

		let context = RuntimeContext {
			runtime_state: RuntimeStateSnapshot::Running,
			scheduler_class: ExecClass::Orchestrated,
			active_capabilities: vec![],
			active_capability_masks: vec![], // <-- fs_write fehlt hier
		};

		let result = resolver.resolve(&intent, &graph, &policy, &context);

		assert!(result.plan.is_none());
		assert_eq!(result.confidence_safety, 0.0);
		assert_eq!(result.safety_violations.len(), 1);

		match result.safety_violations[0].rule {
			SafetyRule::CapabilityOutOfScope => {}
			ref other => panic!("expected CapabilityOutOfScope, got {:?}", other),
		}
	}
	
	
	#[test]
	fn resolver_reorders_nodes_when_dependency_can_be_satisfied_later() {
		let resolver = RuleBasedResolver;
		let intent = make_intent();

		let id1 = Uuid::new_v4();
		let id2 = Uuid::new_v4();

		// Dependent node comes first, but should still be planned after its dependency.
		let graph = AdrGraph {
			nodes: vec![
				AdrNodeMeta {
					id: id2,
					effect: Effect::None,
					dependencies: vec![id1],
				},
				AdrNodeMeta {
					id: id1,
					effect: Effect::None,
					dependencies: vec![],
				},
			],
		};

		let policy = stub_policy();
		let context = make_context(RuntimeStateSnapshot::Running);

		let result = resolver.resolve(&intent, &graph, &policy, &context);

		assert!(result.plan.is_some());
		assert_eq!(result.plan.as_ref().unwrap().nodes, vec![id1, id2]);
		assert_eq!(result.plan.as_ref().unwrap().parallel, vec![vec![id1], vec![id2]]);
		assert!(result.safety_violations.is_empty());
	}

	#[test]
	fn resolver_derives_parallel_groups_from_same_kahn_layer() {
		let resolver = RuleBasedResolver;
		let intent = make_intent();

		let id1 = Uuid::new_v4();
		let id2 = Uuid::new_v4();
		let id3 = Uuid::new_v4();

		let graph = AdrGraph {
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::None,
					dependencies: vec![],
				},
				AdrNodeMeta {
					id: id2,
					effect: Effect::None,
					dependencies: vec![],
				},
				AdrNodeMeta {
					id: id3,
					effect: Effect::None,
					dependencies: vec![id1, id2],
				},
			],
		};

		let policy = stub_policy();
		let context = make_context(RuntimeStateSnapshot::Running);

		let result = resolver.resolve(&intent, &graph, &policy, &context);
		let plan = result.plan.expect("expected plan");

		assert_eq!(plan.nodes, vec![id1, id2, id3]);
		assert_eq!(plan.parallel, vec![vec![id1, id2], vec![id3]]);

		let parallel_flat: Vec<NodeId> = plan.parallel.iter().flatten().copied().collect();
		assert_eq!(parallel_flat, plan.nodes);
	}
	

	#[test]
	fn resolver_blocks_node_when_dependency_cannot_be_satisfied() {
		let resolver = RuleBasedResolver;
		let intent = make_intent();

		let missing_id = Uuid::new_v4();
		let id1 = Uuid::new_v4();

		let graph = AdrGraph {
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::None,
					dependencies: vec![missing_id],
				},
			],
		};

		let policy = stub_policy();
		let context = make_context(RuntimeStateSnapshot::Running);

		let result = resolver.resolve(&intent, &graph, &policy, &context);

		assert!(result.plan.is_none());
		assert_eq!(result.safety_violations.len(), 1);

		match result.safety_violations[0].rule {
			SafetyRule::DependencyNotSatisfied(dep) => {
				assert_eq!(dep, missing_id);
			}
			ref other => panic!("unexpected safety rule: {:?}", other),
		}
	}

	#[test]
	fn resolver_blocks_graph_with_duplicate_node_ids() {
		let resolver = RuleBasedResolver;
		let intent = make_intent();

		let duplicate_id = Uuid::new_v4();
		let graph = AdrGraph {
			nodes: vec![
				AdrNodeMeta {
					id: duplicate_id,
					effect: Effect::None,
					dependencies: vec![],
				},
				AdrNodeMeta {
					id: duplicate_id,
					effect: Effect::None,
					dependencies: vec![],
				},
			],
		};

		let policy = stub_policy();
		let context = make_context(RuntimeStateSnapshot::Running);

		let result = resolver.resolve(&intent, &graph, &policy, &context);

		assert!(result.plan.is_none());
		assert_eq!(result.confidence_safety, 0.0);
		assert_eq!(result.safety_violations.len(), 1);

		match result.safety_violations[0].rule {
			SafetyRule::DuplicateNodeId(node_id) => assert_eq!(node_id, duplicate_id),
			ref other => panic!("unexpected safety rule: {:?}", other),
		}
	}
	
	#[test]
	fn resolver_reports_dependency_not_satisfied_when_no_cycle_exists() {
		let resolver = RuleBasedResolver;
		let intent = make_intent();

		let missing_id = Uuid::new_v4();
		let id1 = Uuid::new_v4();
		let id2 = Uuid::new_v4();

		let graph = AdrGraph {
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::None,
					dependencies: vec![missing_id],
				},
				AdrNodeMeta {
					id: id2,
					effect: Effect::None,
					dependencies: vec![id1],
				},
			],
		};

		let policy = stub_policy();
		let context = make_context(RuntimeStateSnapshot::Running);

		let result = resolver.resolve(&intent, &graph, &policy, &context);

		assert!(result.plan.is_none());
		assert_eq!(result.safety_violations.len(), 2);

		for violation in &result.safety_violations {
			match violation.rule {
				SafetyRule::DependencyNotSatisfied(dep) => {
					assert!(dep == missing_id || dep == id1);
				}
				ref other => panic!("unexpected safety rule: {:?}", other),
			}
		}
	}

	#[test]
	fn resolver_reports_cycle_detected_for_cyclic_graph() {
		let resolver = RuleBasedResolver;
		let intent = make_intent();

		let id1 = Uuid::new_v4();
		let id2 = Uuid::new_v4();

		let graph = AdrGraph {
			nodes: vec![
				AdrNodeMeta {
					id: id1,
					effect: Effect::None,
					dependencies: vec![id2],
				},
				AdrNodeMeta {
					id: id2,
					effect: Effect::None,
					dependencies: vec![id1],
				},
			],
		};

		let policy = stub_policy();
		let context = make_context(RuntimeStateSnapshot::Running);

		let result = resolver.resolve(&intent, &graph, &policy, &context);

		assert!(result.plan.is_none());
		assert_eq!(result.safety_violations.len(), 2);

		for violation in &result.safety_violations {
			match violation.rule {
				SafetyRule::CycleDetected(node_id) => {
					assert!(node_id == id1 || node_id == id2);
				}
				ref other => panic!("unexpected safety rule: {:?}", other),
			}
		}
	}
	
	
}

