use crate::VkStructureType;
use std::ffi::{c_char, c_void};

#[repr(C)]
pub(crate) struct VkApplicationInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) p_application_name: *const c_char,
    pub(crate) application_version: u32,
    pub(crate) p_engine_name: *const c_char,
    pub(crate) engine_version: u32,
    pub(crate) api_version: u32,
}
