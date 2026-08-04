use crate::virtio_gpu::protocol::{
    CMD_CTX_ATTACH_RESOURCE, CMD_CTX_CREATE, CMD_CTX_DESTROY, CMD_CTX_DETACH_RESOURCE, CtrlHeader,
};

/// Context initialization flags.
///
/// The low 8 bits select the capability set.
pub const CONTEXT_INIT_CAPSET_ID_MASK: u32 = 0x0000_00FF;

/// Context creation request.
///
/// Wire layout:
/// - CtrlHeader:       24 bytes
/// - nlen:              4 bytes
/// - context_init:      4 bytes
/// - debug_name:       64 bytes
///
/// Total: 96 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextCreate {
    pub header: CtrlHeader,
    pub nlen: u32,
    pub context_init: u32,
    pub debug_name: [u8; 64],
}

impl ContextCreate {
    pub const SIZE: usize = 96;

    pub fn new(ctx_id: u32, context_init: u32, name: &[u8]) -> Self {
        let mut debug_name = [0u8; 64];

        let len = name.len().min(debug_name.len());
        debug_name[..len].copy_from_slice(&name[..len]);

        Self {
            header: CtrlHeader::new(CMD_CTX_CREATE).with_context(ctx_id),
            nlen: len as u32,
            context_init,
            debug_name,
        }
    }

    pub fn venus(ctx_id: u32, name: &[u8]) -> Self {
        Self::new(ctx_id, crate::virtio_gpu::protocol::CAPSET_VENUS, name)
    }

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..24].copy_from_slice(&self.header.encode_le());
        out[24..28].copy_from_slice(&self.nlen.to_le_bytes());
        out[28..32].copy_from_slice(&self.context_init.to_le_bytes());
        out[32..96].copy_from_slice(&self.debug_name);

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            header: CtrlHeader::decode_le(&bytes[0..24])?,
            nlen: u32::from_le_bytes(bytes[24..28].try_into().ok()?),
            context_init: u32::from_le_bytes(bytes[28..32].try_into().ok()?),
            debug_name: bytes[32..96].try_into().ok()?,
        })
    }

    pub fn capset_id(self) -> u32 {
        self.context_init & CONTEXT_INIT_CAPSET_ID_MASK
    }

    pub fn is_venus(self) -> bool {
        self.capset_id() == crate::virtio_gpu::protocol::CAPSET_VENUS
    }
}

/// CTX_DESTROY request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextDestroy {
    pub header: CtrlHeader,
}

impl ContextDestroy {
    pub const SIZE: usize = 24;

    pub fn new(ctx_id: u32) -> Self {
        Self {
            header: CtrlHeader::new(CMD_CTX_DESTROY).with_context(ctx_id),
        }
    }

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        self.header.encode_le()
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        Some(Self {
            header: CtrlHeader::decode_le(bytes)?,
        })
    }
}

/// CTX_ATTACH_RESOURCE request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextAttachResource {
    pub header: CtrlHeader,
    pub resource_id: u32,
    pub padding: u32,
}

impl ContextAttachResource {
    pub const SIZE: usize = 32;

    pub fn new(ctx_id: u32, resource_id: u32) -> Self {
        Self {
            header: CtrlHeader::new(CMD_CTX_ATTACH_RESOURCE).with_context(ctx_id),
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

/// CTX_DETACH_RESOURCE request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextDetachResource {
    pub header: CtrlHeader,
    pub resource_id: u32,
    pub padding: u32,
}

impl ContextDetachResource {
    pub const SIZE: usize = 32;

    pub fn new(ctx_id: u32, resource_id: u32) -> Self {
        Self {
            header: CtrlHeader::new(CMD_CTX_DETACH_RESOURCE).with_context(ctx_id),
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
    use crate::virtio_gpu::protocol::CAPSET_VENUS;

    #[test]
    fn context_create_size_matches_protocol() {
        assert_eq!(ContextCreate::SIZE, 96);
    }

    #[test]
    fn venus_context_create_round_trip() {
        let request = ContextCreate::venus(42, b"venus");

        let bytes = request.encode_le();
        let decoded = ContextCreate::decode_le(&bytes).unwrap();

        assert_eq!(decoded, request);
        assert_eq!(decoded.header.typ, CMD_CTX_CREATE);
        assert_eq!(decoded.header.ctx_id, 42);
        assert_eq!(decoded.context_init, CAPSET_VENUS);
        assert_eq!(decoded.capset_id(), CAPSET_VENUS);
        assert!(decoded.is_venus());
        assert_eq!(decoded.nlen, 5);
    }

    #[test]
    fn context_create_truncates_long_name() {
        let name = [b'A'; 100];

        let request = ContextCreate::new(1, CAPSET_VENUS, &name);

        assert_eq!(request.nlen, 64);
        assert!(request.debug_name.iter().all(|&byte| byte == b'A'));
    }

    #[test]
    fn context_create_rejects_short_input() {
        assert_eq!(ContextCreate::decode_le(&[0u8; 95]), None);
    }

    #[test]
    fn context_destroy_round_trip() {
        let request = ContextDestroy::new(42);

        let bytes = request.encode_le();
        let decoded = ContextDestroy::decode_le(&bytes).unwrap();

        assert_eq!(decoded, request);
        assert_eq!(decoded.header.typ, CMD_CTX_DESTROY);
        assert_eq!(decoded.header.ctx_id, 42);
    }

    #[test]
    fn context_attach_resource_round_trip() {
        let request = ContextAttachResource::new(42, 100);

        let bytes = request.encode_le();
        let decoded = ContextAttachResource::decode_le(&bytes).unwrap();

        assert_eq!(decoded, request);
        assert_eq!(decoded.header.typ, CMD_CTX_ATTACH_RESOURCE);
        assert_eq!(decoded.header.ctx_id, 42);
        assert_eq!(decoded.resource_id, 100);
    }

    #[test]
    fn context_detach_resource_round_trip() {
        let request = ContextDetachResource::new(42, 100);

        let bytes = request.encode_le();
        let decoded = ContextDetachResource::decode_le(&bytes).unwrap();

        assert_eq!(decoded, request);
        assert_eq!(decoded.header.typ, CMD_CTX_DETACH_RESOURCE);
        assert_eq!(decoded.header.ctx_id, 42);
        assert_eq!(decoded.resource_id, 100);
    }

    #[test]
    fn context_headers_use_correct_command_ids() {
        assert_eq!(ContextDestroy::new(1).header.typ, CMD_CTX_DESTROY);
        assert_eq!(
            ContextAttachResource::new(1, 2).header.typ,
            CMD_CTX_ATTACH_RESOURCE
        );
        assert_eq!(
            ContextDetachResource::new(1, 2).header.typ,
            CMD_CTX_DETACH_RESOURCE
        );
    }
}
