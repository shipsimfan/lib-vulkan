use crate::{VkShaderModuleCreateFlags, VkStructureType};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkShaderModuleCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkShaderModuleCreateFlags,
    pub(crate) code_size: usize,
    pub(crate) p_code: *const u32,
}
