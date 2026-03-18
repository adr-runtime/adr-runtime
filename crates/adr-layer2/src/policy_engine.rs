use adr_core::Effect;
use adr_core::capability_name_to_mask;
use crate::policy::CompiledPolicy;
use crate::types::{Capability, IntentNode, TrustTier};


#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub allowed_capabilities: Vec<Capability>,
    pub minimum_trust_tier: Option<TrustTier>,
    pub allowed_effects: Option<Vec<Effect>>,
}



pub struct PolicyEngine {
    pub rules: Vec<PolicyRule>,
}

impl PolicyEngine {
    pub fn new(rules: Vec<PolicyRule>) -> Self {
        Self { rules }
    }
	
	pub fn from_compiled_policy(policy: &CompiledPolicy) -> Self {
		let rule = PolicyRule {
			allowed_capabilities: policy.allowed_capabilities.clone(),
			minimum_trust_tier: policy.minimum_trust_tier.clone(),
			allowed_effects: policy.allowed_effects.clone(),
		};

		Self { rules: vec![rule] }
	}

	pub fn allows(&self, node: &IntentNode) -> bool {
		// Phase 18: capability name must be known / mappable
		for cap in &node.capabilities {
			if capability_name_to_mask(&cap.0).is_none() {
				return false;
			}
		}
				
		for rule in &self.rules {
			if let Some(min_tier) = &rule.minimum_trust_tier {
				if &node.trust_tier < min_tier {
					return false;
				}
			}

			for cap in &node.capabilities {
				if !rule.allowed_capabilities.contains(cap) {
					return false;
				}
			}
		}

		true
	}
	
	pub fn allows_with_effect(&self, node: &IntentNode, effect: &Effect) -> bool {
		for rule in &self.rules {
			if let Some(min_tier) = &rule.minimum_trust_tier {
				if &node.trust_tier < min_tier {
					return false;
				}
			}

			for cap in &node.capabilities {
				if !rule.allowed_capabilities.contains(cap) {
					return false;
				}
			}

			if let Some(allowed_effects) = &rule.allowed_effects {
				if !allowed_effects.contains(effect) {
					return false;
				}
			}
		}

		true
}
	
	

}