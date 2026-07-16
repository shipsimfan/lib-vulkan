use crate::{VkDeviceSize, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_3;

/// Structure specifying a buffer copy operation
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferCopy2 {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::BufferCopy2`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `src_offset` is the starting offset in bytes from the start of `src_buffer`.
    pub src_offset: VkDeviceSize,

    /// `dst_offset` is the starting offset in bytes from the start of `dst_buffer`.
    pub dst_offset: VkDeviceSize,

    /// `size` is the number of bytes to copy.
    ///
    /// # Valid Usage
    ///  - The `size` must be greater than 0
    pub size: VkDeviceSize,
}

const impl Default for VkBufferCopy2 {
    fn default() -> VkBufferCopy2 {
        VkBufferCopy2 {
            r#type: VkStructureType::BufferCopy2,
            next: null(),
            src_offset: 0,
            dst_offset: 0,
            size: 0,
        }
    }
}

impl NextChain for VkBufferCopy2 {
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
