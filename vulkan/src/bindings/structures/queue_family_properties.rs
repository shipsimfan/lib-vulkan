use super::{VkExtent3D, VkQueueFlags};

#[repr(C)]
pub struct VkQueueFamilyProperties {
    queue_flags: VkQueueFlags,
    queue_count: u32,
    timestamp_valid_bits: u32,
    min_image_transfer_granularity: VkExtent3D,
}

impl VkQueueFamilyProperties {
    pub fn flags(&self) -> VkQueueFlags {
        self.queue_flags
    }

    pub fn queue_count(&self) -> u32 {
        self.queue_count
    }

    pub fn timestamp_valid_bits(&self) -> u32 {
        self.timestamp_valid_bits
    }

    pub fn min_image_transfer_granularity(&self) -> &VkExtent3D {
        &self.min_image_transfer_granularity
    }
}
