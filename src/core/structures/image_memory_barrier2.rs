use crate::{
    VkAccessFlags2, VkImage, VkImageLayout, VkImageSubresourceRange, VkPipelineStageFlags2,
    VkStructureType, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkDependencyFlag, VkImageAspectFlag, VkSharingMode};

/// Structure specifying an image memory barrier
///
/// # Description
/// This structure defines a memory dependency limited to an image subresource range, and can
/// define a queue family ownership transfer operation and image layout transition for that
/// subresource range.
///
/// The first synchronization scope and access scope described by this structure include only
/// operations and memory accesses specified by the source stage mask and the source access mask.
///
/// The second synchronization scope and access scope described by this structure include only
/// operations and memory accesses specified by the destination stage mask and the destination
/// access mask.
///
/// Both access scopes are limited to only memory accesses to `image` in the subresource range
/// defined by `subresource_range`.
///
/// If `image` was created with [`VkSharingMode::Exclusive`], and `src_queue_family_index` is not
/// equal to `dst_queue_family_index`, this memory barrier defines a queue family ownership
/// transfer operation. When executed on a queue in the family identified by
/// `src_queue_family_index`, this barrier defines a queue family release operation for the
/// specified image subresource range, and if [`VkDependencyInfo::dependency_flags`] did not
/// include [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the second
/// synchronization scope does not apply to this operation. When executed on a queue in the family
/// identified by `dst_queue_family_index`, this barrier defines a queue family acquire operation
/// for the specified image subresource range, and if [`VkDependencyInfo::dependency_flags`] did
/// not include [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the first
/// synchronization scope does not apply to this operation.
///
/// A queue family ownership transfer operation is also defined if the values are not equal, and
/// either is one of the special queue family values reserved for external memory ownership
/// transfers. A queue family release operation is defined when `dst_queue_family_index` is one of
/// those values, and a queue family acquire operation is defined when `src_queue_family_index` is
/// one of those values.
///
/// If `old_layout` is not equal to `new_layout`, then the memory barrier defines an image layout
/// transition for the specified image subresource range. If this memory barrier defines a queue
/// family ownership transfer operation, the layout transition is only executed once between the
/// queues.
///
/// If `src_queue_family_index` and `dst_queue_family_index` are equal and `old_layout` and
/// `new_layout` are also equal, the layout values are ignored and the image contents are preserved
/// regardless of the values of `old_layout`, `new_layout`, and the current layout of the image.
///
/// If `image` is a 3D image created with [`VkImageCreateFlag::2dArrayCompatible`] and the
/// `maintenance9` feature is enabled, the `base_array_layer` and `layer_count` members of
/// `subresource_range` specify the subset of slices of the 3D image affected by the memory
/// barrier, including the layout transition. Any slices of a 3D image not included in
/// `subresource_range` are not affected by the memory barrier and remain in their existing layout.
///
/// If `image` has a multi-planar format and the `image` is disjoint, then including
/// [`VkImageAspectFlag::Color`] in the `aspect_mask` member of `subresource_range` is equivalent
/// to including [`VkImageAspectFlag::Plane0`], [`VkImageAspectFlag::Plane1`], and (for three-plane
/// formats only) [`VkImageAspectFlag::Plane2`].
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageMemoryBarrier2 {
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

    /// `old_layout` is the old layout in an image layout transition.
    pub old_layout: VkImageLayout,

    /// `new_layout` is the new layout in an image layout transition.
    pub new_layout: VkImageLayout,

    /// `src_queue_family_index` is the source queue family for a queue family ownership transfer.
    pub src_queue_family_index: u32,

    /// `dst_queue_family_index` is the destination queue family for a queue family ownership
    /// transfer.
    pub dst_queue_family_index: u32,

    /// `image` is a handle to the image affected by this barrier.
    pub image: VkImage,

    /// `subresource_range` describes the image subresource range within `image` that is affected
    /// by this barrier.
    pub subresource_range: VkImageSubresourceRange,
}

impl const Default for VkImageMemoryBarrier2 {
    fn default() -> Self {
        VkImageMemoryBarrier2 {
            r#type: VkStructureType::ImageMemoryBarrier2,
            next: null(),
            src_stage_mask: VkPipelineStageFlags2::default(),
            src_access_mask: VkAccessFlags2::default(),
            dst_stage_mask: VkPipelineStageFlags2::default(),
            dst_access_mask: VkAccessFlags2::default(),
            old_layout: VkImageLayout::Undefined,
            new_layout: VkImageLayout::Undefined,
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            image: VkImage::null(),
            subresource_range: VkImageSubresourceRange::default(),
        }
    }
}

impl NextChain for VkImageMemoryBarrier2 {
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
