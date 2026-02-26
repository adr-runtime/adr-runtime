// =============================================================================
// ADR – Agent-Oriented Declarative Runtime
// Layer 2: Shared Types
//
// These types form the contract between Layer 1 (Safety Engine) and
// Layer 2 (Declarative Logic). No algorithms here – only type definitions.
//
// Authors: Claude (Anthropic) & ChatGPT (OpenAI)
// Version: 0.1.0 – Phase 7 Skeleton
// License: MIT
// =============================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// -----------------------------------------------------------------------------
// Core identifiers
// -----------------------------------------------------------------------------

/// Unique identifier for a graph node.
pub type NodeId = Uuid;

/// A capability string, e.g. "net:api.example.com" or "fs:/data/out"
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Capability(pub String);

impl Capability {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

// -----------------------------------------------------------------------------
// Trust Tier
// Ordered: AiAutonomous < AiProposed < HumanRequired
// A trust tier can only be RAISED by policy, never lowered.
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustTier {
    AiAutonomous,
    AiProposed,
    HumanRequired,
}

// -----------------------------------------------------------------------------
// Execution class
// realtime_safe nodes must NEVER block (no async, no human gates).
// orchestrated nodes may wait (HTTP, human approval, DB writes).
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecClass {
    RealtimeSafe,
    Orchestrated,
}

/// Marker trait: only nodes that never block may implement this.
/// Human-gate futures do NOT implement RealtimeSafe.
/// Enforced at compile time – a RealtimeSafe node cannot contain
/// a blocking future.
pub trait RealtimeSafe: Send {}

// -----------------------------------------------------------------------------
// Graph-IR Node types
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    /// Declarative goal block – no executable code, only specification
    Intent,
    /// Executable unit with effects and capabilities
    Step,
    /// Decision point: auto (ai_autonomous) or human approval gate
    Gate,
    /// Mandatory audit point – always human_required, never overridable
    Checkpoint,
}

/// Contracts attached to a node (pre/post conditions, invariants).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contracts {
    pub pre:  Vec<String>,
    pub post: Vec<String>,
    pub inv:  Vec<String>,
    /// SHA-256 hash of this contract definition – used in ActionLog evidence
    pub contract_hash: Option<String>,
}

/// Stop handlers: what to do when a kill signal arrives at this node.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StopHandlers {
    pub on_soft_stop: Option<String>,
    pub on_hard_stop: Option<String>,
    pub on_freeze:    Option<String>,
}

/// Optional metadata for confidence and risk annotation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeMeta {
    /// 0.0–1.0 confidence annotation (Meta-Layer P6)
    pub confidence: Option<f32>,
    pub source:     Option<String>,
    pub risk:       Option<RiskLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

// -----------------------------------------------------------------------------
// Intent Node (P7)
// Declarative goal block – the entry point for the IntentResolver.
// -----------------------------------------------------------------------------

/// A declarative intent block. No executable code.
/// The IntentResolver uses this to build an ExecutionPlan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentNode {
    pub id:           NodeId,
    pub goal:         String,
    pub constraints:  Vec<String>,
    pub trust_tier:   TrustTier,
    pub capabilities: Vec<Capability>,
}

// -----------------------------------------------------------------------------
// Execution Plan
// Output of the IntentResolver – ordered list of nodes to execute.
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    /// Sequentially ordered node IDs
    pub nodes:       Vec<NodeId>,
    /// Groups of node IDs that can run in parallel (no shared edges)
    pub parallel:    Vec<Vec<NodeId>>,
    /// Mandatory human review stops (checkpoint nodes)
    pub checkpoints: Vec<NodeId>,
}

// -----------------------------------------------------------------------------
// Resolver Result (P7 + confidence_safety from Phase 5)
// -----------------------------------------------------------------------------

/// Result returned by the IntentResolver.
///
/// A plan is executed ONLY when:
///   confidence_safety == 1.0  AND  confidence_semantic >= threshold
///
/// confidence_safety is BINARY: 1.0 or 0.0 – no middle ground.
/// A plan with confidence_safety < 1.0 is NEVER executed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverResult {
    pub plan:                Option<ExecutionPlan>,

    /// Semantic confidence: how well does the plan fulfil the intent?
    /// Range: 0.0–1.0
    pub confidence_semantic: f32,

    /// Safety confidence: are ALL safety constraints satisfied?
    /// Binary: 1.0 (all satisfied) or 0.0 (any violation found).
    pub confidence_safety:   f32,

    /// Nodes in the plan that require human approval before execution.
    pub open_human_gates:    Vec<NodeId>,

    /// Plans that were considered but rejected, with reasons.
    pub rejected_plans:      Vec<RejectedPlan>,

    /// Safety violations found. Empty if confidence_safety == 1.0.
    pub safety_violations:   Vec<SafetyViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedPlan {
    pub nodes:  Vec<NodeId>,
    pub reason: RejectionReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectionReason {
    CapabilityMissing(String),
    TrustTierInsufficient {
        node:     NodeId,
        required: TrustTier,
        actual:   TrustTier,
    },
    ExecClassConflict(NodeId),
    ContractUnverifiable(NodeId),
    PolicyViolation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyViolation {
    pub node_id:  NodeId,
    pub rule:     SafetyRule,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafetyRule {
    TrustTierInsufficient,
    RealtimeSafeBlockingForbidden,
    CapabilityOutOfScope,
    PolicyConstraintViolated(String),
    CheckpointBypassed,
    /// Critical: the kill switch path must always be reachable
    KillSwitchPathBlocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Warning,
    Error,
    Critical,
}

// -----------------------------------------------------------------------------
// Execution Decision
// Final gate before any plan is handed to Layer 1 for execution.
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum ExecutionDecision {
    /// Plan is safe and semantically confident – hand to Layer 1
    Approved,
    /// Safety constraint violated – never execute, no exceptions
    Blocked {
        reason:     String,
        violations: Vec<SafetyViolation>,
    },
    /// Semantically uncertain – human must review before execution
    HumanReviewRequired {
        reason: String,
    },
}

/// Thresholds for execution decisions.
pub struct Thresholds {
    /// Minimum semantic confidence to auto-approve (default: 0.80)
    pub semantic_min: f32,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self { semantic_min: 0.80 }
    }
}

/// Core execution gate – called before any plan reaches Layer 1.
pub fn should_execute(result: &ResolverResult, thresholds: &Thresholds) -> ExecutionDecision {
    // Safety is binary and absolute – checked first, always
    if result.confidence_safety < 1.0 {
        return ExecutionDecision::Blocked {
            reason:     "Safety constraint violated – no exceptions".to_string(),
            violations: result.safety_violations.clone(),
        };
    }
    // Semantic confidence gate
    if result.confidence_semantic < thresholds.semantic_min {
        return ExecutionDecision::HumanReviewRequired {
            reason: format!(
                "Semantic confidence {:.2} below threshold {:.2}",
                result.confidence_semantic, thresholds.semantic_min
            ),
        };
    }
    ExecutionDecision::Approved
}
