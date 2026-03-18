pub const CAP_FS_WRITE: u64 = 1 << 0;
pub const CAP_NET_EXTERNAL: u64 = 1 << 1;
pub const CAP_ACTUATOR_CONTROL: u64 = 1 << 2;

pub fn capability_name_to_mask(name: &str) -> Option<u64> {
    match name {
        "fs_write" => Some(CAP_FS_WRITE),
        "net_external" => Some(CAP_NET_EXTERNAL),
        "actuator_control" => Some(CAP_ACTUATOR_CONTROL),
        _ => None,
    }
}
