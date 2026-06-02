use crate::{VkObjectType, VkStructureType, util::NextChain};
use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE,
    ext_debug_utils::{self, VkSetDebugUtilsObjectNameExt},
};

/// Specify parameters of a name to give to an object
///
/// # Description
/// Applications may change the name associated with an object simply by calling
/// [`VkSetDebugUtilsObjectNameExt`] again with a new string. If `object_name` is either [`null`]
/// or an empty string, then any previously set name is removed.
///
/// The `graphics_pipeline_library` feature allows the specification of pipelines without the
/// creation of [`VkShaderModule`] objects beforehand. In order to continue to allow naming these
/// shaders independently, [`VkDebugUtilsObjectNameInfoExt`] can be included in the `next` chain of
/// [`VkPipelineShaderStageCreateInfo`], which associates a static name with that particular
/// shader.
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDebugUtilsObjectNameInfoExt {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DebugUtilsObjectNameInfoExt`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `object_type` is a [`VkObjectType`] specifying the type of the object to be named.
    ///
    /// # Valid Usage (Implicit)
    ///  - `object_type` must be a valid [`VkObjectType`] value
    pub object_type: VkObjectType,

    /// `object_handle` is the object to be named.
    ///
    /// # Valid Usage
    ///  - If `object_type` is [`VkObjectType::Unknown`], `object_handle` must not be
    ///    [`VK_NULL_HANDLE`]
    ///  - If `object_type` is not [`VkObjectType::Unknown`], `object_handle` must be
    ///    [`VK_NULL_HANDLE`] or a valid Vulkan handle of the type associated with `object_type`
    pub object_handle: u64,

    /// `object_name` is either [`null`] or a null-terminated UTF-8 string specifying the name to
    /// apply to `object_handle`.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `object_name` is not [`null`], `object_name` must be a null-terminated UTF-8 string
    pub object_name: *const c_char,
}

impl const Default for VkDebugUtilsObjectNameInfoExt {
    fn default() -> Self {
        VkDebugUtilsObjectNameInfoExt {
            r#type: VkStructureType::DebugUtilsObjectNameInfoExt,
            next: null(),
            object_type: VkObjectType::Unknown,
            object_handle: 0,
            object_name: null(),
        }
    }
}

impl NextChain for VkDebugUtilsObjectNameInfoExt {
    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: *const c_void) {
        self.next = next;
    }
}
