use crate::{VkCommandBufferLevel, VkCommandPool, VkStructureType};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkCommandBufferAllocateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) command_pool: VkCommandPool,
    pub(crate) level: VkCommandBufferLevel,
    pub(crate) command_buffer_count: u32,
}
