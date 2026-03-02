use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphHeader {
    pub graph_version: String,
    pub deterministic_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub header: GraphHeader,
    // Phase 8: nodes/edges omitted (stub)
}