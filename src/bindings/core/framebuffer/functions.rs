use crate::{VkAllocationCallbacks, VkDevice, VkFramebuffer, VkFramebufferCreateInfo, VkResult};

pub(crate) type VkCreateFramebuffer = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkFramebufferCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_framebuffer: *mut Option<VkFramebuffer>,
) -> VkResult;

pub(crate) type VkDestroyFramebuffer = extern "system" fn(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    p_allocator: *const VkAllocationCallbacks,
);
