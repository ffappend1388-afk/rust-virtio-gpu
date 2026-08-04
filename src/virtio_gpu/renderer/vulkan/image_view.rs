use ash::{Device, vk};

pub struct ImageView {
    pub view: vk::ImageView,
}

impl ImageView {
    pub fn new(device: &Device, image: vk::Image) -> Result<Self, vk::Result> {
        let info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(vk::Format::B8G8R8A8_UNORM)
            .subresource_range(
                vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1),
            );

        let view = unsafe { device.create_image_view(&info, None)? };

        println!("Image View created.");

        Ok(Self { view })
    }
}
