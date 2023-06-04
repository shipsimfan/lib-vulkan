use crate::{VkCommandPoolCreateFlags, VkStructureType};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkCommandPoolCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkCommandPoolCreateFlags,
    pub(crate) queue_family_index: u32,
}
