use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type NodeId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecClass {
    RealtimeSafe,
    Orchestrated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Effect {
    None,
    FsWrite,
    NetExternal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphHeader {
    pub graph_version: String,
    pub deterministic_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub exec_class: ExecClass,
    pub effect: Effect,
    pub capabilities: Vec<u64>,
    pub dependencies: Vec<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub header: GraphHeader,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionPlan {
    pub nodes: Vec<NodeId>,
    pub parallel: Vec<Vec<NodeId>>,
    pub checkpoints: Vec<NodeId>,
}