use crate::{VkAllocationCallbacks, VkCommandPool, VkCommandPoolCreateInfo, VkDevice, VkResult};

pub(crate) type VkCreateCommandPool = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkCommandPoolCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_framebuffer: *mut Option<VkCommandPool>,
) -> VkResult;

pub(crate) type VkDestroyCommandPool = extern "system" fn(
    device: VkDevice,
    framebuffer: VkCommandPool,
    p_allocator: *const VkAllocationCallbacks,
);
