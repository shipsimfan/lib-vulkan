use crate::{VkAllocationCallbacks, VkInstance, VkInstanceCreateInfo, VkResult};

pub(crate) type VkCreateInstance = extern "system" fn(
    p_create_info: *const VkInstanceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_instance: *mut Option<VkInstance>,
) -> VkResult;
