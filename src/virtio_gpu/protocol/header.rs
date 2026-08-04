use std::fmt;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct CtrlHeader {
    pub typ: u32,
    pub flags: u32,
    pub fence_id: u64,
    pub ctx_id: u32,
    pub ring_idx: u8,
    pub padding: [u8; 3],
}

impl CtrlHeader {
    pub const SIZE: usize = 24;

    pub const fn new(typ: u32) -> Self {
        Self {
            typ,
            flags: 0,
            fence_id: 0,
            ctx_id: 0,
            ring_idx: 0,
            padding: [0; 3],
        }
    }

    pub const fn with_context(mut self, ctx_id: u32) -> Self {
        self.ctx_id = ctx_id;
        self
    }

    pub const fn with_fence(mut self, fence_id: u64) -> Self {
        self.fence_id = fence_id;
        self.flags |= 1;
        self
    }

    pub const fn with_ring(mut self, ring_idx: u8) -> Self {
        assert!(ring_idx < 64);
        self.ring_idx = ring_idx;
        self.flags |= 2;
        self
    }
    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..4].copy_from_slice(&self.typ.to_le_bytes());
        out[4..8].copy_from_slice(&self.flags.to_le_bytes());
        out[8..16].copy_from_slice(&self.fence_id.to_le_bytes());
        out[16..20].copy_from_slice(&self.ctx_id.to_le_bytes());
        out[20] = self.ring_idx;
        out[21..24].copy_from_slice(&self.padding);

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            typ: u32::from_le_bytes(bytes[0..4].try_into().ok()?),
            flags: u32::from_le_bytes(bytes[4..8].try_into().ok()?),
            fence_id: u64::from_le_bytes(bytes[8..16].try_into().ok()?),
            ctx_id: u32::from_le_bytes(bytes[16..20].try_into().ok()?),
            ring_idx: bytes[20],
            padding: bytes[21..24].try_into().ok()?,
        })
    }
}

impl fmt::Debug for CtrlHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CtrlHeader")
            .field("typ", &format_args!("{:#x}", self.typ))
            .field("flags", &format_args!("{:#x}", self.flags))
            .field("fence_id", &self.fence_id)
            .field("ctx_id", &self.ctx_id)
            .field("ring_idx", &self.ring_idx)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_round_trip() {
        let header = CtrlHeader {
            typ: 0x0100,
            flags: 0x1234,
            fence_id: 123,
            ctx_id: 42,
            ring_idx: 7,
            padding: [0; 3],
        };

        let bytes = header.encode_le();
        let decoded = CtrlHeader::decode_le(&bytes).unwrap();

        assert_eq!(header, decoded);
    }

    #[test]
    fn header_is_24_bytes() {
        assert_eq!(CtrlHeader::SIZE, 24);
        assert_eq!(std::mem::size_of::<CtrlHeader>(), 24);
    }

    #[test]
    fn header_encodes_little_endian() {
        let header = CtrlHeader {
            typ: 0x1122_3344,
            flags: 0x5566_7788,
            fence_id: 0x1122_3344_5566_7788,
            ctx_id: 0xAABB_CCDD,
            ring_idx: 0xEE,
            padding: [0xFF, 0x00, 0x11],
        };

        let bytes = header.encode_le();

        assert_eq!(&bytes[0..4], &0x1122_3344u32.to_le_bytes());
        assert_eq!(&bytes[4..8], &0x5566_7788u32.to_le_bytes());
        assert_eq!(&bytes[8..16], &0x1122_3344_5566_7788u64.to_le_bytes());
        assert_eq!(&bytes[16..20], &0xAABB_CCDDu32.to_le_bytes());
        assert_eq!(bytes[20], 0xEE);
        assert_eq!(&bytes[21..24], &[0xFF, 0x00, 0x11]);
    }

    #[test]
    fn header_decode_rejects_short_input() {
        let bytes = [0u8; CtrlHeader::SIZE - 1];

        assert_eq!(CtrlHeader::decode_le(&bytes), None);
    }

    #[test]
    fn header_decode_accepts_exact_size() {
        let header = CtrlHeader::new(0x0100)
            .with_context(42)
            .with_fence(0x1122_3344_5566_7788)
            .with_ring(3);

        let bytes = header.encode_le();

        assert_eq!(bytes.len(), CtrlHeader::SIZE);
        assert_eq!(CtrlHeader::decode_le(&bytes), Some(header));
    }

    #[test]
    fn header_decode_accepts_larger_input() {
        let header = CtrlHeader::new(0x0100)
            .with_context(42)
            .with_fence(0x1122_3344_5566_7788)
            .with_ring(3);

        let mut bytes = vec![0u8; CtrlHeader::SIZE + 16];

        bytes[..CtrlHeader::SIZE].copy_from_slice(&header.encode_le());

        assert_eq!(CtrlHeader::decode_le(&bytes), Some(header));
    }

    #[test]
    fn header_builder_helpers_work() {
        let header = CtrlHeader::new(0x1234)
            .with_context(99)
            .with_fence(123456)
            .with_ring(5);

        assert_eq!(header.typ, 0x1234);
        assert_eq!(header.ctx_id, 99);
        assert_eq!(header.fence_id, 123456);
        assert_eq!(header.ring_idx, 5);
    }
    #[test]
    fn header_fence_and_ring_flags_are_set() {
        let header = CtrlHeader::new(0x1234).with_fence(123).with_ring(5);

        assert_ne!(header.flags & 1, 0);
        assert_ne!(header.flags & 2, 0);
        assert_eq!(header.fence_id, 123);
        assert_eq!(header.ring_idx, 5);
    }
}
