use adr_core::{ActionKind, ActionLogEntry, Evidence, NodeId};
use uuid::Uuid;

#[test]
fn linear_hash_chain_changes_with_prev_hash() {
    let first = ActionLogEntry {
        node_id: Uuid::new_v4() as NodeId,
        kind: ActionKind::Execute,
        timestamp_utc: "2026-03-03T10:00:00Z".to_string(),
        success: true,
        evidence: Evidence {
            graph_version: "0.1".to_string(),
            policy_version: "policy-1".to_string(),
            contract_hash: "contract-1".to_string(),
        },
        prev_hash: None,
        entry_hash: String::new(),
    }
    .with_computed_hash();

    let second = ActionLogEntry {
        node_id: Uuid::new_v4() as NodeId,
        kind: ActionKind::Execute,
        timestamp_utc: "2026-03-03T10:01:00Z".to_string(),
        success: true,
        evidence: Evidence {
            graph_version: "0.1".to_string(),
            policy_version: "policy-1".to_string(),
            contract_hash: "contract-1".to_string(),
        },
        prev_hash: Some(first.entry_hash.clone()),
        entry_hash: String::new(),
    }
    .with_computed_hash();

    assert_ne!(first.entry_hash, second.entry_hash);
    assert_eq!(second.prev_hash, Some(first.entry_hash.clone()));
}