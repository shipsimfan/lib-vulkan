use crate::{VkCommandBuffer, VkCommandBufferAllocateInfo, VkDevice, VkResult};

pub(crate) type VkAllocateCommandBuffers = extern "system" fn(
    device: VkDevice,
    p_allocate_info: *const VkCommandBufferAllocateInfo,
    p_command_buffers: *mut VkCommandBuffer,
) -> VkResult;
