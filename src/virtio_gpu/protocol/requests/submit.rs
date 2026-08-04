use crate::virtio_gpu::protocol::{CMD_SUBMIT_3D, CtrlHeader};

/// SUBMIT_3D request.
///
/// The command stream itself is carried in the descriptor chain after this
/// fixed request header.
///
/// Wire layout:
/// - CtrlHeader: 24 bytes
/// - size:         4 bytes
/// - padding:      4 bytes
///
/// Total: 32 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Submit3D {
    pub header: CtrlHeader,
    pub size: u32,
    pub padding: u32,
}

impl Submit3D {
    pub const SIZE: usize = 32;

    pub fn new(ctx_id: u32, size: u32) -> Self {
        Self {
            header: CtrlHeader::new(CMD_SUBMIT_3D).with_context(ctx_id),
            size,
            padding: 0,
        }
    }

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..24].copy_from_slice(&self.header.encode_le());
        out[24..28].copy_from_slice(&self.size.to_le_bytes());
        out[28..32].copy_from_slice(&self.padding.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            header: CtrlHeader::decode_le(&bytes[0..24])?,
            size: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            padding: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
        })
    }
}

/// A complete SUBMIT_3D request together with its command stream.
///
/// This is still transport-independent: the actual VirtQueue descriptor
/// construction comes later.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Submit3DCommand {
    pub request: Submit3D,
    pub command_stream: Vec<u8>,
}

impl Submit3DCommand {
    pub fn new(ctx_id: u32, command_stream: Vec<u8>) -> Self {
        assert!(
            command_stream.len() <= u32::MAX as usize,
            "command stream is too large"
        );

        let request = Submit3D::new(ctx_id, command_stream.len() as u32);

        Self {
            request,
            command_stream,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(Submit3D::SIZE + self.command_stream.len());

        out.extend_from_slice(&self.request.encode_le());
        out.extend_from_slice(&self.command_stream);

        out
    }

    pub fn command_stream_size(&self) -> usize {
        self.command_stream.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submit_size_matches_protocol() {
        assert_eq!(Submit3D::SIZE, 32);
    }

    #[test]
    fn submit_round_trip() {
        let request = Submit3D::new(42, 128);

        let bytes = request.encode_le();
        let decoded = Submit3D::decode_le(&bytes).unwrap();

        assert_eq!(decoded, request);
        assert_eq!(decoded.header.typ, CMD_SUBMIT_3D);
        assert_eq!(decoded.header.ctx_id, 42);
        assert_eq!(decoded.size, 128);
    }

    #[test]
    fn submit_rejects_short_input() {
        assert_eq!(Submit3D::decode_le(&[0u8; 31]), None);
    }

    #[test]
    fn submit_command_stream_size_is_recorded() {
        let stream = vec![0x11, 0x22, 0x33, 0x44];

        let request = Submit3DCommand::new(7, stream.clone());

        assert_eq!(request.request.header.ctx_id, 7);
        assert_eq!(request.request.header.typ, CMD_SUBMIT_3D);
        assert_eq!(request.request.size, 4);
        assert_eq!(request.command_stream_size(), 4);
        assert_eq!(&request.encode()[32..], &stream);
    }

    #[test]
    fn empty_command_stream_is_supported() {
        let request = Submit3DCommand::new(1, Vec::new());

        assert_eq!(request.request.size, 0);
        assert!(request.command_stream.is_empty());
        assert_eq!(request.encode().len(), Submit3D::SIZE);
    }
}
