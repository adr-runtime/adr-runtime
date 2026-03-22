use adr_core::{Effect, ExecClass, Graph, GraphHeader, Node};
use uuid::Uuid;

#[test]
fn graph_roundtrip_json() {
	 


	let id1 = Uuid::new_v4();
	let id2 = Uuid::new_v4();

	let graph = Graph {
		header: GraphHeader {
			graph_version: "0.1".to_string(),
			deterministic_mode: true,
		},
		nodes: vec![
			Node {
				id: id1,
				label: "node_a".to_string(),
				exec_class: ExecClass::Orchestrated,
				effect: Effect::None,
				capabilities: vec![],
				dependencies: vec![],
			},
			Node {
				id: id2,
				label: "node_b".to_string(),
				exec_class: ExecClass::Orchestrated,
				effect: Effect::None,
				capabilities: vec![],
				dependencies: vec![id1],
			},
		],
	};


    let json = serde_json::to_string_pretty(&graph).expect("serialize graph");
    let decoded: Graph = serde_json::from_str(&json).expect("deserialize graph");

    assert_eq!(decoded.header.graph_version, "0.1");
    assert_eq!(decoded.header.deterministic_mode, true);
    assert_eq!(decoded.nodes.len(), 2);    
	assert_eq!(decoded.nodes[0].label, "node_a");
	assert_eq!(decoded.nodes[0].exec_class, ExecClass::Orchestrated);
	assert_eq!(decoded.nodes[0].effect, Effect::None);
	assert_eq!(decoded.nodes[1].dependencies, vec![id1]);	
}