use crate::virtio_gpu::protocol::header::CtrlHeader;

#[repr(C)]
pub struct VirtioGpuResourceAttachBacking {
    pub header: CtrlHeader,
    pub resource_id: u32,
    pub nr_entries: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VirtioGpuMemEntry {
    pub addr: u64,
    pub length: u32,
    pub padding: u32,
}
pub struct ResourceAttachBacking {
    pub resource_id: u32,
    pub nr_entries: u32,
    pub entries: Vec<VirtioGpuMemEntry>,
}

impl ResourceAttachBacking {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 32 {
            return None;
        }

        let resource_id = u32::from_le_bytes(data[24..28].try_into().ok()?);

        let nr_entries = u32::from_le_bytes(data[28..32].try_into().ok()?);

        let mut entries = Vec::new();

        let mut offset = 32;

        for _ in 0..nr_entries {
            if data.len() < offset + 16 {
                return None;
            }

            let addr = u64::from_le_bytes(data[offset..offset + 8].try_into().ok()?);

            let length = u32::from_le_bytes(data[offset + 8..offset + 12].try_into().ok()?);

            let padding = u32::from_le_bytes(data[offset + 12..offset + 16].try_into().ok()?);

            entries.push(VirtioGpuMemEntry {
                addr,
                length,
                padding,
            });

            offset += 16;
        }

        Some(Self {
            resource_id,
            nr_entries,
            entries,
        })
    }
}
