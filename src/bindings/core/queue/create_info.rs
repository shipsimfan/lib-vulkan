use crate::{VkDeviceQueueCreateFlags, VkStructureType};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkDeviceQueueCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkDeviceQueueCreateFlags,
    pub(crate) queue_family_index: u32,
    pub(crate) queue_count: u32,
    pub(crate) queue_priorities: *const f32,
}
