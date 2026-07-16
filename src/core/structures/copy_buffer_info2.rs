use crate::{VkBuffer, VkBufferCopy2, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkBufferUsageFlag, VkDevice, VkDeviceMemory};

/// Structure specifying parameters of a buffer copy command
///
/// # Valid Usage (Implicit)
///  - Both of `dst_buffer`, and `src_buffer` must have been created, allocated, or retrieved from
///    the same [`VkDevice`]
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkCopyBufferInfo2 {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::CopyBufferInfo2`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `src_buffer` is the source buffer.
    ///
    /// # Valid Usage
    ///  - `src_buffer` must have been created with the [`VkBufferUsageFlag::TransferSrc`] usage
    ///    flag set
    ///  - If `src_buffer` is non-sparse then it must be bound completely and contiguously to a
    ///    single [`VkDeviceMemory`] object
    ///
    /// # Valid Usage (Implicit)
    ///  - `src_buffer` must be a valid [`VkBuffer`] handle
    pub src_buffer: VkBuffer,

    /// `dst_buffer` is the destination buffer.
    ///
    /// # Valid Usage
    ///  - `dst_buffer` must have been created with the [`VkBufferUsageFlag::TransferDst`] usage
    ///    flag set
    ///  - If `dst_buffer` is non-sparse then it must be bound completely and contiguously to a
    ///    single [`VkDeviceMemory`] object
    ///
    /// # Valid Usage (Implicit)
    ///  - `dst_buffer` must be a valid [`VkBuffer`] handle
    pub dst_buffer: VkBuffer,

    /// `region_count` is the number of regions to copy.
    ///
    /// # Valid Usage (Implicit)
    ///  - `region_count` must be greater than 0
    pub region_count: u32,

    /// `regions` is a pointer to an array of [`VkBufferCopy2`] structures specifying the regions
    /// to copy.
    ///
    /// # Valid Usage
    ///  - The `src_offset` member of each element of `regions` must be less than the size of
    ///    `src_buffer`
    ///  - The `dst_offset` member of each element of `regions` must be less than the size of
    ///    `dst_buffer`
    ///  - The `size` member of each element of `regions` must be less than or equal to the size of
    ///    `src_buffer` minus `src_offset`
    ///  - The `size` member of each element of `regions` must be less than or equal to the size of
    ///    `dst_buffer` minus `dst_offset`
    ///  - The union of the source regions, and the union of the destination regions, specified by
    ///    the elements of `regions`, must not overlap in memory
    ///
    /// # Valid Usage (Implicit)
    ///  - `regions` must be a valid pointer to an array of `region_count` valid [`VkBufferCopy2`]
    ///    structures
    pub regions: *const VkBufferCopy2,
}

const impl Default for VkCopyBufferInfo2 {
    fn default() -> Self {
        VkCopyBufferInfo2 {
            r#type: VkStructureType::CopyBufferInfo2,
            next: null(),
            src_buffer: VkBuffer::null(),
            dst_buffer: VkBuffer::null(),
            region_count: 0,
            regions: null(),
        }
    }
}

impl NextChain for VkCopyBufferInfo2 {
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
