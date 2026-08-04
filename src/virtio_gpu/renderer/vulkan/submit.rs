use ash::{Device, vk};

pub fn submit(device: &Device, queue: vk::Queue, cmd: vk::CommandBuffer) -> Result<(), vk::Result> {
    let submit_info = vk::SubmitInfo::default().command_buffers(std::slice::from_ref(&cmd));

    unsafe {
        device.queue_submit(queue, std::slice::from_ref(&submit_info), vk::Fence::null())?;

        device.queue_wait_idle(queue)?;
    }

    Ok(())
}
