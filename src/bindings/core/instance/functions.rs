use crate::{VkAllocationCallbacks, VkInstance};

pub(crate) type VkDestroyInstance =
    extern "system" fn(instance: VkInstance, p_allocator: *const VkAllocationCallbacks);
