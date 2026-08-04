use super::CtrlHeader;
use super::commands::{
    RESP_OK_CAPSET, RESP_OK_CAPSET_INFO, RESP_OK_DISPLAY_INFO, RESP_OK_EDID, RESP_OK_MAP_INFO,
    RESP_OK_NODATA, RESP_OK_RESOURCE_UUID,
};

// -----------------------------------------------------------------------------
// Common response helpers
// -----------------------------------------------------------------------------

fn decode_header(bytes: &[u8], expected_type: u32, size: usize) -> Option<CtrlHeader> {
    if bytes.len() < size {
        return None;
    }

    let header = CtrlHeader::decode_le(bytes)?;

    if header.typ != expected_type {
        return None;
    }

    Some(header)
}

// -----------------------------------------------------------------------------
// Display information
// -----------------------------------------------------------------------------

pub const MAX_SCANOUTS: usize = 16;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub const SIZE: usize = 16;

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..4].copy_from_slice(&self.x.to_le_bytes());
        out[4..8].copy_from_slice(&self.y.to_le_bytes());
        out[8..12].copy_from_slice(&self.width.to_le_bytes());
        out[12..16].copy_from_slice(&self.height.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            x: u32::from_le_bytes(bytes[0..4].try_into().ok()?),
            y: u32::from_le_bytes(bytes[4..8].try_into().ok()?),
            width: u32::from_le_bytes(bytes[8..12].try_into().ok()?),
            height: u32::from_le_bytes(bytes[12..16].try_into().ok()?),
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DisplayOne {
    pub rect: Rect,
    pub enabled: u32,
    pub flags: u32,
}

impl DisplayOne {
    pub const SIZE: usize = 24;

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..16].copy_from_slice(&self.rect.encode_le());
        out[16..20].copy_from_slice(&self.enabled.to_le_bytes());
        out[20..24].copy_from_slice(&self.flags.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            rect: Rect::decode_le(&bytes[0..16])?,
            enabled: u32::from_le_bytes(bytes[16..20].try_into().ok()?),
            flags: u32::from_le_bytes(bytes[20..24].try_into().ok()?),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RespDisplayInfo {
    pub header: CtrlHeader,
    pub pmodes: [DisplayOne; MAX_SCANOUTS],
}

impl RespDisplayInfo {
    pub const SIZE: usize = CtrlHeader::SIZE + DisplayOne::SIZE * MAX_SCANOUTS;

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[..CtrlHeader::SIZE].copy_from_slice(&self.header.encode_le());

        for (index, mode) in self.pmodes.iter().enumerate() {
            let start = CtrlHeader::SIZE + index * DisplayOne::SIZE;
            let end = start + DisplayOne::SIZE;

            out[start..end].copy_from_slice(&mode.encode_le());
        }

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        let header = decode_header(bytes, RESP_OK_DISPLAY_INFO, Self::SIZE)?;

        let mut pmodes = [DisplayOne::default(); MAX_SCANOUTS];

        for (index, mode) in pmodes.iter_mut().enumerate() {
            let start = CtrlHeader::SIZE + index * DisplayOne::SIZE;
            let end = start + DisplayOne::SIZE;

            *mode = DisplayOne::decode_le(&bytes[start..end])?;
        }

        Some(Self { header, pmodes })
    }
}

// -----------------------------------------------------------------------------
// Capset information
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RespCapsetInfo {
    pub header: CtrlHeader,
    pub capset_id: u32,
    pub capset_max_version: u32,
    pub capset_max_size: u32,
    pub padding: u32,
}

impl RespCapsetInfo {
    pub const SIZE: usize = 40;

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        let header = decode_header(bytes, RESP_OK_CAPSET_INFO, Self::SIZE)?;

        Some(Self {
            header,
            capset_id: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            capset_max_version: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
            capset_max_size: u32::from_le_bytes(bytes[32..36].try_into().ok()?),
            padding: u32::from_le_bytes(bytes[36..40].try_into().ok()?),
        })
    }
}

// -----------------------------------------------------------------------------
// Capset
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RespCapset<'a> {
    pub header: CtrlHeader,
    pub data: &'a [u8],
}

impl<'a> RespCapset<'a> {
    pub const HEADER_SIZE: usize = CtrlHeader::SIZE;

    pub fn decode_le(bytes: &'a [u8]) -> Option<Self> {
        let header = decode_header(bytes, RESP_OK_CAPSET, Self::HEADER_SIZE)?;

        Some(Self {
            header,
            data: &bytes[Self::HEADER_SIZE..],
        })
    }
}

// -----------------------------------------------------------------------------
// EDID
// -----------------------------------------------------------------------------

pub const EDID_SIZE: usize = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RespEdid {
    pub header: CtrlHeader,
    pub size: u32,
    pub padding: u32,
    pub edid: [u8; EDID_SIZE],
}

impl RespEdid {
    pub const SIZE: usize = CtrlHeader::SIZE + 8 + EDID_SIZE;

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        let header = decode_header(bytes, RESP_OK_EDID, Self::SIZE)?;

        let mut edid = [0u8; EDID_SIZE];
        edid.copy_from_slice(&bytes[32..32 + EDID_SIZE]);

        Some(Self {
            header,
            size: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            padding: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
            edid,
        })
    }
}

// -----------------------------------------------------------------------------
// Resource UUID
// -----------------------------------------------------------------------------

pub const UUID_SIZE: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RespResourceUuid {
    pub header: CtrlHeader,
    pub uuid: [u8; UUID_SIZE],
}

impl RespResourceUuid {
    pub const SIZE: usize = CtrlHeader::SIZE + UUID_SIZE;

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        let header = decode_header(bytes, RESP_OK_RESOURCE_UUID, Self::SIZE)?;

        let mut uuid = [0u8; UUID_SIZE];
        uuid.copy_from_slice(&bytes[CtrlHeader::SIZE..Self::SIZE]);

        Some(Self { header, uuid })
    }
}

// -----------------------------------------------------------------------------
// Map information
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RespMapInfo {
    pub header: CtrlHeader,
    pub map_info: u32,
    pub padding: u32,
}

impl RespMapInfo {
    pub const SIZE: usize = 32;

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        let header = decode_header(bytes, RESP_OK_MAP_INFO, Self::SIZE)?;

        Some(Self {
            header,
            map_info: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            padding: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RespOkNoData {
    pub header: CtrlHeader,
}

impl RespOkNoData {
    pub const SIZE: usize = CtrlHeader::SIZE;

    pub fn new() -> Self {
        Self {
            header: CtrlHeader::new(RESP_OK_NODATA),
        }
    }

    pub fn encode_le(&self) -> Vec<u8> {
        self.header.encode_le().to_vec()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_round_trip() {
        let rect = Rect {
            x: 10,
            y: 20,
            width: 1920,
            height: 1080,
        };

        let bytes = rect.encode_le();
        let decoded = Rect::decode_le(&bytes).unwrap();

        assert_eq!(rect, decoded);
    }

    #[test]
    fn display_one_round_trip() {
        let display = DisplayOne {
            rect: Rect {
                x: 10,
                y: 20,
                width: 1920,
                height: 1080,
            },
            enabled: 1,
            flags: 0x1234,
        };

        let bytes = display.encode_le();
        let decoded = DisplayOne::decode_le(&bytes).unwrap();

        assert_eq!(display, decoded);
    }

    #[test]
    fn response_sizes_match_protocol() {
        assert_eq!(Rect::SIZE, 16);
        assert_eq!(DisplayOne::SIZE, 24);
        assert_eq!(RespDisplayInfo::SIZE, 408);
        assert_eq!(RespCapsetInfo::SIZE, 40);
        assert_eq!(RespEdid::SIZE, 1056);
        assert_eq!(RespResourceUuid::SIZE, 40);
        assert_eq!(RespMapInfo::SIZE, 32);
    }

    #[test]
    fn display_info_rejects_wrong_response_type() {
        let mut bytes = vec![0u8; RespDisplayInfo::SIZE];

        let header = CtrlHeader::new(RESP_OK_CAPSET_INFO);
        bytes[..CtrlHeader::SIZE].copy_from_slice(&header.encode_le());

        assert_eq!(RespDisplayInfo::decode_le(&bytes), None);
    }

    #[test]
    fn capset_preserves_variable_length_payload() {
        let header = CtrlHeader::new(RESP_OK_CAPSET);

        let payload = [1u8, 2, 3, 4, 5];

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&header.encode_le());
        bytes.extend_from_slice(&payload);

        let response = RespCapset::decode_le(&bytes).unwrap();

        assert_eq!(response.header, header);
        assert_eq!(response.data, &payload);
    }

    #[test]
    fn display_info_round_trip() {
        let response = RespDisplayInfo {
            header: CtrlHeader::new(RESP_OK_DISPLAY_INFO),
            pmodes: [DisplayOne {
                rect: Rect {
                    x: 0,
                    y: 0,
                    width: 1920,
                    height: 1080,
                },
                enabled: 1,
                flags: 0,
            }; MAX_SCANOUTS],
        };

        let bytes = response.encode_le();
        let decoded = RespDisplayInfo::decode_le(&bytes).unwrap();

        assert_eq!(decoded, response);
    }
}
