use rust_virtio_gpu::virtio_gpu::renderer::vulkan::VulkanInstance;

fn main() {
    let _vk = VulkanInstance::new().unwrap();
    println!("Done");
}
