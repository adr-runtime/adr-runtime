use adr_core::{
    capability_name_to_mask,
    CAP_ACTUATOR_CONTROL,
    CAP_FS_WRITE,
    CAP_NET_EXTERNAL,
};

#[test]
fn known_capability_names_map_to_masks() {
    assert_eq!(capability_name_to_mask("fs_write"), Some(CAP_FS_WRITE));
    assert_eq!(capability_name_to_mask("net_external"), Some(CAP_NET_EXTERNAL));
    assert_eq!(
        capability_name_to_mask("actuator_control"),
        Some(CAP_ACTUATOR_CONTROL)
    );
}

#[test]
fn unknown_capability_name_returns_none() {
    assert_eq!(capability_name_to_mask("does_not_exist"), None);
}
