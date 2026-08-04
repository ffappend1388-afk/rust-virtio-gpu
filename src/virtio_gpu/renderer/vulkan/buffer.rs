use ash::{Device, vk};

pub struct Buffer {
    pub buffer: vk::Buffer,
    pub size: vk::DeviceSize,
}

impl Buffer {
    pub fn new(
        device: &Device,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
    ) -> Result<Self, vk::Result> {
        let info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { device.create_buffer(&info, None)? };

        println!("VkBuffer created ({} bytes)", size);

        Ok(Self { buffer, size })
    }
}
