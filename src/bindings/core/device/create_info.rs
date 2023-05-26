use crate::{
    VkDeviceCreateFlags, VkDeviceQueueCreateInfo, VkPhysicalDeviceFeatures, VkStructureType,
};
use std::ffi::{c_char, c_void};

#[repr(C)]
pub(crate) struct VkDeviceCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkDeviceCreateFlags,
    pub(crate) queue_create_info_count: u32,
    pub(crate) p_queue_create_infos: *const VkDeviceQueueCreateInfo,
    pub(crate) enabled_layer_count: u32,
    pub(crate) pp_enabled_layer_names: *const *const c_char,
    pub(crate) enabled_extension_count: u32,
    pub(crate) pp_enabled_extension_names: *const *const c_char,
    pub(crate) p_enabled_features: *const VkPhysicalDeviceFeatures,
}
