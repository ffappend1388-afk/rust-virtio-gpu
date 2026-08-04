use ash::vk;
use std::collections::HashMap;

use crate::virtio_gpu::protocol::formats::VirtioGpuFormat;
use crate::virtio_gpu::protocol::requests::attach_backing::VirtioGpuMemEntry;

pub type ResourceId = u32;

#[derive(Debug)]
pub struct Resource {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub format: VirtioGpuFormat,

    pub data: Vec<u8>,

    pub backing: Vec<VirtioGpuMemEntry>,

    pub dirty: Option<[u32; 4]>,
    pub vk_buffer: Option<vk::Buffer>,
    pub vk_buffer_memory: Option<vk::DeviceMemory>,

    pub vk_image: Option<vk::Image>,
    pub vk_image_memory: Option<vk::DeviceMemory>,
}

impl Resource {
    pub fn new(id: ResourceId, width: u32, height: u32, format: VirtioGpuFormat) -> Self {
        let size = width as usize * height as usize * 4;

        Self {
            id,
            width,
            height,
            format,
            backing: Vec::new(),
            data: vec![0; size],
            dirty: None,
            vk_buffer: None,
            vk_buffer_memory: None,

            vk_image: None,
            vk_image_memory: None,
        }
    }

    pub fn write_backing(&mut self, offset: usize, data: &[u8]) -> Result<(), ()> {
        if offset + data.len() > self.data.len() {
            return Err(());
        }

        self.data[offset..offset + data.len()].copy_from_slice(data);

        Ok(())
    }
    pub fn pixels(&self) -> &[u8] {
        &self.data
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

#[derive(Default)]
pub struct ResourceTable {
    resources: HashMap<ResourceId, Resource>,
}

impl ResourceTable {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn attach_backing(
        &mut self,
        id: ResourceId,
        entries: Vec<VirtioGpuMemEntry>,
    ) -> Result<(), ()> {
        let resource = self.resources.get_mut(&id).ok_or(())?;

        resource.backing = entries;

        Ok(())
    }

    pub fn insert(&mut self, resource: Resource) -> bool {
        if self.resources.contains_key(&resource.id) {
            return false;
        }

        self.resources.insert(resource.id, resource);
        true
    }

    pub fn get(&self, id: ResourceId) -> Option<&Resource> {
        self.resources.get(&id)
    }

    // اضافه شود برای تغییر framebuffer
    pub fn get_mut(&mut self, id: ResourceId) -> Option<&mut Resource> {
        self.resources.get_mut(&id)
    }

    pub fn remove(&mut self, id: ResourceId) -> Option<Resource> {
        self.resources.remove(&id)
    }

    pub fn contains(&self, id: ResourceId) -> bool {
        self.resources.contains_key(&id)
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::virtio_gpu::protocol::formats::VirtioGpuFormat;

    #[test]
    fn framebuffer_size_is_correct() {
        let resource = Resource::new(1, 1920, 1080, VirtioGpuFormat::B8G8R8A8Unorm);

        assert_eq!(resource.pixels().len(), 1920 * 1080 * 4);
    }

    #[test]
    fn framebuffer_can_be_modified() {
        let mut resource = Resource::new(1, 2, 2, VirtioGpuFormat::B8G8R8A8Unorm);

        resource.pixels_mut()[0] = 0xff;

        assert_eq!(resource.pixels()[0], 0xff);
    }
}
