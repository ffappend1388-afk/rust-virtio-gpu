/// VirtIO-GPU resource pixel formats.
///
/// Values are defined by the VirtIO-GPU specification and are part of the
/// wire protocol, so they must not be changed.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtioGpuFormat {
    B8G8R8A8Unorm = 1,
    B8G8R8X8Unorm = 2,
    A8R8G8B8Unorm = 3,
    X8R8G8B8Unorm = 4,

    R8G8B8A8Unorm = 67,
    X8B8G8R8Unorm = 68,

    A8B8G8R8Unorm = 121,
    R8G8B8X8Unorm = 134,
}

impl VirtioGpuFormat {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub const fn from_u32(value: u32) -> Option<Self> {
        match value {
            1 => Some(Self::B8G8R8A8Unorm),
            2 => Some(Self::B8G8R8X8Unorm),
            3 => Some(Self::A8R8G8B8Unorm),
            4 => Some(Self::X8R8G8B8Unorm),
            67 => Some(Self::R8G8B8A8Unorm),
            68 => Some(Self::X8B8G8R8Unorm),
            121 => Some(Self::A8B8G8R8Unorm),
            134 => Some(Self::R8G8B8X8Unorm),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_values_match_virtio_gpu_spec() {
        assert_eq!(VirtioGpuFormat::B8G8R8A8Unorm.as_u32(), 1);
        assert_eq!(VirtioGpuFormat::B8G8R8X8Unorm.as_u32(), 2);
        assert_eq!(VirtioGpuFormat::A8R8G8B8Unorm.as_u32(), 3);
        assert_eq!(VirtioGpuFormat::X8R8G8B8Unorm.as_u32(), 4);

        assert_eq!(VirtioGpuFormat::R8G8B8A8Unorm.as_u32(), 67);
        assert_eq!(VirtioGpuFormat::X8B8G8R8Unorm.as_u32(), 68);

        assert_eq!(VirtioGpuFormat::A8B8G8R8Unorm.as_u32(), 121);
        assert_eq!(VirtioGpuFormat::R8G8B8X8Unorm.as_u32(), 134);
    }

    #[test]
    fn format_round_trip() {
        let formats = [
            VirtioGpuFormat::B8G8R8A8Unorm,
            VirtioGpuFormat::B8G8R8X8Unorm,
            VirtioGpuFormat::A8R8G8B8Unorm,
            VirtioGpuFormat::X8R8G8B8Unorm,
            VirtioGpuFormat::R8G8B8A8Unorm,
            VirtioGpuFormat::X8B8G8R8Unorm,
            VirtioGpuFormat::A8B8G8R8Unorm,
            VirtioGpuFormat::R8G8B8X8Unorm,
        ];

        for format in formats {
            assert_eq!(VirtioGpuFormat::from_u32(format.as_u32()), Some(format));
        }
    }

    #[test]
    fn unknown_format_is_rejected() {
        assert_eq!(VirtioGpuFormat::from_u32(0), None);
        assert_eq!(VirtioGpuFormat::from_u32(5), None);
        assert_eq!(VirtioGpuFormat::from_u32(u32::MAX), None);
    }
}
