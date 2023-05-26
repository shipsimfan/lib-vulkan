use crate::{VkAllocationCallbacks, VkDevice};

pub(crate) type VkDestroyDevice =
    extern "system" fn(device: VkDevice, p_allocator: *const VkAllocationCallbacks);
