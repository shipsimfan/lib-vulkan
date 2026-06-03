use crate::{VkObjectType, VkStructureType, util::NextChain};
use std::{
    ffi::{c_size_t, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Specify parameters of a tag to attach to an object
///
/// # Description
/// The `tag_name` parameter gives a name or identifier to the type of data being tagged. This can
/// be used by debugging layers to easily filter for only data that can be used by that
/// implementation.
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDebugUtilsObjectTagInfoExt {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DebugUtilsObjectTagInfoExt`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `object_type` is a [`VkObjectType`] specifying the type of the object to be named.
    ///
    /// # Valid Usage
    ///  - `object_type` must not be [`VkObjectType::Unknown`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `object_type` must be a valid [`VkObjectType`] value
    pub object_type: VkObjectType,

    /// `object_handle` is the object to be tagged.
    ///
    /// # Valid Usage
    ///  - `object_handle` must be a valid Vulkan handle of the type associated with `object_type`
    ///
    /// # Host Synchronization
    ///  - Host access to `object_handle` must be externally synchronized
    pub object_handle: u64,

    /// `tag_name` is a numerical identifier of the tag.
    pub tag_name: u64,

    /// `tag_size` is the number of bytes of data to attach to the object.
    ///
    /// # Valid Usage (Implicit)
    ///  - `tag_size` must be greater than 0
    pub tag_size: c_size_t,

    /// `tag` is a pointer to an array of `tag_size` bytes containing the data to be associated
    /// with the object.
    ///
    /// # Valid Usage (Implicit)
    ///  - `tag` must be a valid pointer to an array of `tag_size` bytes
    pub tag: *const c_void,
}

impl const Default for VkDebugUtilsObjectTagInfoExt {
    fn default() -> Self {
        VkDebugUtilsObjectTagInfoExt {
            r#type: VkStructureType::DebugUtilsObjectTagInfoExt,
            next: null(),
            object_type: VkObjectType::Unknown,
            object_handle: 0,
            tag_name: 0,
            tag_size: 0,
            tag: null(),
        }
    }
}

impl NextChain for VkDebugUtilsObjectTagInfoExt {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }
    
    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
