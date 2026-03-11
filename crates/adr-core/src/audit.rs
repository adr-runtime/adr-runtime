use serde::{Deserialize, Serialize};
use crate::graph::NodeId;
use sha2::{Digest, Sha256};

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

    pub prev_hash: Option<String>,
    pub entry_hash: String,
}

impl ActionLogEntry {
    pub fn compute_entry_hash(&self) -> String {
        let mut hasher = Sha256::new();

        hasher.update(self.node_id.to_string().as_bytes());
        hasher.update(format!("{:?}", self.kind).as_bytes());
        hasher.update(self.timestamp_utc.as_bytes());
        hasher.update(if self.success { b"1" } else { b"0" });

        hasher.update(self.evidence.graph_version.as_bytes());
        hasher.update(self.evidence.policy_version.as_bytes());
        hasher.update(self.evidence.contract_hash.as_bytes());

        match &self.prev_hash {
            Some(prev) => hasher.update(prev.as_bytes()),
            None => hasher.update(b"GENESIS"),
        }

        let digest = hasher.finalize();
        hex::encode(digest)
    }

    pub fn with_computed_hash(mut self) -> Self {
        self.entry_hash = self.compute_entry_hash();
        self
    }
}