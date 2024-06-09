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
pub struct VkDebugUtilsLabelEXT {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `label_name` is a pointer to a null-terminated UTF-8 string containing the name of the
    /// label.
    pub label_name: *const c_char,

    /// `color` is an optional RGBA color value that can be associated with the label. A particular
    /// implementation may choose to ignore this color value. The values contain RGBA values in
    /// order, in the range 0.0 to 1.0. If all elements in color are set to 0.0 then it is ignored.
    pub color: [f32; 4],
}

impl Default for VkDebugUtilsLabelEXT {
    fn default() -> Self {
        VkDebugUtilsLabelEXT {
            r#type: VkStructureType::DebugUtilsLabelEXT,
            next: null(),
            label_name: null(),
            color: [0.; 4],
        }
    }
}
