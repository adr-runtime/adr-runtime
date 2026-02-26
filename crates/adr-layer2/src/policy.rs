// =============================================================================
// ADR – Agent-Oriented Declarative Runtime
// Layer 2: Policy Types
//
// Represents a compiled policy.yaml as Rust structs.
// The policy compiler reads policy.yaml and produces a CompiledPolicy.
// The CompiledPolicy is then applied to the Graph-IR (trust override pass,
// freeze trigger pass, audit pass).
//
// Authors: Claude (Anthropic) & ChatGPT (OpenAI)
// Version: 0.1.0 – Phase 7 Skeleton
// License: MIT
// =============================================================================

use serde::{Deserialize, Serialize};
use crate::types::{Capability, ExecClass, NodeType, RiskLevel, TrustTier};

// -----------------------------------------------------------------------------
// Trust Override
// Trust tier can only be RAISED, never lowered.
// -----------------------------------------------------------------------------

/// A trust override rule from policy.yaml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustOverride {
    pub match_rule:        MatchRule,
    pub set_tier:          TrustTier,
    /// If true, no subsequent policy or runtime call can lower this tier
    pub downgrade_forbidden: bool,
    /// If true, not even the operator can override (used for checkpoints)
    pub immutable:         bool,
}

/// Rules for matching nodes in the graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRule {
    pub effect_prefix: Option<String>,   // e.g. "fs_write" matches "fs_write:/data"
    pub node_type:     Option<NodeType>,
    pub exec_class:    Option<ExecClass>,
    pub capability:    Option<Capability>,
}

// -----------------------------------------------------------------------------
// Freeze Triggers
// Conditions that cause the runtime to enter emergency_freeze state.
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreezeTrigger {
    ContractFailure,
    UnverifiedCapabilityUse,
    TrustTierDowngradeAttempt,
    /// Unexpected capability scope change detected
    CapScopeHashMismatch,
    DeterministicModeViolation,
}

// -----------------------------------------------------------------------------
// Audit Configuration
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Minimal,
    Standard,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MerkleRootHolder {
    /// Operator holds the root locally (default for low-risk domains)
    Local,
    /// External certifier (e.g. medical regulator)
    Certifier { id: String },
    /// Multi-party: operator + regulator + independent auditor
    MultiParty { signers: Vec<MerkleSigner> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleSigner {
    pub role: String,
    pub id:   Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeSource {
    LocalClock,
    SecureNtp,
    HardwareRtc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub log_level:              LogLevel,
    pub merkle_root_holder:     MerkleRootHolder,
    /// How often to write a heartbeat anchor even without actions
    pub merkle_anchor_interval: std::time::Duration,
    pub tamper_evident:         bool,
    pub time_source:            TimeSource,
}

// -----------------------------------------------------------------------------
// Kill Switch Configuration
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KillSwitchChannel {
    UnixSignal,
    HardwareGpio { pin: u8 },
    LocalNamedPipe { path: String },
    LocalHttp { port: u16 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillSwitchConfig {
    /// Critical domains MUST have at least one physical channel
    pub require_physical_channel: bool,
    pub channels:                 Vec<KillSwitchChannel>,
    /// Watchdog timer: if no heartbeat within this duration → hard_stop
    pub watchdog_timer:           Option<std::time::Duration>,
    pub offline_capable:          bool,
}

// -----------------------------------------------------------------------------
// Compiled Policy
// The result of compiling a policy.yaml file.
// Immutable once compiled – applied as graph transformation passes.
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledPolicy {
    pub domain:          String,
    pub version:         String,
    /// SHA-256 hash of the original policy.yaml – stored in ActionLog evidence
    pub policy_hash:     String,

    pub trust_overrides: Vec<TrustOverride>,
    pub freeze_triggers: Vec<FreezeTrigger>,
    pub audit:           AuditConfig,
    pub kill_switch:     KillSwitchConfig,
}

impl CompiledPolicy {
    /// Returns true if this policy requires a physical kill switch channel.
    pub fn requires_physical_kill_switch(&self) -> bool {
        self.kill_switch.require_physical_channel
    }

    /// Returns true if the given freeze trigger is active in this policy.
    pub fn has_freeze_trigger(&self, trigger: &FreezeTrigger) -> bool {
        self.freeze_triggers.contains(trigger)
    }

    /// Returns the effective trust tier for a node, after applying overrides.
    /// Trust tier can only be raised, never lowered.
    pub fn effective_trust_tier(
        &self,
        declared: &TrustTier,
        effect:   Option<&str>,
        node_type: Option<&NodeType>,
        exec_class: Option<&ExecClass>,
    ) -> TrustTier {
        let mut tier = declared.clone();
        for rule in &self.trust_overrides {
            if self.rule_matches(&rule.match_rule, effect, node_type, exec_class) {
                if rule.set_tier > tier {
                    tier = rule.set_tier.clone();
                }
            }
        }
        tier
    }

    fn rule_matches(
        &self,
        rule:      &MatchRule,
        effect:    Option<&str>,
        node_type: Option<&NodeType>,
        exec_class: Option<&ExecClass>,
    ) -> bool {
        if let Some(prefix) = &rule.effect_prefix {
            if let Some(eff) = effect {
                if !eff.starts_with(prefix.as_str()) {
                    return false;
                }
            } else {
                return false;
            }
        }
        if let Some(nt) = &rule.node_type {
            if node_type != Some(nt) {
                return false;
            }
        }
        if let Some(ec) = &rule.exec_class {
            if exec_class != Some(ec) {
                return false;
            }
        }
        true
    }
}
