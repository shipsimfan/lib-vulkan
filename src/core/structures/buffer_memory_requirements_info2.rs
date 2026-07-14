use crate::{VkBuffer, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_1;

/// Provided by [`VK_VERSION_1_1`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferMemoryRequirementsInfo2 {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::BufferMemoryRequirementsInfo2`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `buffer` is the buffer to query.
    ///
    /// # Valid Usage (Implicit)
    ///  - `buffer` must be a valid [`VkBuffer`] handle
    pub buffer: VkBuffer,
}

impl const Default for VkBufferMemoryRequirementsInfo2 {
    fn default() -> Self {
        VkBufferMemoryRequirementsInfo2 {
            r#type: VkStructureType::BufferMemoryRequirementsInfo2,
            next: null(),
            buffer: VkBuffer::null(),
        }
    }
}

impl NextChain for VkBufferMemoryRequirementsInfo2 {
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
