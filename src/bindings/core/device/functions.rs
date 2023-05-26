use crate::{VkAllocationCallbacks, VkDevice, VkQueue};

pub(crate) type VkDestroyDevice =
    extern "system" fn(device: VkDevice, p_allocator: *const VkAllocationCallbacks);

pub(crate) type VkGetDeviceQueue = extern "system" fn(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut Option<VkQueue>,
);
