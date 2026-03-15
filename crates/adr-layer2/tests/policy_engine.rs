use adr_layer2::policy_engine::{PolicyEngine, PolicyRule};
use adr_layer2::types::{Capability, IntentNode, TrustTier};
use uuid::Uuid;
use adr_core::Effect;

fn make_intent(capabilities: Vec<Capability>) -> IntentNode {
    IntentNode {
        id: Uuid::new_v4(),
        goal: "test".to_string(),
        constraints: vec![],
        trust_tier: TrustTier::AiAutonomous,
        capabilities,
    }
}

#[test]
fn policy_allows_matching_capabilities() {
	let rule = PolicyRule {
		allowed_capabilities: vec![
			Capability("fs_write".to_string()),
			Capability("net_external".to_string()),
		],
		minimum_trust_tier: None,
		allowed_effects: None,
	};


    let engine = PolicyEngine::new(vec![rule]);

    let intent = make_intent(vec![Capability("fs_write".to_string())]);

    assert!(engine.allows(&intent));
}

#[test]
fn policy_blocks_disallowed_capabilities() {
	let rule = PolicyRule {
		allowed_capabilities: vec![Capability("fs_write".to_string())],
		minimum_trust_tier: None,
		allowed_effects: None,
	};


    let engine = PolicyEngine::new(vec![rule]);

    let intent = make_intent(vec![Capability("net_external".to_string())]);

    assert!(!engine.allows(&intent));
}

#[test]
fn policy_blocks_insufficient_trust_tier() {
    let rule = PolicyRule {
        allowed_capabilities: vec![Capability("fs_write".to_string())],
        minimum_trust_tier: Some(TrustTier::HumanRequired),
		allowed_effects: None,
    };

    let engine = PolicyEngine::new(vec![rule]);

    let intent = make_intent(vec![Capability("fs_write".to_string())]);

    assert!(!engine.allows(&intent));
}

#[test]
fn policy_blocks_disallowed_effect() {
    let rule = PolicyRule {
        allowed_capabilities: vec![Capability("fs_write".to_string())],
        minimum_trust_tier: None,
        allowed_effects: Some(vec![Effect::None]),
    };

    let engine = PolicyEngine::new(vec![rule]);

    let intent = make_intent(vec![Capability("fs_write".to_string())]);

    assert!(!engine.allows_with_effect(&intent, &Effect::FsWrite));
}

#[test]
fn policy_allows_permitted_effect() {
    let rule = PolicyRule {
        allowed_capabilities: vec![Capability("fs_write".to_string())],
        minimum_trust_tier: None,
        allowed_effects: Some(vec![Effect::FsWrite]),
    };

    let engine = PolicyEngine::new(vec![rule]);

    let intent = make_intent(vec![Capability("fs_write".to_string())]);

    assert!(engine.allows_with_effect(&intent, &Effect::FsWrite));
}


