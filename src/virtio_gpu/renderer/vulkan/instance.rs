use ash::{Entry, Instance, vk};

pub struct VulkanInstance {
    pub entry: Entry,
    pub instance: Instance,
}

impl VulkanInstance {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let entry = unsafe { Entry::load()? };

        let app_name = std::ffi::CString::new("rust-virtio-gpu")?;
        let engine_name = std::ffi::CString::new("rust-virtio-gpu")?;

        let app_info = vk::ApplicationInfo::default()
            .application_name(&app_name)
            .engine_name(&engine_name)
            .api_version(vk::API_VERSION_1_3);

        let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        println!("Vulkan Instance created.");

        Ok(Self { entry, instance })
    }
}

impl Drop for VulkanInstance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
