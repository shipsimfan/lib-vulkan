use crate::{VkAllocationCallbacks, VkDevice, VkRenderPass, VkRenderPassCreateInfo, VkResult};

pub(crate) type VkCreateRenderPass = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkRenderPassCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_render_pass: *mut Option<VkRenderPass>,
) -> VkResult;

pub(crate) type VkDestroyRenderPass = extern "system" fn(
    device: VkDevice,
    render_pass: VkRenderPass,
    p_allocator: *const VkAllocationCallbacks,
);
