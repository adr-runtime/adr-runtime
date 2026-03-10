use serde::{Deserialize, Serialize};

use crate::graph::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionKind {
    Resolve,
    Execute,
    Freeze,
    Halt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Evidence {
    pub graph_version: String,
    pub policy_version: String,
    pub contract_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionLogEntry {
    pub node_id: NodeId,
    pub kind: ActionKind,
    pub timestamp_utc: String,
    pub success: bool,
    pub evidence: Evidence,
}