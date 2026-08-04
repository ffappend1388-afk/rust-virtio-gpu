use ash::{Instance, vk};

pub struct PhysicalDevice {
    pub physical_device: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
}

impl PhysicalDevice {
    pub fn pick(instance: &Instance) -> Result<Self, vk::Result> {
        let devices = unsafe { instance.enumerate_physical_devices()? };

        if devices.is_empty() {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }

        // فعلاً اولین GPU
        let physical_device = devices[0];

        let properties = unsafe { instance.get_physical_device_properties(physical_device) };

        Ok(Self {
            physical_device,
            properties,
        })
    }

    pub fn enumerate(instance: &Instance) -> Result<Vec<Self>, vk::Result> {
        let devices = unsafe { instance.enumerate_physical_devices()? };

        let mut result = Vec::new();

        for device in devices {
            let properties = unsafe { instance.get_physical_device_properties(device) };

            result.push(Self {
                physical_device: device,
                properties,
            });
        }

        Ok(result)
    }

    pub fn print(devices: &[Self]) {
        for (index, gpu) in devices.iter().enumerate() {
            let name = unsafe { std::ffi::CStr::from_ptr(gpu.properties.device_name.as_ptr()) };

            println!("{} : {}", index, name.to_string_lossy());

            println!("Type : {:?}", gpu.properties.device_type);

            println!();
        }
    }

    fn score(properties: &vk::PhysicalDeviceProperties) -> i32 {
        match properties.device_type {
            vk::PhysicalDeviceType::DISCRETE_GPU => 1000,
            vk::PhysicalDeviceType::INTEGRATED_GPU => 500,
            vk::PhysicalDeviceType::VIRTUAL_GPU => 300,
            vk::PhysicalDeviceType::CPU => -100000,
            _ => 0,
        }
    }

    pub fn pick_best(instance: &Instance) -> Result<Self, vk::Result> {
        let gpus = Self::enumerate(instance)?;

        let mut best = None;
        let mut best_score = i32::MIN;

        for gpu in gpus {
            let score = Self::score(&gpu.properties);

            let name = unsafe { std::ffi::CStr::from_ptr(gpu.properties.device_name.as_ptr()) };

            println!("{} -> score {}", name.to_string_lossy(), score);

            if score > best_score {
                best_score = score;
                best = Some(gpu);
            }
        }

        best.ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)
    }
}
