use adr_core::{AdrRuntime, RuntimeState};
use adr_core::killswitch::{KillSwitchChannel, StopSignal};

use adr_layer2::IntentResolver;
use adr_layer2::resolver::{AdrGraph, RuleBasedResolver, RuntimeContext, RuntimeStateSnapshot};
use adr_layer2::policy::{
    AuditConfig, CompiledPolicy, KillSwitchConfig, LogLevel, MerkleRootHolder, TimeSource,
};
use adr_layer2::types::{ExecClass, IntentNode, TrustTier};

use uuid::Uuid;

struct NoSignal;
impl KillSwitchChannel for NoSignal {
    fn poll(&self) -> Option<StopSignal> {
        None
    }
}

fn make_context(state: RuntimeState) -> RuntimeContext {
    RuntimeContext {
        active_capabilities: vec![],
        runtime_state: RuntimeStateSnapshot::from(state),
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

fn stub_policy() -> CompiledPolicy {
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
fn e2e_resolve_then_execute_noop() {
    // Layer 1 runtime
    let mut rt = AdrRuntime::new(NoSignal);

    // Layer 2 inputs
    let context = make_context(RuntimeState::Running);
    let intent = make_intent();

    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();
	let graph = AdrGraph {		
		nodes: vec![
			adr_layer2::resolver::AdrNodeMeta { id: id1, effect: adr_core::Effect::None },
			adr_layer2::resolver::AdrNodeMeta { id: id2, effect: adr_core::Effect::None },
		],
	};

    let policy = stub_policy();

    // Resolve (Layer 2)
    let resolver = RuleBasedResolver;
    let result = resolver.resolve(&intent, &graph, &policy, &context);

    assert_eq!(result.confidence_safety, 1.0);
    assert!(result.plan.is_some());
    assert_eq!(result.plan.as_ref().unwrap().nodes, vec![id1, id2]);
	
    // Execute noop (Layer 1)
    rt.execute_noop().expect("runtime execute ok");
}