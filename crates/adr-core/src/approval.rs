use serde::{Deserialize, Serialize};

use crate::graph::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApprovalContext {
    pub checkpoint_node: NodeId,
    pub approved_by:     Option<String>,
    pub approved_at:     Option<String>,
}
