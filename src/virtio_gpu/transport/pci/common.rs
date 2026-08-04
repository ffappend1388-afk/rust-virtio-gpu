//! VirtIO PCI common configuration space.
//!
//! Layout follows the modern VirtIO PCI transport specification.
//!
//! `CommonConfig` stores the mutable PCI common configuration state.
//! Device/driver feature bitmaps themselves remain owned by
//! `VirtioGpuDevice`; this structure only stores the selector registers.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct CommonConfig {
    /// Selects the 32-bit half of the device feature bitmap.
    pub device_feature_select: u32,

    /// Selects the 32-bit half of the driver feature bitmap.
    pub driver_feature_select: u32,

    /// MSI-X vector for configuration interrupts.
    pub config_msix_vector: u16,

    /// Number of available virtqueues.
    pub num_queues: u16,

    /// VirtIO device status.
    pub device_status: u8,

    /// Configuration generation counter.
    pub config_generation: u8,

    /// Currently selected virtqueue.
    pub queue_select: u16,

    /// Size of the currently selected virtqueue.
    pub queue_size: u16,

    /// MSI-X vector assigned to the currently selected queue.
    pub queue_msix_vector: u16,

    /// Enables/disables the currently selected queue.
    pub queue_enable: u16,

    /// Queue notification offset.
    pub queue_notify_off: u16,

    /// Descriptor table physical address.
    pub queue_desc: u64,

    /// Driver area physical address.
    pub queue_driver: u64,

    /// Device area physical address.
    pub queue_device: u64,

    /// Queue notification data.
    pub queue_notify_data: u16,

    /// Queue reset control.
    pub queue_reset: u16,
}

impl CommonConfig {
    /// Sentinel value meaning that no MSI-X vector is assigned.
    pub const MSIX_VECTOR_NONE: u16 = u16::MAX;

    /*
     * virtio_pci_common_cfg offsets.
     */

    pub const DEVICE_FEATURE_SELECT: u64 = 0x00;
    pub const DEVICE_FEATURE: u64 = 0x04;

    pub const DRIVER_FEATURE_SELECT: u64 = 0x08;
    pub const DRIVER_FEATURE: u64 = 0x0c;

    pub const CONFIG_MSIX_VECTOR: u64 = 0x10;
    pub const NUM_QUEUES: u64 = 0x12;

    pub const DEVICE_STATUS: u64 = 0x14;
    pub const CONFIG_GENERATION: u64 = 0x15;

    pub const QUEUE_SELECT: u64 = 0x16;
    pub const QUEUE_SIZE: u64 = 0x18;
    pub const QUEUE_MSIX_VECTOR: u64 = 0x1a;
    pub const QUEUE_ENABLE: u64 = 0x1c;
    pub const QUEUE_NOTIFY_OFF: u64 = 0x1e;

    pub const QUEUE_DESC: u64 = 0x20;
    pub const QUEUE_DRIVER: u64 = 0x28;
    pub const QUEUE_DEVICE: u64 = 0x30;

    pub const QUEUE_NOTIFY_DATA: u64 = 0x38;
    pub const QUEUE_RESET: u64 = 0x3a;

    pub const SIZE: u64 = 0x3c;

    pub const fn new(num_queues: u16, config_generation: u8) -> Self {
        Self {
            device_feature_select: 0,
            driver_feature_select: 0,

            config_msix_vector: Self::MSIX_VECTOR_NONE,
            num_queues,

            device_status: 0,
            config_generation,

            queue_select: 0,
            queue_size: 0,
            queue_msix_vector: Self::MSIX_VECTOR_NONE,
            queue_enable: 0,
            queue_notify_off: 0,

            queue_desc: 0,
            queue_driver: 0,
            queue_device: 0,

            queue_notify_data: 0,
            queue_reset: 0,
        }
    }

    /// Reset the currently selected queue.
    pub fn reset_queue(&mut self) {
        self.queue_size = 0;
        self.queue_msix_vector = Self::MSIX_VECTOR_NONE;
        self.queue_enable = 0;
        self.queue_notify_off = 0;

        self.queue_desc = 0;
        self.queue_driver = 0;
        self.queue_device = 0;

        self.queue_notify_data = 0;
        self.queue_reset = 0;
    }

    // ---------------------------------------------------------------------
    // Read access
    // ---------------------------------------------------------------------

    /// Read a 32-bit common configuration register.
    ///
    /// Feature bitmap registers are intentionally not handled here because
    /// the actual device feature bitmap belongs to `VirtioGpuDevice`.
    pub fn read_u32(&self, offset: u64) -> Option<u32> {
        match offset {
            Self::DEVICE_FEATURE_SELECT => Some(self.device_feature_select),
            Self::DRIVER_FEATURE_SELECT => Some(self.driver_feature_select),
            _ => None,
        }
    }

    /// Read a 16-bit common configuration register.
    pub fn read_u16(&self, offset: u64) -> Option<u16> {
        match offset {
            Self::CONFIG_MSIX_VECTOR => Some(self.config_msix_vector),
            Self::NUM_QUEUES => Some(self.num_queues),

            Self::QUEUE_SELECT => Some(self.queue_select),
            Self::QUEUE_SIZE => Some(self.queue_size),
            Self::QUEUE_MSIX_VECTOR => Some(self.queue_msix_vector),
            Self::QUEUE_ENABLE => Some(self.queue_enable),
            Self::QUEUE_NOTIFY_OFF => Some(self.queue_notify_off),

            Self::QUEUE_NOTIFY_DATA => Some(self.queue_notify_data),
            Self::QUEUE_RESET => Some(self.queue_reset),

            _ => None,
        }
    }

    /// Read an 8-bit common configuration register.
    pub fn read_u8(&self, offset: u64) -> Option<u8> {
        match offset {
            Self::DEVICE_STATUS => Some(self.device_status),
            Self::CONFIG_GENERATION => Some(self.config_generation),
            _ => None,
        }
    }

    /// Read a 64-bit common configuration register.
    pub fn read_u64(&self, offset: u64) -> Option<u64> {
        match offset {
            Self::QUEUE_DESC => Some(self.queue_desc),
            Self::QUEUE_DRIVER => Some(self.queue_driver),
            Self::QUEUE_DEVICE => Some(self.queue_device),
            _ => None,
        }
    }

    // ---------------------------------------------------------------------
    // Write access
    // ---------------------------------------------------------------------

    /// Write a 32-bit common configuration register.
    ///
    /// The actual feature bitmaps are handled by `VirtioGpuDevice`.
    pub fn write_u32(&mut self, offset: u64, value: u32) -> bool {
        match offset {
            Self::DEVICE_FEATURE_SELECT => {
                self.device_feature_select = value;
                true
            }

            Self::DRIVER_FEATURE_SELECT => {
                self.driver_feature_select = value;
                true
            }

            _ => false,
        }
    }

    /// Write a 16-bit common configuration register.
    pub fn write_u16(&mut self, offset: u64, value: u16) -> bool {
        match offset {
            Self::CONFIG_MSIX_VECTOR => {
                self.config_msix_vector = value;
                true
            }

            Self::QUEUE_SELECT => {
                self.queue_select = value;
                true
            }

            Self::QUEUE_SIZE => {
                self.queue_size = value;
                true
            }

            Self::QUEUE_MSIX_VECTOR => {
                self.queue_msix_vector = value;
                true
            }

            Self::QUEUE_ENABLE => {
                self.queue_enable = value;
                true
            }

            Self::QUEUE_RESET => {
                if value != 1 {
                    return false;
                }
                self.reset_queue();
                true
            }

            // Read-only according to the VirtIO common configuration layout.
            Self::NUM_QUEUES | Self::QUEUE_NOTIFY_OFF | Self::QUEUE_NOTIFY_DATA => false,

            _ => false,
        }
    }

    /// Write an 8-bit common configuration register.
    pub fn write_u8(&mut self, offset: u64, value: u8) -> bool {
        match offset {
            Self::DEVICE_STATUS => {
                self.device_status = value;
                true
            }

            // Configuration generation is read-only.
            Self::CONFIG_GENERATION => false,

            _ => false,
        }
    }

    /// Write a 64-bit common configuration register.
    pub fn write_u64(&mut self, offset: u64, value: u64) -> bool {
        match offset {
            Self::QUEUE_DESC => {
                self.queue_desc = value;
                true
            }

            Self::QUEUE_DRIVER => {
                self.queue_driver = value;
                true
            }

            Self::QUEUE_DEVICE => {
                self.queue_device = value;
                true
            }

            _ => false,
        }
    }
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self::new(2, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_config_offsets_match_virtio() {
        assert_eq!(CommonConfig::DEVICE_FEATURE_SELECT, 0x00);
        assert_eq!(CommonConfig::DEVICE_FEATURE, 0x04);

        assert_eq!(CommonConfig::DRIVER_FEATURE_SELECT, 0x08);
        assert_eq!(CommonConfig::DRIVER_FEATURE, 0x0c);

        assert_eq!(CommonConfig::CONFIG_MSIX_VECTOR, 0x10);
        assert_eq!(CommonConfig::NUM_QUEUES, 0x12);

        assert_eq!(CommonConfig::DEVICE_STATUS, 0x14);
        assert_eq!(CommonConfig::CONFIG_GENERATION, 0x15);

        assert_eq!(CommonConfig::QUEUE_SELECT, 0x16);
        assert_eq!(CommonConfig::QUEUE_SIZE, 0x18);
        assert_eq!(CommonConfig::QUEUE_MSIX_VECTOR, 0x1a);
        assert_eq!(CommonConfig::QUEUE_ENABLE, 0x1c);
        assert_eq!(CommonConfig::QUEUE_NOTIFY_OFF, 0x1e);

        assert_eq!(CommonConfig::QUEUE_DESC, 0x20);
        assert_eq!(CommonConfig::QUEUE_DRIVER, 0x28);
        assert_eq!(CommonConfig::QUEUE_DEVICE, 0x30);

        assert_eq!(CommonConfig::QUEUE_NOTIFY_DATA, 0x38);
        assert_eq!(CommonConfig::QUEUE_RESET, 0x3a);

        assert_eq!(CommonConfig::SIZE, 0x3c);
    }

    #[test]
    fn common_config_starts_in_reset_state() {
        let config = CommonConfig::new(2, 7);

        assert_eq!(config.device_feature_select, 0);
        assert_eq!(config.driver_feature_select, 0);

        assert_eq!(config.num_queues, 2);
        assert_eq!(config.config_generation, 7);

        assert_eq!(config.device_status, 0);
        assert_eq!(config.queue_select, 0);
        assert_eq!(config.queue_size, 0);
        assert_eq!(config.queue_enable, 0);
    }

    #[test]
    fn queue_reset_clears_queue_state() {
        let mut config = CommonConfig {
            queue_size: 256,
            queue_enable: 1,
            queue_desc: 0x1000,
            queue_driver: 0x2000,
            queue_device: 0x3000,
            ..Default::default()
        };

        config.reset_queue();

        assert_eq!(config.queue_size, 0);
        assert_eq!(config.queue_enable, 0);
        assert_eq!(config.queue_desc, 0);
        assert_eq!(config.queue_driver, 0);
        assert_eq!(config.queue_device, 0);
    }

    #[test]
    fn common_config_reads_registers() {
        let mut config = CommonConfig::new(2, 7);

        config.device_feature_select = 1;
        config.driver_feature_select = 2;

        config.queue_select = 1;
        config.queue_size = 256;

        config.queue_desc = 0x1000;
        config.queue_driver = 0x2000;
        config.queue_device = 0x3000;

        assert_eq!(
            config.read_u32(CommonConfig::DEVICE_FEATURE_SELECT),
            Some(1)
        );

        assert_eq!(
            config.read_u32(CommonConfig::DRIVER_FEATURE_SELECT),
            Some(2)
        );

        assert_eq!(config.read_u16(CommonConfig::QUEUE_SELECT), Some(1));

        assert_eq!(config.read_u16(CommonConfig::QUEUE_SIZE), Some(256));

        assert_eq!(config.read_u64(CommonConfig::QUEUE_DESC), Some(0x1000));

        assert_eq!(config.read_u64(CommonConfig::QUEUE_DRIVER), Some(0x2000));

        assert_eq!(config.read_u64(CommonConfig::QUEUE_DEVICE), Some(0x3000));
    }

    #[test]
    fn common_config_writes_registers() {
        let mut config = CommonConfig::new(2, 7);

        assert!(config.write_u32(CommonConfig::DEVICE_FEATURE_SELECT, 1));
        assert!(config.write_u32(CommonConfig::DRIVER_FEATURE_SELECT, 1));

        assert!(config.write_u16(CommonConfig::QUEUE_SELECT, 1));
        assert!(config.write_u16(CommonConfig::QUEUE_SIZE, 256));
        assert!(config.write_u16(CommonConfig::QUEUE_ENABLE, 1));

        assert!(config.write_u64(CommonConfig::QUEUE_DESC, 0x1000));
        assert!(config.write_u64(CommonConfig::QUEUE_DRIVER, 0x2000));
        assert!(config.write_u64(CommonConfig::QUEUE_DEVICE, 0x3000));

        assert_eq!(config.queue_select, 1);
        assert_eq!(config.queue_size, 256);
        assert_eq!(config.queue_enable, 1);

        assert_eq!(config.queue_desc, 0x1000);
        assert_eq!(config.queue_driver, 0x2000);
        assert_eq!(config.queue_device, 0x3000);
    }

    #[test]
    fn read_only_registers_reject_writes() {
        let mut config = CommonConfig::new(2, 7);

        assert!(!config.write_u16(CommonConfig::NUM_QUEUES, 99));
        assert!(!config.write_u16(CommonConfig::QUEUE_NOTIFY_OFF, 99));
        assert!(!config.write_u16(CommonConfig::QUEUE_NOTIFY_DATA, 99));

        assert!(!config.write_u8(CommonConfig::CONFIG_GENERATION, 99));

        assert_eq!(config.num_queues, 2);
        assert_eq!(config.config_generation, 7);
    }

    #[test]
    fn queue_reset_register_resets_selected_queue() {
        let mut config = CommonConfig {
            queue_size: 256,
            queue_enable: 1,
            queue_desc: 0x1000,
            queue_driver: 0x2000,
            queue_device: 0x3000,
            ..Default::default()
        };

        assert!(config.write_u16(CommonConfig::QUEUE_RESET, 1));

        assert_eq!(config.queue_size, 0);
        assert_eq!(config.queue_enable, 0);
        assert_eq!(config.queue_desc, 0);
        assert_eq!(config.queue_driver, 0);
        assert_eq!(config.queue_device, 0);
    }
}
