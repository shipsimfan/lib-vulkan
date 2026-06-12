use crate::VkSpecializationMapEntry;
use std::{
    ffi::{c_size_t, c_void},
    ptr::null,
};

/// Structure specifying specialization information
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSpecializationInfo {
    /// `map_entry_count` is the number of entries in the `map_entries` array.
    pub map_entry_count: u32,

    /// `map_entries` is a pointer to an array of [`VkSpecializationMapEntry`] structures, which
    /// map constant IDs to offsets in `data`.
    ///
    /// # Valid Usage
    ///  - The `offset` member of each element of `map_entries` must be less than `data_size`
    ///  - The `size` member of each element of `map_entries` must be less than or equal to
    ///    `data_size` minus `offset`
    ///  - The `constant_id` value of each element of `map_entries` must be unique within
    ///    `map_entries`
    ///
    /// # Valid Usage (Implicit)
    ///  - If `map_entry_count` is not 0, `map_entries` must be a valid pointer to an array of
    ///    `map_entry_count` valid [`VkSpecializationMapEntry`] structures
    pub map_entries: *const VkSpecializationMapEntry,

    /// `data_size` is the byte size of the `data` buffer.
    pub data_size: c_size_t,

    /// `data` contains the actual constant values to specialize with.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `data_size` is not 0, `data` must be a valid pointer to an array of `data_size` bytes
    pub data: *const c_void,
}

impl const Default for VkSpecializationInfo {
    fn default() -> Self {
        VkSpecializationInfo {
            map_entry_count: 0,
            map_entries: null(),
            data_size: 0,
            data: null(),
        }
    }
}
