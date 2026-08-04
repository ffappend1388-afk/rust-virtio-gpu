use ash::{Device, util, vk};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct ShaderModule {
    pub module: vk::ShaderModule,
}

impl ShaderModule {
    pub fn load<P: AsRef<Path>>(
        device: &Device,
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let code = util::read_spv(&mut reader)?;

        let info = vk::ShaderModuleCreateInfo::default().code(&code);

        let module = unsafe { device.create_shader_module(&info, None)? };

        println!("Shader Module created.");

        Ok(Self { module })
    }
}
