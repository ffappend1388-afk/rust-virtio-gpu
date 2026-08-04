use ash::{Instance, vk};

pub struct QueueFamily {
    pub index: u32,
    pub properties: vk::QueueFamilyProperties,
}

impl QueueFamily {
    pub fn enumerate(instance: &Instance, physical_device: vk::PhysicalDevice) -> Vec<Self> {
        let queues =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        queues
            .into_iter()
            .enumerate()
            .map(|(index, properties)| Self {
                index: index as u32,
                properties,
            })
            .collect()
    }

    pub fn print(queues: &[Self]) {
        for queue in queues {
            println!("Queue {}", queue.index);

            println!(
                " Graphics : {}",
                queue
                    .properties
                    .queue_flags
                    .contains(vk::QueueFlags::GRAPHICS)
            );

            println!(
                " Compute  : {}",
                queue
                    .properties
                    .queue_flags
                    .contains(vk::QueueFlags::COMPUTE)
            );

            println!(
                " Transfer : {}",
                queue
                    .properties
                    .queue_flags
                    .contains(vk::QueueFlags::TRANSFER)
            );

            println!(" Count    : {}", queue.properties.queue_count);

            println!();
        }
    }
    pub fn graphics(queues: &[Self]) -> Option<u32> {
        queues
            .iter()
            .find(|q| q.properties.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|q| q.index)
    }
}
