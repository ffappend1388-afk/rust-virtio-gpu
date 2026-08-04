use crate::virtio_gpu::protocol::{CAPSET_VENUS, CMD_GET_CAPSET, CMD_GET_CAPSET_INFO, CtrlHeader};

/// GET_CAPSET_INFO request.
///
/// Wire layout:
/// - CtrlHeader: 24 bytes
/// - capset_index: 4 bytes
/// - padding: 4 bytes
///
/// Total: 32 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetCapsetInfo {
    pub header: CtrlHeader,
    pub capset_index: u32,
    pub padding: u32,
}

impl GetCapsetInfo {
    pub const SIZE: usize = 32;

    pub fn new(capset_index: u32) -> Self {
        Self {
            header: CtrlHeader::new(CMD_GET_CAPSET_INFO),
            capset_index,
            padding: 0,
        }
    }

    pub fn venus(capset_index: u32) -> Self {
        Self::new(capset_index)
    }

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..24].copy_from_slice(&self.header.encode_le());
        out[24..28].copy_from_slice(&self.capset_index.to_le_bytes());
        out[28..32].copy_from_slice(&self.padding.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            header: CtrlHeader::decode_le(&bytes[0..24])?,
            capset_index: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            padding: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
        })
    }
}

/// Response payload of GET_CAPSET_INFO.
///
/// Wire layout:
/// - CtrlHeader: 24 bytes
/// - capset_id: 4 bytes
/// - capset_max_version: 4 bytes
/// - capset_max_size: 4 bytes
/// - padding: 4 bytes
///
/// Total: 40 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CapsetInfo {
    pub header: CtrlHeader,
    pub capset_id: u32,
    pub capset_max_version: u32,
    pub capset_max_size: u32,
    pub padding: u32,
}

impl CapsetInfo {
    pub const SIZE: usize = 40;

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            header: CtrlHeader::decode_le(&bytes[0..24])?,
            capset_id: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            capset_max_version: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
            capset_max_size: u32::from_le_bytes(bytes[32..36].try_into().ok()?),
            padding: u32::from_le_bytes(bytes[36..40].try_into().ok()?),
        })
    }

    pub fn is_venus(self) -> bool {
        self.capset_id == CAPSET_VENUS
    }
}

/// GET_CAPSET request.
///
/// Wire layout:
/// - CtrlHeader: 24 bytes
/// - capset_id: 4 bytes
/// - capset_version: 4 bytes
///
/// Total: 32 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetCapset {
    pub header: CtrlHeader,
    pub capset_id: u32,
    pub capset_version: u32,
}

impl GetCapset {
    pub const SIZE: usize = 32;

    pub fn new(capset_id: u32, capset_version: u32) -> Self {
        Self {
            header: CtrlHeader::new(CMD_GET_CAPSET),
            capset_id,
            capset_version,
        }
    }

    pub fn venus(version: u32) -> Self {
        Self::new(CAPSET_VENUS, version)
    }

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..24].copy_from_slice(&self.header.encode_le());
        out[24..28].copy_from_slice(&self.capset_id.to_le_bytes());
        out[28..32].copy_from_slice(&self.capset_version.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            header: CtrlHeader::decode_le(&bytes[0..24])?,
            capset_id: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            capset_version: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
        })
    }
}

/// Variable-length GET_CAPSET response.
///
/// The response consists of the normal VirtIO-GPU header followed by
/// an opaque capset payload. The payload is intentionally kept as bytes:
/// Venus defines the contents of this data separately.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapsetResponse {
    pub header: CtrlHeader,
    pub data: Vec<u8>,
}

impl CapsetResponse {
    pub const HEADER_SIZE: usize = CtrlHeader::SIZE;

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::HEADER_SIZE {
            return None;
        }

        Some(Self {
            header: CtrlHeader::decode_le(&bytes[..Self::HEADER_SIZE])?,
            data: bytes[Self::HEADER_SIZE..].to_vec(),
        })
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtio_gpu::protocol::{RESP_OK_CAPSET, RESP_OK_CAPSET_INFO};

    #[test]
    fn get_capset_info_size_matches_protocol() {
        assert_eq!(GetCapsetInfo::SIZE, 32);
    }

    #[test]
    fn capset_info_size_matches_protocol() {
        assert_eq!(CapsetInfo::SIZE, 40);
    }

    #[test]
    fn get_capset_size_matches_protocol() {
        assert_eq!(GetCapset::SIZE, 32);
    }

    #[test]
    fn get_capset_info_round_trip() {
        let request = GetCapsetInfo::new(3);

        let bytes = request.encode_le();
        let decoded = GetCapsetInfo::decode_le(&bytes).unwrap();

        assert_eq!(decoded, request);
        assert_eq!(decoded.header.typ, CMD_GET_CAPSET_INFO);
        assert_eq!(decoded.capset_index, 3);
    }

    #[test]
    fn venus_capset_info_request_is_correct() {
        let request = GetCapsetInfo::venus(7);

        assert_eq!(request.header.typ, CMD_GET_CAPSET_INFO);
        assert_eq!(request.capset_index, 7);
    }

    #[test]
    fn capset_info_decodes_venus() {
        let mut bytes = [0u8; CapsetInfo::SIZE];

        bytes[0..4].copy_from_slice(&RESP_OK_CAPSET_INFO.to_le_bytes());
        bytes[24..28].copy_from_slice(&CAPSET_VENUS.to_le_bytes());
        bytes[28..32].copy_from_slice(&1u32.to_le_bytes());
        bytes[32..36].copy_from_slice(&4096u32.to_le_bytes());

        let response = CapsetInfo::decode_le(&bytes).unwrap();

        assert_eq!(response.header.typ, RESP_OK_CAPSET_INFO);
        assert_eq!(response.capset_id, CAPSET_VENUS);
        assert_eq!(response.capset_max_version, 1);
        assert_eq!(response.capset_max_size, 4096);
        assert!(response.is_venus());
    }

    #[test]
    fn capset_info_rejects_short_input() {
        assert_eq!(CapsetInfo::decode_le(&[0u8; 39]), None);
    }

    #[test]
    fn get_capset_round_trip() {
        let request = GetCapset::venus(1);

        let bytes = request.encode_le();
        let decoded = GetCapset::decode_le(&bytes).unwrap();

        assert_eq!(decoded, request);
        assert_eq!(decoded.header.typ, CMD_GET_CAPSET);
        assert_eq!(decoded.capset_id, CAPSET_VENUS);
        assert_eq!(decoded.capset_version, 1);
    }

    #[test]
    fn get_capset_rejects_short_input() {
        assert_eq!(GetCapset::decode_le(&[0u8; 31]), None);
    }

    #[test]
    fn capset_response_preserves_variable_payload() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&CtrlHeader::new(RESP_OK_CAPSET).encode_le());

        let payload = [0x11, 0x22, 0x33, 0x44, 0x55];
        bytes.extend_from_slice(&payload);

        let response = CapsetResponse::decode_le(&bytes).unwrap();

        assert_eq!(response.header.typ, RESP_OK_CAPSET);
        assert_eq!(response.data, payload);
        assert_eq!(response.data_len(), 5);
        assert!(!response.is_empty());
    }

    #[test]
    fn empty_capset_payload_is_supported() {
        let bytes = CtrlHeader::new(RESP_OK_CAPSET).encode_le();

        let response = CapsetResponse::decode_le(&bytes).unwrap();

        assert!(response.is_empty());
        assert_eq!(response.data_len(), 0);
    }

    #[test]
    fn capset_response_rejects_short_header() {
        assert_eq!(CapsetResponse::decode_le(&[0u8; 23]), None);
    }
}
