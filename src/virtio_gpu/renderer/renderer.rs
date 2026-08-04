use super::error::RendererError;
use super::framebuffer::FrameBuffer;
use crate::virtio_gpu::resource::Resource;

pub trait Renderer {
    fn upload(&mut self, data: &[u8]);

    fn framebuffer(&self) -> &FrameBuffer;

    fn framebuffer_mut(&mut self) -> &mut FrameBuffer;

    fn transfer_resource(&mut self, resource: &mut Resource) -> Result<(), RendererError>;

    fn flush_resource(&mut self, resource: &mut Resource) -> Result<(), RendererError>;
}
