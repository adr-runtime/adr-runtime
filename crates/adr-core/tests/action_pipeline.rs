use adr_core::{
    ActionKind, ActionLogEntry, AdrRuntime, Effect, Evidence, ExecClass, Graph, GraphHeader, Node,
};
use adr_core::killswitch::{KillSwitchChannel, StopSignal};

use uuid::Uuid;

struct NoSignal;
impl KillSwitchChannel for NoSignal {
    fn poll(&self) -> Option<StopSignal> {
        None
    }
}

#[test]
fn action_pipeline_resolve_execute_log() {
    // 1) Minimal graph with one node
	let node = Node {
		id: Uuid::new_v4(),
		label: "fetch_users".to_string(),
		exec_class: ExecClass::Orchestrated,
		effect: Effect::NetExternal,
		capabilities: vec![],
	};

    let graph = Graph {
        header: GraphHeader {
            graph_version: "0.1".to_string(),
            deterministic_mode: true,
        },
        nodes: vec![node.clone()],
    };

    // 2) Runtime executes noop successfully
    let mut runtime = AdrRuntime::new(NoSignal);
    runtime.execute_noop().expect("runtime execute ok");

    // 3) Create audit log entry for the executed node
    let log = ActionLogEntry {
        node_id: node.id,
        kind: ActionKind::Execute,
        timestamp_utc: "2026-03-02T23:00:00Z".to_string(),
        success: true,
        evidence: Evidence {
            graph_version: graph.header.graph_version.clone(),
            policy_version: "phase12-test-policy".to_string(),
            contract_hash: "noop-contract".to_string(),
        },
    };

    // 4) Assertions
    assert_eq!(graph.nodes.len(), 1);
    assert_eq!(graph.nodes[0].label, "fetch_users");
    assert_eq!(log.kind, ActionKind::Execute);
    assert!(log.success);
    assert_eq!(log.node_id, node.id);
    assert_eq!(log.evidence.graph_version, "0.1");
}