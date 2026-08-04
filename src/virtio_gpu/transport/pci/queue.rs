#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct QueueState {
    pub size: u16,
    pub enabled: bool,

    pub desc_addr: u64,
    pub driver_addr: u64,
    pub device_addr: u64,

    pub notify_off: u16,
}