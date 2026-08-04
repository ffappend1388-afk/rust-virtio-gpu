use ash::{Device, Instance, vk};

pub struct LogicalDevice {
    pub device: Device,
    pub graphics_queue: vk::Queue,
    pub graphics_queue_family: u32,
}

impl LogicalDevice {
    pub fn new(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        graphics_queue_family: u32,
    ) -> Result<Self, vk::Result> {
        let priority = [1.0f32];

        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(graphics_queue_family)
            .queue_priorities(&priority);

        let device_info =
            vk::DeviceCreateInfo::default().queue_create_infos(std::slice::from_ref(&queue_info));

        let device = unsafe { instance.create_device(physical_device, &device_info, None)? };

        let graphics_queue = unsafe { device.get_device_queue(graphics_queue_family, 0) };

        println!("Logical Device created.");

        Ok(Self {
            device,
            graphics_queue,
            graphics_queue_family,
        })
    }

    pub fn queue(&self) -> vk::Queue {
        self.graphics_queue
    }
}
