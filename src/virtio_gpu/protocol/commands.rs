/// VirtIO-GPU device type.
pub const VIRTIO_ID_GPU: u32 = 16;

/// VirtIO-GPU control queue.
pub const CONTROLQ: u16 = 0;

/// VirtIO-GPU cursor queue.
pub const CURSORQ: u16 = 1;

// -----------------------------------------------------------------------------
// 2D / general control commands
// -----------------------------------------------------------------------------

pub const CMD_GET_DISPLAY_INFO: u32 = 0x0100;
pub const CMD_RESOURCE_CREATE_2D: u32 = 0x0101;
pub const CMD_RESOURCE_UNREF: u32 = 0x0102;
pub const CMD_SET_SCANOUT: u32 = 0x0103;
pub const CMD_RESOURCE_FLUSH: u32 = 0x0104;
pub const CMD_TRANSFER_TO_HOST_2D: u32 = 0x0105;
pub const CMD_RESOURCE_ATTACH_BACKING: u32 = 0x0106;
pub const CMD_RESOURCE_DETACH_BACKING: u32 = 0x0107;
pub const CMD_GET_CAPSET_INFO: u32 = 0x0108;
pub const CMD_GET_CAPSET: u32 = 0x0109;
pub const CMD_GET_EDID: u32 = 0x010A;
pub const CMD_RESOURCE_ASSIGN_UUID: u32 = 0x010B;
pub const CMD_RESOURCE_CREATE_BLOB: u32 = 0x010C;
pub const CMD_SET_SCANOUT_BLOB: u32 = 0x010D;

// -----------------------------------------------------------------------------
// 3D / context commands
// -----------------------------------------------------------------------------

pub const CMD_CTX_CREATE: u32 = 0x0200;
pub const CMD_CTX_DESTROY: u32 = 0x0201;
pub const CMD_CTX_ATTACH_RESOURCE: u32 = 0x0202;
pub const CMD_CTX_DETACH_RESOURCE: u32 = 0x0203;
pub const CMD_RESOURCE_CREATE_3D: u32 = 0x0204;
pub const CMD_TRANSFER_TO_HOST_3D: u32 = 0x0205;
pub const CMD_TRANSFER_FROM_HOST_3D: u32 = 0x0206;
pub const CMD_SUBMIT_3D: u32 = 0x0207;
pub const CMD_RESOURCE_MAP_BLOB: u32 = 0x0208;
pub const CMD_RESOURCE_UNMAP_BLOB: u32 = 0x0209;

// -----------------------------------------------------------------------------
// Cursor commands
// -----------------------------------------------------------------------------

pub const CMD_UPDATE_CURSOR: u32 = 0x0300;
pub const CMD_MOVE_CURSOR: u32 = 0x0301;

// -----------------------------------------------------------------------------
// Successful responses
// -----------------------------------------------------------------------------

pub const RESP_OK_NODATA: u32 = 0x1100;
pub const RESP_OK_DISPLAY_INFO: u32 = 0x1101;
pub const RESP_OK_CAPSET_INFO: u32 = 0x1102;
pub const RESP_OK_CAPSET: u32 = 0x1103;
pub const RESP_OK_EDID: u32 = 0x1104;
pub const RESP_OK_RESOURCE_UUID: u32 = 0x1105;
pub const RESP_OK_MAP_INFO: u32 = 0x1106;

// -----------------------------------------------------------------------------
// Error responses
// -----------------------------------------------------------------------------

pub const RESP_ERR_UNSPEC: u32 = 0x1200;
pub const RESP_ERR_OUT_OF_MEMORY: u32 = 0x1201;
pub const RESP_ERR_INVALID_SCANOUT_ID: u32 = 0x1202;
pub const RESP_ERR_INVALID_RESOURCE_ID: u32 = 0x1203;
pub const RESP_ERR_INVALID_CONTEXT_ID: u32 = 0x1204;
pub const RESP_ERR_INVALID_PARAMETER: u32 = 0x1205;

// -----------------------------------------------------------------------------
// VirtIO-GPU flags
// -----------------------------------------------------------------------------

/// Request a fence for this command.
pub const FLAG_FENCE: u32 = 1 << 0;

/// Select the command ring for the fence.
pub const FLAG_INFO_RING_IDX: u32 = 1 << 1;

// -----------------------------------------------------------------------------
// Capability set IDs
// -----------------------------------------------------------------------------

pub const CAPSET_VIRGL: u32 = 1;
pub const CAPSET_VIRGL2: u32 = 2;
pub const CAPSET_GFXSTREAM_VULKAN: u32 = 3;

/// Venus Vulkan capability set.
pub const CAPSET_VENUS: u32 = 4;

pub const CAPSET_CROSS_DOMAIN: u32 = 5;
pub const CAPSET_DRM: u32 = 6;

// -----------------------------------------------------------------------------
// Context initialization
// -----------------------------------------------------------------------------

/// Low 8 bits select the capability set used by the context.
pub const CONTEXT_INIT_CAPSET_ID_MASK: u32 = 0x0000_00FF;

// -----------------------------------------------------------------------------
// Resource flags
// -----------------------------------------------------------------------------

pub const RESOURCE_FLAG_Y_0_TOP: u32 = 1 << 0;

// -----------------------------------------------------------------------------
// Blob memory types
// -----------------------------------------------------------------------------

/// Guest-backed blob.
pub const BLOB_MEM_GUEST: u32 = 0x0001;

/// Host 3D-backed blob.
pub const BLOB_MEM_HOST3D: u32 = 0x0002;

/// Host 3D + guest-backed blob.
pub const BLOB_MEM_HOST3D_GUEST: u32 = 0x0003;

// -----------------------------------------------------------------------------
// Blob flags
// -----------------------------------------------------------------------------

/// Blob can be mapped through host-visible memory.
pub const BLOB_FLAG_USE_MAPPABLE: u32 = 0x0001;

/// Blob can be shared with another device.
pub const BLOB_FLAG_USE_SHAREABLE: u32 = 0x0002;

/// Blob can be shared across devices.
pub const BLOB_FLAG_USE_CROSS_DEVICE: u32 = 0x0004;

// -----------------------------------------------------------------------------
// Shared memory regions
// -----------------------------------------------------------------------------

pub const SHM_ID_UNDEFINED: u32 = 0;
pub const SHM_ID_HOST_VISIBLE: u32 = 1;

// -----------------------------------------------------------------------------
// Blob map cache modes
// -----------------------------------------------------------------------------

pub const MAP_CACHE_MASK: u32 = 0x0F;
pub const MAP_CACHE_NONE: u32 = 0x00;
pub const MAP_CACHE_CACHED: u32 = 0x01;
pub const MAP_CACHE_UNCACHED: u32 = 0x02;
pub const MAP_CACHE_WC: u32 = 0x03;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_ids_are_unique() {
        let ids = [
            CMD_GET_DISPLAY_INFO,
            CMD_RESOURCE_CREATE_2D,
            CMD_RESOURCE_UNREF,
            CMD_SET_SCANOUT,
            CMD_RESOURCE_FLUSH,
            CMD_TRANSFER_TO_HOST_2D,
            CMD_RESOURCE_ATTACH_BACKING,
            CMD_RESOURCE_DETACH_BACKING,
            CMD_GET_CAPSET_INFO,
            CMD_GET_CAPSET,
            CMD_GET_EDID,
            CMD_RESOURCE_ASSIGN_UUID,
            CMD_RESOURCE_CREATE_BLOB,
            CMD_SET_SCANOUT_BLOB,
            CMD_CTX_CREATE,
            CMD_CTX_DESTROY,
            CMD_CTX_ATTACH_RESOURCE,
            CMD_CTX_DETACH_RESOURCE,
            CMD_RESOURCE_CREATE_3D,
            CMD_TRANSFER_TO_HOST_3D,
            CMD_TRANSFER_FROM_HOST_3D,
            CMD_SUBMIT_3D,
            CMD_RESOURCE_MAP_BLOB,
            CMD_RESOURCE_UNMAP_BLOB,
            CMD_UPDATE_CURSOR,
            CMD_MOVE_CURSOR,
        ];

        for (i, &id) in ids.iter().enumerate() {
            assert!(
                !ids[..i].contains(&id),
                "duplicate VirtIO-GPU command ID: {id:#x}"
            );
        }
    }
    #[test]
    fn venus_capset_id_is_correct() {
        assert_eq!(CAPSET_VENUS, 4);
    }

    #[test]
    fn blob_memory_values_match_protocol() {
        assert_eq!(BLOB_MEM_GUEST, 0x0001);
        assert_eq!(BLOB_MEM_HOST3D, 0x0002);
        assert_eq!(BLOB_MEM_HOST3D_GUEST, 0x0003);
    }

    #[test]
    fn blob_flags_match_protocol() {
        assert_eq!(BLOB_FLAG_USE_MAPPABLE, 0x0001);
        assert_eq!(BLOB_FLAG_USE_SHAREABLE, 0x0002);
        assert_eq!(BLOB_FLAG_USE_CROSS_DEVICE, 0x0004);
    }

    #[test]
    fn fence_flags_match_protocol() {
        assert_eq!(FLAG_FENCE, 1 << 0);
        assert_eq!(FLAG_INFO_RING_IDX, 1 << 1);
    }

    #[test]
    fn host_visible_shared_memory_id_is_correct() {
        assert_eq!(SHM_ID_UNDEFINED, 0);
        assert_eq!(SHM_ID_HOST_VISIBLE, 1);
    }

    #[test]
    fn venus_context_init_value_is_valid() {
        assert_eq!(CAPSET_VENUS & CONTEXT_INIT_CAPSET_ID_MASK, CAPSET_VENUS);
    }
}
