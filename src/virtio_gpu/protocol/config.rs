/// VirtIO-GPU display-change event.
pub const EVENT_DISPLAY: u32 = 1 << 0;

/// VirtIO-GPU device configuration space.
///
/// The first four fields are part of the base VirtIO-GPU configuration.
/// `blob_alignment` is present when the device exposes the blob-alignment
/// configuration field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GpuConfig {
    pub events_read: u32,
    pub events_clear: u32,
    pub num_scanouts: u32,
    pub num_capsets: u32,
    pub blob_alignment: Option<u32>,
}

impl GpuConfig {
    pub const BASE_SIZE: usize = 16;
    pub const WITH_BLOB_ALIGNMENT_SIZE: usize = 20;

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::BASE_SIZE {
            return None;
        }

        Some(Self {
            events_read: u32::from_le_bytes(bytes[0..4].try_into().ok()?),
            events_clear: u32::from_le_bytes(bytes[4..8].try_into().ok()?),
            num_scanouts: u32::from_le_bytes(bytes[8..12].try_into().ok()?),
            num_capsets: u32::from_le_bytes(bytes[12..16].try_into().ok()?),
            blob_alignment: if bytes.len() >= Self::WITH_BLOB_ALIGNMENT_SIZE {
                Some(u32::from_le_bytes(bytes[16..20].try_into().ok()?))
            } else {
                None
            },
        })
    }

    pub fn display_event_pending(self) -> bool {
        self.events_read & EVENT_DISPLAY != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_decodes_base_layout() {
        let mut bytes = [0u8; 16];

        bytes[0..4].copy_from_slice(&1u32.to_le_bytes());
        bytes[4..8].copy_from_slice(&2u32.to_le_bytes());
        bytes[8..12].copy_from_slice(&1u32.to_le_bytes());
        bytes[12..16].copy_from_slice(&4u32.to_le_bytes());

        let config = GpuConfig::decode_le(&bytes).unwrap();

        assert_eq!(config.events_read, 1);
        assert_eq!(config.events_clear, 2);
        assert_eq!(config.num_scanouts, 1);
        assert_eq!(config.num_capsets, 4);
        assert_eq!(config.blob_alignment, None);
        assert!(config.display_event_pending());
    }

    #[test]
    fn config_decodes_blob_alignment() {
        let mut bytes = [0u8; 20];

        bytes[16..20].copy_from_slice(&4096u32.to_le_bytes());

        let config = GpuConfig::decode_le(&bytes).unwrap();

        assert_eq!(config.blob_alignment, Some(4096));
    }

    #[test]
    fn short_config_is_rejected() {
        assert_eq!(GpuConfig::decode_le(&[0u8; 15]), None);
    }

    #[test]
    fn no_display_event_when_bit_is_clear() {
        let config = GpuConfig {
            events_read: 0,
            ..GpuConfig::default()
        };

        assert!(!config.display_event_pending());
    }

    #[test]
    fn unrelated_event_bits_do_not_trigger_display_event() {
        let config = GpuConfig {
            events_read: 1 << 7,
            ..GpuConfig::default()
        };

        assert!(!config.display_event_pending());
    }
}
