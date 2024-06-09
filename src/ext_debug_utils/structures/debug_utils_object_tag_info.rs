use crate::{VkObjectType, VkStructureType};
use core::ffi::{c_size_t, c_void};
use std::ptr::null;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Specify parameters of a tag to attach to an object
///
/// The `tag_name` parameter gives a name or identifier to the type of data being tagged. This can
/// be used by debugging layers to easily filter for only data that can be used by that
/// implementation.
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDebugUtilsObjectTagInfoEXT {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `object_type` is a [`VkObjectType`] specifying the type of the object to be named.
    pub object_type: VkObjectType,

    /// `object_handle` is the object to be tagged.
    pub object_handle: u64,

    /// `tag_name` is a numerical identifier of the tag.
    pub tag_name: u64,

    /// `tag_size` is the number of bytes of data to attach to the object.
    pub tag_size: c_size_t,

    /// `tag` is a pointer to an array of `tag_size` bytes containing the data to be associated
    /// with the object.
    pub tag: *const c_void,
}

impl Default for VkDebugUtilsObjectTagInfoEXT {
    fn default() -> Self {
        VkDebugUtilsObjectTagInfoEXT {
            r#type: VkStructureType::DebugUtilsObjectTagInfoEXT,
            next: null(),
            object_type: VkObjectType::Unknown,
            object_handle: 0,
            tag_name: 0,
            tag_size: 0,
            tag: null(),
        }
    }
}
