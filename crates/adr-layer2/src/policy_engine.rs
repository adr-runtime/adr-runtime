use adr_core::Effect;
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

	pub fn allows(&self, node: &IntentNode) -> bool {
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

}