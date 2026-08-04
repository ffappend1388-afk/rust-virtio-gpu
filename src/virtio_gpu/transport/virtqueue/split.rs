use std::sync::atomic::{Ordering, fence};

use crate::virtio_gpu::transport::memory::{GuestAddress, GuestMemory, GuestMemoryError};

pub const DESC_F_NEXT: u16 = 1 << 0;
pub const DESC_F_WRITE: u16 = 1 << 1;
pub const DESC_F_INDIRECT: u16 = 1 << 2;

pub const AVAIL_F_NO_INTERRUPT: u16 = 1;
pub const USED_F_NO_NOTIFY: u16 = 1;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Descriptor {
    pub addr: GuestAddress,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

impl Descriptor {
    pub const SIZE: usize = 16;

    pub fn encode_le(self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];

        out[0..8].copy_from_slice(&self.addr.0.to_le_bytes());
        out[8..12].copy_from_slice(&self.len.to_le_bytes());
        out[12..14].copy_from_slice(&self.flags.to_le_bytes());
        out[14..16].copy_from_slice(&self.next.to_le_bytes());

        out
    }

    pub fn decode_le(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE {
            return None;
        }

        Some(Self {
            addr: GuestAddress(u64::from_le_bytes(bytes[0..8].try_into().ok()?)),
            len: u32::from_le_bytes(bytes[8..12].try_into().ok()?),
            flags: u16::from_le_bytes(bytes[12..14].try_into().ok()?),
            next: u16::from_le_bytes(bytes[14..16].try_into().ok()?),
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UsedElement {
    pub id: u32,
    pub len: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VirtQueueError {
    InvalidQueueSize,
    QueueFull,
    DescriptorUnavailable,
    InvalidDescriptor,
    InvalidMemory,
    GuestMemory(GuestMemoryError),
}

impl From<GuestMemoryError> for VirtQueueError {
    fn from(value: GuestMemoryError) -> Self {
        Self::GuestMemory(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitVirtQueueLayout {
    pub queue_size: u16,
    pub descriptor_offset: usize,
    pub available_offset: usize,
    pub used_offset: usize,
    pub total_size: usize,
}

impl SplitVirtQueueLayout {
    pub fn new(queue_size: u16) -> Result<Self, VirtQueueError> {
        if queue_size == 0 || !queue_size.is_power_of_two() {
            return Err(VirtQueueError::InvalidQueueSize);
        }

        if queue_size > 32768 {
            return Err(VirtQueueError::InvalidQueueSize);
        }

        let descriptor_offset = 0;

        let descriptor_size = 16usize
            .checked_mul(queue_size as usize)
            .ok_or(VirtQueueError::InvalidQueueSize)?;

        let available_offset = align_up(descriptor_offset + descriptor_size, 2);

        let available_size = 6usize
            .checked_add(2 * queue_size as usize)
            .ok_or(VirtQueueError::InvalidQueueSize)?;

        let used_offset = align_up(available_offset + available_size, 4);

        let used_size = 6usize
            .checked_add(8 * queue_size as usize)
            .ok_or(VirtQueueError::InvalidQueueSize)?;

        let total_size = used_offset
            .checked_add(used_size)
            .ok_or(VirtQueueError::InvalidQueueSize)?;

        Ok(Self {
            queue_size,
            descriptor_offset,
            available_offset,
            used_offset,
            total_size,
        })
    }

    pub fn descriptor_address(
        self,
        base: GuestAddress,
        index: u16,
    ) -> Result<GuestAddress, VirtQueueError> {
        if index >= self.queue_size {
            return Err(VirtQueueError::InvalidDescriptor);
        }

        Ok(base.offset(self.descriptor_offset + 16 * index as usize))
    }

    pub fn available_address(self, base: GuestAddress) -> GuestAddress {
        base.offset(self.available_offset)
    }

    pub fn used_address(self, base: GuestAddress) -> GuestAddress {
        base.offset(self.used_offset)
    }
}

fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

pub struct SplitVirtQueue {
    memory: GuestMemory,
    base: GuestAddress,
    layout: SplitVirtQueueLayout,

    free: Vec<bool>,

    next_avail: u16,
    last_used: u16,
}

impl SplitVirtQueue {
    pub fn new(
        memory: GuestMemory,
        base: GuestAddress,
        queue_size: u16,
    ) -> Result<Self, VirtQueueError> {
        let layout = SplitVirtQueueLayout::new(queue_size)?;

        let mut queue = Self {
            memory,
            base,
            layout,
            free: vec![true; queue_size as usize],
            next_avail: 0,
            last_used: 0,
        };

        queue.initialize()?;

        Ok(queue)
    }

    pub fn queue_size(&self) -> u16 {
        self.layout.queue_size
    }

    pub fn layout(&self) -> SplitVirtQueueLayout {
        self.layout
    }

    pub fn memory(&self) -> &GuestMemory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut GuestMemory {
        &mut self.memory
    }

    pub fn base(&self) -> GuestAddress {
        self.base
    }

    fn initialize(&mut self) -> Result<(), VirtQueueError> {
        self.memory
            .write(self.base, &vec![0u8; self.layout.total_size])?;

        Ok(())
    }

    pub fn read_chain(&self, chain: &DescriptorChain) -> Result<Vec<u8>, VirtQueueError> {
        let mut data = Vec::new();

        let mut index = chain.head;

        loop {
            let desc = self.descriptor(index)?;

            if desc.len > 0 {
                let mut bytes = vec![0u8; desc.len as usize];

                self.memory.read(desc.addr, &mut bytes)?;

                data.extend_from_slice(&bytes);
            }

            if desc.flags & DESC_F_NEXT == 0 {
                break;
            }

            index = desc.next;
        }

        Ok(data)
    }

    pub fn descriptor(&self, index: u16) -> Result<Descriptor, VirtQueueError> {
        let addr = self.layout.descriptor_address(self.base, index)?;

        let mut bytes = [0u8; Descriptor::SIZE];

        self.memory.read(addr, &mut bytes)?;

        Descriptor::decode_le(&bytes).ok_or(VirtQueueError::InvalidDescriptor)
    }

    pub fn descriptor_chain(&self, head: u16) -> Result<Vec<(u16, Descriptor)>, VirtQueueError> {
        if head >= self.queue_size() {
            return Err(VirtQueueError::InvalidDescriptor);
        }

        let mut chain = Vec::new();
        let mut current = head;

        for _ in 0..self.queue_size() {
            let descriptor = self.descriptor(current)?;
            chain.push((current, descriptor));

            if descriptor.flags & DESC_F_NEXT == 0 {
                return Ok(chain);
            }

            let next = descriptor.next;

            if next >= self.queue_size() {
                return Err(VirtQueueError::InvalidDescriptor);
            }

            current = next;
        }

        // We followed queue_size descriptors without finding
        // a descriptor without NEXT => cyclic chain.
        Err(VirtQueueError::InvalidDescriptor)
    }

    fn allocate_descriptors(&mut self, count: usize) -> Result<Vec<u16>, VirtQueueError> {
        if count == 0 || count > self.queue_size() as usize {
            return Err(VirtQueueError::QueueFull);
        }

        let descriptors: Vec<u16> = self
            .free
            .iter()
            .enumerate()
            .filter_map(
                |(index, is_free)| {
                    if *is_free { Some(index as u16) } else { None }
                },
            )
            .take(count)
            .collect();

        if descriptors.len() != count {
            return Err(VirtQueueError::QueueFull);
        }

        for &index in &descriptors {
            self.free[index as usize] = false;
        }

        Ok(descriptors)
    }

    pub fn add_chain(&mut self, descriptors: &[Descriptor]) -> Result<u16, VirtQueueError> {
        if descriptors.is_empty() {
            return Err(VirtQueueError::InvalidDescriptor);
        }

        let indices = self.allocate_descriptors(descriptors.len())?;

        for (position, &index) in indices.iter().enumerate() {
            let mut descriptor = descriptors[position];

            if position + 1 < indices.len() {
                descriptor.flags |= DESC_F_NEXT;
                descriptor.next = indices[position + 1];
            } else {
                descriptor.flags &= !DESC_F_NEXT;
                descriptor.next = 0;
            }

            self.set_descriptor(index, descriptor)?;
        }

        let head = indices[0];

        self.add_available(head)?;

        Ok(head)
    }

    pub fn set_descriptor(
        &mut self,
        index: u16,
        descriptor: Descriptor,
    ) -> Result<(), VirtQueueError> {
        let addr = self.layout.descriptor_address(self.base, index)?;

        self.memory.write(addr, &descriptor.encode_le())?;

        Ok(())
    }

    fn available_idx_address(&self) -> GuestAddress {
        self.layout.available_address(self.base).offset(2)
    }

    fn available_ring_address(&self, index: u16) -> GuestAddress {
        self.layout
            .available_address(self.base)
            .offset(4 + 2 * index as usize)
    }

    pub fn available_index(&self) -> Result<u16, VirtQueueError> {
        Ok(self.memory.read_u16(self.available_idx_address())?)
    }

    pub fn add_available(&mut self, head: u16) -> Result<(), VirtQueueError> {
        if head >= self.queue_size() {
            return Err(VirtQueueError::InvalidDescriptor);
        }

        let idx = self.available_index()?;
        let slot = idx % self.queue_size();

        self.memory
            .write_u16(self.available_ring_address(slot), head)?;

        fence(Ordering::Release);

        self.memory
            .write_u16(self.available_idx_address(), idx.wrapping_add(1))?;

        Ok(())
    }

    pub fn pop_available(&mut self) -> Result<Option<u16>, VirtQueueError> {
        let avail_idx = self.available_index()?;
        if self.next_avail == avail_idx {
            return Ok(None);
        }
        let slot = self.next_avail % self.queue_size();
        let head = self.memory.read_u16(self.available_ring_address(slot))?;
        self.next_avail = self.next_avail.wrapping_add(1);
        Ok(Some(head))
    }

    pub fn pop_chain(&mut self) -> Result<Option<DescriptorChain>, VirtQueueError> {
        Ok(self.pop_available()?.map(DescriptorChain::new))
    }

    pub fn push_used(&mut self, id: u32, len: u32) -> Result<(), VirtQueueError> {
        if id >= self.queue_size() as u32 {
            return Err(VirtQueueError::InvalidDescriptor);
        }

        let used_idx = self.used_index()?;
        let slot = used_idx % self.queue_size();

        let addr = self.used_element_address(slot);

        self.memory.write_u32(addr, id)?;
        self.memory.write_u32(addr.offset(4), len)?;

        fence(Ordering::Release);

        self.memory
            .write_u16(self.used_idx_address(), used_idx.wrapping_add(1))?;

        Ok(())
    }

    fn used_idx_address(&self) -> GuestAddress {
        self.layout.used_address(self.base).offset(2)
    }

    fn used_element_address(&self, index: u16) -> GuestAddress {
        self.layout
            .used_address(self.base)
            .offset(4 + 8 * index as usize)
    }

    pub fn used_index(&self) -> Result<u16, VirtQueueError> {
        fence(Ordering::Acquire);

        Ok(self.memory.read_u16(self.used_idx_address())?)
    }

    pub fn add_used(&mut self, id: u32, len: u32) -> Result<(), VirtQueueError> {
        let used_idx = self.used_index()?;
        let slot = used_idx % self.queue_size();

        let addr = self.used_element_address(slot);

        self.memory.write_u32(addr, id)?;
        self.memory.write_u32(addr.offset(4), len)?;

        fence(Ordering::Release);

        self.memory
            .write_u16(self.used_idx_address(), used_idx.wrapping_add(1))?;

        Ok(())
    }

    pub fn pop_used(&mut self) -> Result<Option<UsedElement>, VirtQueueError> {
        let used_idx = self.used_index()?;

        if self.last_used == used_idx {
            return Ok(None);
        }

        let slot = self.last_used % self.queue_size();
        let addr = self.used_element_address(slot);

        let id = self.memory.read_u32(addr)?;
        let len = self.memory.read_u32(addr.offset(4))?;

        self.last_used = self.last_used.wrapping_add(1);

        if id < self.queue_size() as u32 {
            let head = id as u16;

            if let Ok(chain) = self.descriptor_chain(head) {
                for (index, _) in chain {
                    self.free[index as usize] = true;
                }
            } else {
                self.free[head as usize] = true;
            }
        }

        Ok(Some(UsedElement { id, len }))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DescriptorChain {
    pub head: u16,
}

impl DescriptorChain {
    pub const fn new(head: u16) -> Self {
        Self { head }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_16_bytes() {
        assert_eq!(Descriptor::SIZE, 16);
    }

    #[test]
    fn descriptor_round_trip() {
        let descriptor = Descriptor {
            addr: GuestAddress(0x1234_5678),
            len: 4096,
            flags: DESC_F_NEXT | DESC_F_WRITE,
            next: 7,
        };

        let bytes = descriptor.encode_le();
        let decoded = Descriptor::decode_le(&bytes).unwrap();

        assert_eq!(decoded, descriptor);
    }

    #[test]
    fn queue_size_must_be_power_of_two() {
        assert_eq!(
            SplitVirtQueueLayout::new(3),
            Err(VirtQueueError::InvalidQueueSize)
        );

        assert!(SplitVirtQueueLayout::new(4).is_ok());
    }

    #[test]
    fn queue_size_32768_is_allowed() {
        assert!(SplitVirtQueueLayout::new(32768).is_ok());
    }

    #[test]
    fn queue_size_above_maximum_is_rejected() {
        assert_eq!(
            SplitVirtQueueLayout::new(65535),
            Err(VirtQueueError::InvalidQueueSize)
        );
    }

    #[test]
    fn layout_has_required_alignment() {
        let layout = SplitVirtQueueLayout::new(256).unwrap();

        assert_eq!(layout.descriptor_offset % 16, 0);
        assert_eq!(layout.available_offset % 2, 0);
        assert_eq!(layout.used_offset % 4, 0);
    }

    #[test]
    fn layout_has_expected_sizes() {
        let layout = SplitVirtQueueLayout::new(8).unwrap();

        assert_eq!(layout.descriptor_offset, 0);
        assert_eq!(layout.available_offset, 128);
        assert_eq!(layout.used_offset, 152);
        assert_eq!(layout.total_size, 222);
    }

    #[test]
    fn queue_initializes_empty() {
        let queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();

        assert_eq!(queue.available_index().unwrap(), 0);
        assert_eq!(queue.used_index().unwrap(), 0);
    }

    #[test]
    fn descriptor_can_be_written_and_read() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        let descriptor = Descriptor {
            addr: GuestAddress(0x8000),
            len: 128,
            flags: DESC_F_WRITE,
            next: 0,
        };

        queue.set_descriptor(2, descriptor).unwrap();

        assert_eq!(queue.descriptor(2).unwrap(), descriptor);
    }

    #[test]
    fn available_buffer_is_published() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        queue
            .set_descriptor(
                0,
                Descriptor {
                    addr: GuestAddress(0x2000),
                    len: 64,
                    flags: 0,
                    next: 0,
                },
            )
            .unwrap();

        queue.add_available(0).unwrap();

        assert_eq!(queue.available_index().unwrap(), 1);
    }

    #[test]
    fn used_buffer_can_be_consumed() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        let used_addr = queue.layout().used_address(queue.base());

        queue
            .memory_mut()
            .write_u32(used_addr.offset(4), 0)
            .unwrap();

        queue
            .memory_mut()
            .write_u32(used_addr.offset(8), 128)
            .unwrap();

        queue
            .memory_mut()
            .write_u16(used_addr.offset(2), 1)
            .unwrap();

        let used = queue.pop_used().unwrap().unwrap();

        assert_eq!(used, UsedElement { id: 0, len: 128 });
    }
    #[test]
    fn descriptor_chain_can_be_followed() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        queue
            .set_descriptor(
                0,
                Descriptor {
                    addr: GuestAddress(0x2000),
                    len: 64,
                    flags: DESC_F_NEXT,
                    next: 3,
                },
            )
            .unwrap();

        queue
            .set_descriptor(
                3,
                Descriptor {
                    addr: GuestAddress(0x3000),
                    len: 128,
                    flags: DESC_F_WRITE,
                    next: 0,
                },
            )
            .unwrap();

        let chain = queue.descriptor_chain(0).unwrap();

        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].0, 0);
        assert_eq!(chain[1].0, 3);
    }
    #[test]
    fn descriptor_chain_rejects_loop() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        queue
            .set_descriptor(
                0,
                Descriptor {
                    addr: GuestAddress(0x2000),
                    len: 64,
                    flags: DESC_F_NEXT,
                    next: 1,
                },
            )
            .unwrap();

        queue
            .set_descriptor(
                1,
                Descriptor {
                    addr: GuestAddress(0x3000),
                    len: 64,
                    flags: DESC_F_NEXT,
                    next: 0,
                },
            )
            .unwrap();

        assert_eq!(
            queue.descriptor_chain(0),
            Err(VirtQueueError::InvalidDescriptor)
        );
    }

    #[test]
    fn used_chain_releases_all_descriptors() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        queue
            .set_descriptor(
                0,
                Descriptor {
                    addr: GuestAddress(0x2000),
                    len: 64,
                    flags: DESC_F_NEXT,
                    next: 3,
                },
            )
            .unwrap();

        queue
            .set_descriptor(
                3,
                Descriptor {
                    addr: GuestAddress(0x3000),
                    len: 128,
                    flags: 0,
                    next: 0,
                },
            )
            .unwrap();

        queue.add_available(0).unwrap();

        let used_addr = queue.layout().used_address(queue.base());

        queue
            .memory_mut()
            .write_u32(used_addr.offset(4), 0)
            .unwrap();

        queue
            .memory_mut()
            .write_u32(used_addr.offset(8), 192)
            .unwrap();

        queue
            .memory_mut()
            .write_u16(used_addr.offset(2), 1)
            .unwrap();

        queue.pop_used().unwrap().unwrap();

        // Both descriptors should now be available again.
        assert!(queue.free[0]);
        assert!(queue.free[3]);
    }

    #[test]
    fn add_chain_allocates_single_descriptor() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        let head = queue
            .add_chain(&[Descriptor {
                addr: GuestAddress(0x2000),
                len: 64,
                flags: 0,
                next: 0,
            }])
            .unwrap();

        assert_eq!(head, 0);

        let descriptor = queue.descriptor(head).unwrap();

        assert_eq!(descriptor.addr, GuestAddress(0x2000));
        assert_eq!(descriptor.len, 64);
        assert_eq!(descriptor.flags & DESC_F_NEXT, 0);
    }
    #[test]
    fn add_chain_links_descriptors() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        let head = queue
            .add_chain(&[
                Descriptor {
                    addr: GuestAddress(0x2000),
                    len: 64,
                    flags: 0,
                    next: 0,
                },
                Descriptor {
                    addr: GuestAddress(0x3000),
                    len: 128,
                    flags: DESC_F_WRITE,
                    next: 0,
                },
            ])
            .unwrap();

        let chain = queue.descriptor_chain(head).unwrap();

        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].1.addr, GuestAddress(0x2000));
        assert_eq!(chain[1].1.addr, GuestAddress(0x3000));

        assert_ne!(chain[0].1.flags & DESC_F_NEXT, 0);
        assert_eq!(chain[0].1.next, chain[1].0);

        assert_eq!(chain[1].1.flags & DESC_F_NEXT, 0);
    }
    #[test]
    fn add_chain_rejects_when_descriptors_are_exhausted() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        let descriptors = [
            Descriptor::default(),
            Descriptor::default(),
            Descriptor::default(),
            Descriptor::default(),
            Descriptor::default(),
            Descriptor::default(),
            Descriptor::default(),
            Descriptor::default(),
        ];

        queue.add_chain(&descriptors).unwrap();

        assert_eq!(
            queue.add_chain(&[Descriptor::default()]),
            Err(VirtQueueError::QueueFull)
        );
    }
    #[test]
    fn available_buffer_can_be_popped() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        queue
            .set_descriptor(
                3,
                Descriptor {
                    addr: GuestAddress(0x2000),
                    len: 64,
                    flags: 0,
                    next: 0,
                },
            )
            .unwrap();

        queue.add_available(3).unwrap();

        assert_eq!(queue.pop_available().unwrap(), Some(3));
        assert_eq!(queue.pop_available().unwrap(), None);
    }

    #[test]
    fn used_buffer_can_be_published() {
        let mut queue = SplitVirtQueue::new(
            GuestMemory::new(GuestAddress(0), 0x10000),
            GuestAddress(0x1000),
            8,
        )
        .unwrap();
        queue.push_used(3, 128).unwrap();
        assert_eq!(queue.used_index().unwrap(), 1);
        let used = queue.pop_used().unwrap().unwrap();
        assert_eq!(used, UsedElement { id: 3, len: 128 });
    }
}
