use crate::{VkAccessFlags2, VkBuffer, VkDeviceSize, VkPipelineStageFlags2, VkStructureType};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkDependencyFlag, VkSharingMode};

/// Structure specifying a buffer memory barrier
///
/// # Description
/// This structure defines a memory dependency limited to a range of a buffer, and can define a
/// queue family ownership transfer operation for that range.
///
/// The first synchronization scope and access scope described by this structure include only
/// operations and memory accesses specified by the source stage mask and the source access mask.
///
/// The second synchronization scope and access scope described by this structure include only
/// operations and memory accesses specified by the destination stage mask and the destination
/// access mask.
///
/// Both access scopes are limited to only memory accesses to `buffer` in the range defined by
/// `offset` and `size`.
///
/// If `buffer` was created with [`VkSharingMode::Exclusive`], and `src_queue_family_index` is not
/// equal to `dst_queue_family_index`, this memory barrier defines a queue family ownership
/// transfer operation. When executed on a queue in the family identified by
/// `src_queue_family_index`, this barrier defines a queue family release operation for the
/// specified buffer range, and if [`VkDependencyInfo::dependency_flags`] did not include
/// [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the second synchronization
/// scope does not apply to this operation. When executed on a queue in the family identified by
/// `dst_queue_family_index`, this barrier defines a queue family acquire operation for the
/// specified buffer range, and if [`VkDependencyInfo::dependency_flags`] did not include
/// [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the first synchronization
/// scope does not apply to this operation.
///
/// A queue family ownership transfer operation is also defined if the values are not equal, and
/// either is one of the special queue family values reserved for external memory ownership
/// transfers, as described in Queue Family Ownership Transfer. A queue family release operation is
/// defined when `dst_queue_family_index` is one of those values, and a queue family acquire
/// operation is defined when `src_queue_family_index` is one of those values.
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferMemoryBarrier2 {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `src_stage_mask` is a [`VkPipelineStageFlags2`] mask of pipeline stages to be included in
    /// the first synchronization scope.
    pub src_stage_mask: VkPipelineStageFlags2,

    /// `src_access_mask` is a [`VkAccessFlags2`] mask of access flags to be included in the first
    /// access scope.
    pub src_access_mask: VkAccessFlags2,

    /// `dst_stage_mask` is a [`VkPipelineStageFlags2`] mask of pipeline stages to be included in
    /// the second synchronization scope.
    pub dst_stage_mask: VkPipelineStageFlags2,

    /// `dst_access_mask` is a [`VkAccessFlags2`] mask of access flags to be included in the second
    /// access scope.
    pub dst_access_mask: VkAccessFlags2,

    /// `src_queue_family_index` is the source queue family for a queue family ownership transfer.
    pub src_queue_family_index: u32,

    /// `dst_queue_family_index` is the destination queue family for a queue family ownership
    /// transfer.
    pub dst_queue_family_index: u32,

    /// `buffer` is a handle to the buffer whose backing memory is affected by the barrier.
    pub buffer: VkBuffer,

    /// `offset` is an offset in bytes into the backing memory for `buffer`; this is relative to
    /// the base offset as bound to the buffer (see [`VkBindBufferMemory`]).
    pub offset: VkDeviceSize,

    /// `size` is a size in bytes of the affected area of backing memory for `buffer`, or
    /// [`VK_WHOLE_SIZE`] to use the range from offset to the end of the buffer.
    pub size: VkDeviceSize,
}

impl Default for VkBufferMemoryBarrier2 {
    fn default() -> Self {
        VkBufferMemoryBarrier2 {
            r#type: VkStructureType::BufferMemoryBarrier2,
            next: null(),
            src_access_mask: VkAccessFlags2::default(),
            src_stage_mask: VkPipelineStageFlags2::default(),
            dst_access_mask: VkAccessFlags2::default(),
            dst_stage_mask: VkPipelineStageFlags2::default(),
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            buffer: VkBuffer::null(),
            offset: 0,
            size: 0,
        }
    }
}
