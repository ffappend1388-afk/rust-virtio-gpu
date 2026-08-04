use ash::{Device, vk};

pub fn write(device: &Device, memory: vk::DeviceMemory, data: &[u8]) -> Result<(), vk::Result> {
    let ptr =
        unsafe { device.map_memory(memory, 0, data.len() as u64, vk::MemoryMapFlags::empty())? };

    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), ptr.cast::<u8>(), data.len());

        device.unmap_memory(memory);
    }

    Ok(())
}
