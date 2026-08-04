use ash::{Device, vk};

pub struct CommandPool {
    pub pool: vk::CommandPool,
}

impl CommandPool {
    pub fn new(device: &Device, graphics_queue_family: u32) -> Result<Self, vk::Result> {
        let info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(graphics_queue_family)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let pool = unsafe { device.create_command_pool(&info, None)? };

        println!("Command Pool created.");

        Ok(Self { pool })
    }
}
