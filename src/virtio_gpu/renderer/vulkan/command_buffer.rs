use ash::{Device, vk};

pub struct CommandBuffer {
    pub buffer: vk::CommandBuffer,
}

impl CommandBuffer {
    pub fn new(device: &Device, pool: vk::CommandPool) -> Result<Self, vk::Result> {
        let info = vk::CommandBufferAllocateInfo::default()
            .command_pool(pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let buffer = unsafe { device.allocate_command_buffers(&info)? }[0];

        println!("Command Buffer allocated.");

        Ok(Self { buffer })
    }

    pub fn reset(&self, device: &Device, pool: vk::CommandPool) -> Result<(), vk::Result> {
        unsafe {
            device.reset_command_pool(pool, vk::CommandPoolResetFlags::empty())?;
        }

        Ok(())
    }

    pub fn begin(&self, device: &Device) -> Result<(), vk::Result> {
        let info = vk::CommandBufferBeginInfo::default();

        unsafe {
            device.begin_command_buffer(self.buffer, &info)?;
        }

        println!("Begin Command Buffer");

        Ok(())
    }

    pub fn end(&self, device: &Device) -> Result<(), vk::Result> {
        unsafe {
            device.end_command_buffer(self.buffer)?;
        }

        println!("End Command Buffer");

        Ok(())
    }
}
