use crate::VkSpecializationMapEntry;
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkSpecializationInfo {
    pub(crate) map_entry_count: u32,
    pub(crate) p_map_entries: *const VkSpecializationMapEntry,
    pub(crate) data_size: usize,
    pub(crate) p_data: *const c_void,
}
