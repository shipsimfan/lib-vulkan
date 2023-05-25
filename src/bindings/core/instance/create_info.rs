use crate::{VkApplicationInfo, VkInstanceCreateFlags, VkStructureType};
use std::ffi::{c_char, c_void};

#[repr(C)]
pub(crate) struct VkInstanceCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkInstanceCreateFlags,
    pub(crate) p_application_info: *const VkApplicationInfo,
    pub(crate) enabled_layer_count: u32,
    pub(crate) pp_enabled_layers: *const *const c_char,
    pub(crate) enabled_extension_count: u32,
    pub(crate) pp_enabled_extension_names: *const *const c_char,
}
