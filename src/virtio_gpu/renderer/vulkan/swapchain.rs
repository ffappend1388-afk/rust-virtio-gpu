use ash::{Device, Instance, khr::swapchain, vk};

pub struct Swapchain {
    pub loader: swapchain::Device,
    pub swapchain: vk::SwapchainKHR,
}

impl Swapchain {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let loader = swapchain::Device::new(instance, device);

        println!("Swapchain initialized.");

        Self {
            loader,
            swapchain: vk::SwapchainKHR::null(),
        }
    }
}
