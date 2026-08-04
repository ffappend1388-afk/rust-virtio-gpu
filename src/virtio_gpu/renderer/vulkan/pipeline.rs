use ash::{vk, Device};

pub struct GraphicsPipeline {
    pub layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}

impl GraphicsPipeline {
    pub fn new(
        device: &Device,
    ) -> Result<Self, vk::Result> {

        let layout_info = vk::PipelineLayoutCreateInfo::default();

        let layout = unsafe {
            device.create_pipeline_layout(&layout_info, None)?
        };

        println!("Pipeline Layout created.");

        Ok(Self {
            layout,
            pipeline: vk::Pipeline::null(),
        })
    }
}