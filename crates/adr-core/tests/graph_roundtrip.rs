use adr_core::{Effect, ExecClass, Graph, GraphHeader, Node};
use uuid::Uuid;

#[test]
fn graph_roundtrip_json() {
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

    let json = serde_json::to_string_pretty(&graph).expect("serialize graph");
    let decoded: Graph = serde_json::from_str(&json).expect("deserialize graph");

    assert_eq!(decoded.header.graph_version, "0.1");
    assert_eq!(decoded.header.deterministic_mode, true);
    assert_eq!(decoded.nodes.len(), 1);
    assert_eq!(decoded.nodes[0].label, node.label);
    assert_eq!(decoded.nodes[0].exec_class, node.exec_class);
    assert_eq!(decoded.nodes[0].effect, node.effect);
}