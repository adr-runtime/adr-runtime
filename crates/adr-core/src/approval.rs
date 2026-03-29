use serde::{Deserialize, Serialize};

use crate::graph::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApprovalIdentity {
    pub kind:  String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApprovalContext {
    pub checkpoint_node: NodeId,
    pub approved_by:     Option<ApprovalIdentity>,
    pub approved_at:     Option<String>,
}
