#[derive(Clone, Copy, Debug)]
pub struct Scanout {
    pub enabled: bool,
    pub resource_id: u32,
    pub width: u32,
    pub height: u32,
}

impl Default for Scanout {
    fn default() -> Self {
        Self {
            enabled: false,
            resource_id: 0,
            width: 0,
            height: 0,
        }
    }
}
