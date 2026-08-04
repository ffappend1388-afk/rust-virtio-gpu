use ash::{Device, vk};

pub struct VulkanFramebuffer {
    pub framebuffer: vk::Framebuffer,
}

impl VulkanFramebuffer {
    pub fn new(
        device: &Device,
        render_pass: vk::RenderPass,
        image_view: vk::ImageView,
        width: u32,
        height: u32,
    ) -> Result<Self, vk::Result> {
        let attachments = [image_view];

        let info = vk::FramebufferCreateInfo::default()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(width)
            .height(height)
            .layers(1);

        let framebuffer = unsafe { device.create_framebuffer(&info, None)? };

        println!("Framebuffer created.");

        Ok(Self { framebuffer })
    }
}
