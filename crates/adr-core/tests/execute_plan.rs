use adr_core::{
    AdrRuntime, Effect, ExecClass, ExecutionPlan, Graph, GraphHeader, Node,
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
fn execute_plan_runs_all_nodes_in_order() {
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
            },
            Node {
                id: id2,
                label: "node_b".to_string(),
                exec_class: ExecClass::Orchestrated,
                effect: Effect::None,
                capabilities: vec![],
            },
        ],
    };

    let plan = ExecutionPlan {
        nodes: vec![id1, id2],
        parallel: vec![],
        checkpoints: vec![],
    };

    let mut rt = AdrRuntime::new(NoSignal);

    let executed = rt.execute_plan(&plan, &graph).expect("plan should execute");

    assert_eq!(executed, vec![id1, id2]);
}

#[test]
fn execute_plan_fails_when_node_is_missing_from_graph() {
    let id1 = Uuid::new_v4();
    let missing_id = Uuid::new_v4();

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
            },
        ],
    };

    let plan = ExecutionPlan {
        nodes: vec![id1, missing_id],
        parallel: vec![],
        checkpoints: vec![],
    };

    let mut rt = AdrRuntime::new(NoSignal);

    let err = rt.execute_plan(&plan, &graph).unwrap_err();

    match err {
        adr_core::AdrRuntimeError::PlanNodeMissing(id) => {
            assert_eq!(id, missing_id);
        }
        other => panic!("expected PlanNodeMissing, got {:?}", other),
    }
}

use std::sync::Mutex;

struct SoftStopOnSecondPoll(Mutex<u32>);

impl KillSwitchChannel for SoftStopOnSecondPoll {
    fn poll(&self) -> Option<StopSignal> {
        let mut count = self.0.lock().unwrap();
        *count += 1;

        if *count >= 2 {
            Some(StopSignal::SoftStop)
        } else {
            None
        }
    }
}

#[test]
fn execute_plan_stops_between_nodes_when_killswitch_triggers() {
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
            },
            Node {
                id: id2,
                label: "node_b".to_string(),
                exec_class: ExecClass::Orchestrated,
                effect: Effect::None,
                capabilities: vec![],
            },
        ],
    };

    let plan = ExecutionPlan {
        nodes: vec![id1, id2],
        parallel: vec![],
        checkpoints: vec![],
    };

    let mut rt = AdrRuntime::new(SoftStopOnSecondPoll(Mutex::new(0)));

    let err = rt.execute_plan(&plan, &graph).unwrap_err();

    match err {
        adr_core::AdrRuntimeError::StateBlocked(state) => {
            assert_eq!(state, adr_core::RuntimeState::Stopping);
        }
        other => panic!("expected StateBlocked(Stopping), got {:?}", other),
    }
}


