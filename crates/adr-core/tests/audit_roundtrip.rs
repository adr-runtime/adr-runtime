use adr_core::{ActionKind, ActionLogEntry, Evidence, NodeId};
use uuid::Uuid;

#[test]
fn action_log_roundtrip_json() {
	let entry = ActionLogEntry {
		node_id: Uuid::new_v4() as NodeId,
		kind: ActionKind::Execute,
		timestamp_utc: "2026-03-02T22:30:00Z".to_string(),
		success: true,
		evidence: Evidence {
			graph_version: "0.1".to_string(),
			policy_version: "test-policy-1".to_string(),
			contract_hash: "abc123".to_string(),
		},
		prev_hash: None,
		entry_hash: String::new(),
	}.with_computed_hash();

    let json = serde_json::to_string_pretty(&entry).expect("serialize action log");
    let decoded: ActionLogEntry =
        serde_json::from_str(&json).expect("deserialize action log");

    assert_eq!(decoded.kind, ActionKind::Execute);
    assert_eq!(decoded.success, true);
    assert_eq!(decoded.evidence.graph_version, "0.1");
    assert_eq!(decoded.evidence.policy_version, "test-policy-1");
    assert_eq!(decoded.evidence.contract_hash, "abc123");
	assert!(!decoded.entry_hash.is_empty());
	assert_eq!(decoded.prev_hash, None);
}