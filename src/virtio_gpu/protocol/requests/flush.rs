#[derive(Debug, PartialEq, Eq)]
pub struct ResourceFlush {
    pub resource_id: u32,
    pub rect: [u32; 4],
}

impl ResourceFlush {
    pub const SIZE: usize = 44;

    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }

        let resource_id = u32::from_le_bytes(data[24..28].try_into().ok()?);

        let x = u32::from_le_bytes(data[28..32].try_into().ok()?);
        let y = u32::from_le_bytes(data[32..36].try_into().ok()?);
        let width = u32::from_le_bytes(data[36..40].try_into().ok()?);
        let height = u32::from_le_bytes(data[40..44].try_into().ok()?);

        Some(Self {
            resource_id,
            rect: [x, y, width, height],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_resource_flush() {
        let mut data = [0u8; ResourceFlush::SIZE];

        data[24..28].copy_from_slice(&7u32.to_le_bytes());
        data[28..32].copy_from_slice(&10u32.to_le_bytes());
        data[32..36].copy_from_slice(&20u32.to_le_bytes());
        data[36..40].copy_from_slice(&1920u32.to_le_bytes());
        data[40..44].copy_from_slice(&1080u32.to_le_bytes());

        let request = ResourceFlush::decode(&data).unwrap();

        assert_eq!(request.resource_id, 7);
        assert_eq!(request.rect, [10, 20, 1920, 1080]);
    }

    #[test]
    fn decode_rejects_short_input() {
        let data = [0u8; ResourceFlush::SIZE - 1];

        assert_eq!(ResourceFlush::decode(&data), None);
    }
}
