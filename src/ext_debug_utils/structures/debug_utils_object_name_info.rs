use crate::{VkObjectType, VkStructureType};
use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Specify parameters of a name to give to an object
///
/// Applications may change the name associated with an object simply by calling
/// [`VkSetDebugUtilsObjectNameEXT`] again with a new string. If `object_name` is either [`null`]
/// or an empty string, then any previously set name is removed.
///
/// The `graphics_pipeline_library` feature allows the specification of pipelines without the
/// creation of [`VkShaderModule`] objects beforehand. In order to continue to allow naming these
/// shaders independently, [`VkDebugUtilsObjectNameInfoEXT`] can be included in the `next` chain of
/// [`VkPipelineShaderStageCreateInfo`], which associates a static name with that particular
/// shader.
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `object_type` is a [`VkObjectType`] specifying the type of the object to be named.
    pub object_type: VkObjectType,

    /// `object_handle` is the object to be named.
    pub object_handle: u64,

    /// `object_name` is either [`null`] or a null-terminated UTF-8 string specifying the name to
    /// apply to `object_handle`.
    pub object_name: *const c_char,
}

impl Default for VkDebugUtilsObjectNameInfoEXT {
    fn default() -> Self {
        VkDebugUtilsObjectNameInfoEXT {
            r#type: VkStructureType::DebugUtilsObjectNameInfoEXT,
            next: null(),
            object_type: VkObjectType::Unknown,
            object_handle: 0,
            object_name: null(),
        }
    }
}
