use adr_core::{
    AdrRuntime, AdrRuntimeError, Effect, ExecClass, Node, RuntimeState,
};
use adr_core::killswitch::{KillSwitchChannel, StopSignal};
use uuid::Uuid;

struct NoSignal;
impl KillSwitchChannel for NoSignal {
    fn poll(&self) -> Option<StopSignal> {
        None
    }
}

struct FreezeOnce(std::sync::Mutex<bool>);
impl KillSwitchChannel for FreezeOnce {
    fn poll(&self) -> Option<StopSignal> {
        let mut seen = self.0.lock().unwrap();
        if *seen {
            None
        } else {
            *seen = true;
            Some(StopSignal::Freeze)
        }
    }
}

#[test]
fn orchestrated_node_executes() {
	let node = Node {
		id: Uuid::new_v4(),
		label: "fetch_users".to_string(),
		exec_class: ExecClass::Orchestrated,
		effect: Effect::NetExternal,
		capabilities: vec![],
	};

    let mut rt = AdrRuntime::new(NoSignal);
    rt.execute_node(&node).expect("orchestrated node should execute");
}

#[test]
fn realtime_safe_node_with_effect_is_rejected() {
    let node = Node {
        id: Uuid::new_v4(),
        label: "unsafe_rt".to_string(),
        exec_class: ExecClass::RealtimeSafe,
        effect: Effect::NetExternal,
		capabilities: vec![],
    };

    let mut rt = AdrRuntime::new(NoSignal);
    let err = rt.execute_node(&node).unwrap_err();

    match err {
        AdrRuntimeError::RealtimeViolation => {}
        _ => panic!("expected RealtimeViolation"),
    }
}

#[test]
fn freeze_blocks_execution() {
    let node = Node {
        id: Uuid::new_v4(),
        label: "noop".to_string(),
        exec_class: ExecClass::Orchestrated,
        effect: Effect::None,
		capabilities: vec![],
    };

    let mut rt = AdrRuntime::new(FreezeOnce(std::sync::Mutex::new(false)));

    let err = rt.execute_node(&node).unwrap_err();
    match err {
        AdrRuntimeError::StateBlocked(RuntimeState::Frozen) => {}
        _ => panic!("expected StateBlocked(Frozen)"),
    }
}

// Node verlangt Capability -> Runtime lehnt ab
#[test]
fn node_without_granted_capability_is_rejected() {
    let node = Node {
        id: Uuid::new_v4(),
        label: "fs_write".to_string(),
        exec_class: ExecClass::Orchestrated,
        effect: Effect::FsWrite,
        capabilities: vec![1 << 3],
    };

    let mut rt = AdrRuntime::new(NoSignal);
    let err = rt.execute_node(&node).unwrap_err();

    match err {
        AdrRuntimeError::CapabilityNotGranted(mask) => {
            assert_eq!(mask, 1 << 3);
        }
        _ => panic!("expected CapabilityNotGranted"),
    }
}

// Node verlangt Capability -> Runtime hat Capability -> Node darf laufen
#[test]
fn node_with_granted_capability_executes() {
    let node = Node {
        id: Uuid::new_v4(),
        label: "fs_write".to_string(),
        exec_class: ExecClass::Orchestrated,
        effect: Effect::FsWrite,
        capabilities: vec![1 << 3],
    };

    let mut rt = AdrRuntime::new(NoSignal);
    rt.capabilities().allow_mask(1 << 3);

    rt.execute_node(&node)
        .expect("node with granted capability should execute");
}