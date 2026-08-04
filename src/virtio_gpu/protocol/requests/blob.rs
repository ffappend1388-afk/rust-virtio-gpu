use crate::virtio_gpu::protocol::{
    CMD_RESOURCE_CREATE_BLOB, CMD_RESOURCE_MAP_BLOB, CMD_RESOURCE_UNMAP_BLOB, CtrlHeader,
};

/// A guest memory entry used by RESOURCE_CREATE_BLOB.
///
/// Wire layout:
/// - addr:   u64
/// - length: u32
/// - padding: u32
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemEntry {
    pub addr: u64,
    pub length: u32,
    pub padding: u32,
}

impl MemEntry {
    pub const SIZE: usize = 16;

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..8].copy_from_slice(&self.addr.to_le_bytes());
        out[8..12].copy_from_slice(&self.length.to_le_bytes());
        out[12..16].copy_from_slice(&self.padding.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            addr: u64::from_le_bytes(bytes[0..8].try_into().ok()?),
            length: u32::from_le_bytes(bytes[8..12].try_into().ok()?),
            padding: u32::from_le_bytes(bytes[12..16].try_into().ok()?),
        })
    }
}

/// RESOURCE_CREATE_BLOB request.
///
/// Fixed portion of the wire structure. Memory entries follow immediately
/// after this structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceCreateBlob {
    pub header: CtrlHeader,
    pub resource_id: u32,
    pub blob_mem: u32,
    pub blob_flags: u32,
    pub nr_entries: u32,
    pub blob_id: u64,
    pub size: u64,
}

impl ResourceCreateBlob {
    pub const SIZE: usize = 56;

    pub fn new(
        resource_id: u32,
        blob_mem: u32,
        blob_flags: u32,
        blob_id: u64,
        size: u64,
        nr_entries: u32,
    ) -> Self {
        Self {
            header: CtrlHeader::new(CMD_RESOURCE_CREATE_BLOB),
            resource_id,
            blob_mem,
            blob_flags,
            nr_entries,
            blob_id,
            size,
        }
    }

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..24].copy_from_slice(&self.header.encode_le());
        out[24..28].copy_from_slice(&self.resource_id.to_le_bytes());
        out[28..32].copy_from_slice(&self.blob_mem.to_le_bytes());
        out[32..36].copy_from_slice(&self.blob_flags.to_le_bytes());
        out[36..40].copy_from_slice(&self.nr_entries.to_le_bytes());
        out[40..48].copy_from_slice(&self.blob_id.to_le_bytes());
        out[48..56].copy_from_slice(&self.size.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            header: CtrlHeader::decode_le(&bytes[0..24])?,
            resource_id: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            blob_mem: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
            blob_flags: u32::from_le_bytes(bytes[32..36].try_into().ok()?),
            nr_entries: u32::from_le_bytes(bytes[36..40].try_into().ok()?),
            blob_id: u64::from_le_bytes(bytes[40..48].try_into().ok()?),
            size: u64::from_le_bytes(bytes[48..56].try_into().ok()?),
        })
    }

    pub fn encode_with_entries(&self, entries: &[MemEntry]) -> Vec<u8> {
        assert_eq!(
            entries.len(),
            self.nr_entries as usize,
            "entry count does not match nr_entries"
        );

        let mut out = Vec::with_capacity(Self::SIZE + entries.len() * MemEntry::SIZE);

        out.extend_from_slice(&self.encode_le());

        for entry in entries {
            out.extend_from_slice(&entry.encode_le());
        }

        out
    }
}

/// RESOURCE_MAP_BLOB request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceMapBlob {
    pub header: CtrlHeader,
    pub resource_id: u32,
    pub padding: u32,
    pub offset: u64,
}

impl ResourceMapBlob {
    pub const SIZE: usize = 40;

    pub fn new(resource_id: u32, offset: u64) -> Self {
        Self {
            header: CtrlHeader::new(CMD_RESOURCE_MAP_BLOB),
            resource_id,
            padding: 0,
            offset,
        }
    }

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..24].copy_from_slice(&self.header.encode_le());
        out[24..28].copy_from_slice(&self.resource_id.to_le_bytes());
        out[28..32].copy_from_slice(&self.padding.to_le_bytes());
        out[32..40].copy_from_slice(&self.offset.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            header: CtrlHeader::decode_le(&bytes[0..24])?,
            resource_id: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            padding: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
            offset: u64::from_le_bytes(bytes[32..40].try_into().ok()?),
        })
    }
}

/// RESOURCE_UNMAP_BLOB request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceUnmapBlob {
    pub header: CtrlHeader,
    pub resource_id: u32,
    pub padding: u32,
}

impl ResourceUnmapBlob {
    pub const SIZE: usize = 32;

    pub fn new(resource_id: u32) -> Self {
        Self {
            header: CtrlHeader::new(CMD_RESOURCE_UNMAP_BLOB),
            resource_id,
            padding: 0,
        }
    }

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..24].copy_from_slice(&self.header.encode_le());
        out[24..28].copy_from_slice(&self.resource_id.to_le_bytes());
        out[28..32].copy_from_slice(&self.padding.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            header: CtrlHeader::decode_le(&bytes[0..24])?,
            resource_id: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            padding: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtio_gpu::protocol::{BLOB_FLAG_USE_MAPPABLE, BLOB_MEM_HOST3D};

    #[test]
    fn mem_entry_round_trip() {
        let entry = MemEntry {
            addr: 0x1122_3344_5566_7788,
            length: 0xAABB_CCDD,
            padding: 0,
        };

        let bytes = entry.encode_le();

        assert_eq!(bytes.len(), MemEntry::SIZE);
        assert_eq!(MemEntry::decode_le(&bytes), Some(entry));
    }

    #[test]
    fn mem_entry_rejects_short_input() {
        assert_eq!(MemEntry::decode_le(&[0u8; 15]), None);
    }

    #[test]
    fn create_blob_round_trip() {
        let request = ResourceCreateBlob::new(
            7,
            BLOB_MEM_HOST3D,
            BLOB_FLAG_USE_MAPPABLE,
            0x1122_3344_5566_7788,
            0x1000,
            2,
        );

        let bytes = request.encode_le();

        assert_eq!(bytes.len(), ResourceCreateBlob::SIZE);

        let decoded = ResourceCreateBlob::decode_le(&bytes).unwrap();

        assert_eq!(decoded, request);
        assert_eq!(decoded.header.typ, CMD_RESOURCE_CREATE_BLOB);
    }

    #[test]
    fn create_blob_with_entries_has_expected_size() {
        let request =
            ResourceCreateBlob::new(7, BLOB_MEM_HOST3D, BLOB_FLAG_USE_MAPPABLE, 42, 8192, 2);

        let entries = [
            MemEntry {
                addr: 0x1000,
                length: 4096,
                padding: 0,
            },
            MemEntry {
                addr: 0x2000,
                length: 4096,
                padding: 0,
            },
        ];

        let bytes = request.encode_with_entries(&entries);

        assert_eq!(bytes.len(), ResourceCreateBlob::SIZE + 2 * MemEntry::SIZE);

        assert_eq!(
            MemEntry::decode_le(
                &bytes[ResourceCreateBlob::SIZE..ResourceCreateBlob::SIZE + MemEntry::SIZE]
            ),
            Some(entries[0])
        );
    }

    #[test]
    fn map_blob_round_trip() {
        let request = ResourceMapBlob::new(42, 0x1234_5678_9ABC_DEF0);

        let bytes = request.encode_le();

        assert_eq!(bytes.len(), ResourceMapBlob::SIZE);
        assert_eq!(ResourceMapBlob::decode_le(&bytes), Some(request));
        assert_eq!(request.header.typ, CMD_RESOURCE_MAP_BLOB);
    }

    #[test]
    fn unmap_blob_round_trip() {
        let request = ResourceUnmapBlob::new(42);

        let bytes = request.encode_le();

        assert_eq!(bytes.len(), ResourceUnmapBlob::SIZE);
        assert_eq!(ResourceUnmapBlob::decode_le(&bytes), Some(request));
        assert_eq!(request.header.typ, CMD_RESOURCE_UNMAP_BLOB);
    }

    #[test]
    fn create_blob_header_is_correct() {
        let request = ResourceCreateBlob::new(1, 2, 3, 4, 5, 0);

        assert_eq!(request.header.typ, CMD_RESOURCE_CREATE_BLOB);
        assert_eq!(request.header.flags, 0);
        assert_eq!(request.header.ctx_id, 0);
        assert_eq!(request.header.fence_id, 0);
    }
    #[test]
    fn create_blob_fixed_size_matches_protocol() {
        assert_eq!(ResourceCreateBlob::SIZE, 56);
    }
}
