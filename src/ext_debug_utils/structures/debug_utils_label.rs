use crate::VkStructureType;
use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Specify parameters of a label region
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkDebugUtilsLabelExt {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DebugUtilsLabelExt`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `label_name` is a pointer to a null-terminated UTF-8 string containing the name of the
    /// label.
    ///
    /// # Valid Usage (Implicit)
    ///  - `label_name` must be a null-terminated UTF-8 string
    pub label_name: *const c_char,

    /// `color` is an optional RGBA color value that can be associated with the label. A particular
    /// implementation may choose to ignore this color value. The values contain RGBA values in
    /// order, in the range 0.0 to 1.0. If all elements in color are set to 0.0 then it is ignored.
    pub color: [f32; 4],
}

impl Default for VkDebugUtilsLabelExt {
    fn default() -> Self {
        VkDebugUtilsLabelExt {
            r#type: VkStructureType::DebugUtilsLabelExt,
            next: null(),
            label_name: null(),
            color: [0.; 4],
        }
    }
}
