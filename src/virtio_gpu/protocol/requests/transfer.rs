use crate::virtio_gpu::protocol::responses::Rect;

pub struct ResourceTransferToHost2D {
    pub resource_id: u32,
    pub rect: Rect,
    pub offset: u64,
}

impl ResourceTransferToHost2D {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 40 {
            return None;
        }

        let resource_id = u32::from_le_bytes(data[24..28].try_into().ok()?);

        let rect = Rect {
            x: u32::from_le_bytes(data[28..32].try_into().ok()?),
            y: u32::from_le_bytes(data[32..36].try_into().ok()?),
            width: u32::from_le_bytes(data[36..40].try_into().ok()?),
            height: u32::from_le_bytes(data[40..44].try_into().ok()?),
        };

        let offset = u64::from_le_bytes(data[48..56].try_into().ok()?);

        Some(Self {
            resource_id,
            rect,
            offset,
        })
    }
}
