use crate::{
    VkAccessFlags, VkImage, VkImageLayout, VkImageSubresourceRange, VkStructureType,
    util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkAccessFlag, VkApplicationInfo, VkDependencyFlag, VkImageAspectFlag,
    VkImageUsageFlag, VkInstance, VkSharingMode,
};

/// Structure specifying the parameters of an image memory barrier
///
/// # Description
/// The first access scope is limited to access to memory through the specified `image` subresource
/// range, via access types in the source access mask specified by `src_access_mask`. If
/// `src_access_mask` includes [`VkAccessFlag::HostWrite`], memory writes performed by that
/// access type are also made visible, as that access type is not performed through a resource.
///
/// The second access scope is limited to access to memory through the specified `image`
/// subresource range, via access types in the destination access mask specified by
/// `dst_access_mask`. If `dst_access_mask` includes [`VkAccessFlag::HostWrite`] or
/// [`VkAccessFlag::HostRead`], available memory writes are also made visible to accesses of
/// those types, as those access types are not performed through a resource.
///
/// If `src_queue_family_index` is not equal to `dst_queue_family_index`, and
/// `src_queue_family_index` is equal to the current queue family, then the memory barrier defines
/// a queue family release operation for the specified `image` subresource range, and if
/// `dependency_flags` did not include
/// [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the second
/// synchronization scope of the calling command does not apply to this operation.
///
/// If `dst_queue_family_index` is not equal to `src_queue_family_index`, and
/// `dst_queue_family_index` is equal to the current queue family, then the memory barrier defines
/// a queue family acquire operation for the specified `image` subresource range, and if
/// `dependency_flags` did not include
/// [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the first synchronization
/// scope of the calling command does not apply to this operation.
///
/// If the `synchronization2` feature is not enabled or `old_layout` is not equal to `new_layout`,
/// `old_layout` and `new_layout` define an image layout transition for the specified `image`
/// subresource range.
///
/// If the `synchronization2` feature is enabled, `src_queue_family_index` and
/// `dst_queue_family_index` are equal, and `old_layout` and `new_layout` are also equal, the
/// layout values are ignored and the image contents are preserved regardless of the values of
/// `old_layout`, `new_layout`, and the current layout of the `image`.
///
/// If `image` is a 3D image created with [`VkImageCreateFlag::_2dArrayCompatible`] and the
/// `maintenance9` feature is enabled, the `base_array_layer` and `layer_count` members of
/// `subresource_range` specify the subset of slices of the 3D image affected by the memory
/// barrier, including the layout transition. Any slices of a 3D image not included in
/// `subresource_range` are not affected by the memory barrier and remain in their existing layout.
///
/// If `image` has a multi-planar format and the `image` is disjoint, then including
/// [`VkImageAspectFlag::Color`] in the `aspect_mask` member of `subresource_range` is
/// equivalent to including [`VkImageAspectFlag::Plane0`], [`VkImageAspectFlag::Plane1`], and
/// (for three-plane formats only) [`VkImageAspectFlag::Plane2`].
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageMemoryBarrier {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::ImageMemoryBarrier`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkExternalMemoryAcquireUnmodifiedExt`] or [`VkSampleLocationsInfoExt`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `src_access_mask` is a bitmask of [`VkAccessFlag`]s specifying a source access mask.
    pub src_access_mask: VkAccessFlags,

    /// `dst_access_mask` is a bitmask of [`VkAccessFlag`]s specifying a destination access mask.
    pub dst_access_mask: VkAccessFlags,

    /// `old_layout` is the old layout in an `image` layout transition.
    ///
    /// # Valid Usage
    ///  - If layouts are not ignored, `old_layout` must be [`VkImageLayout::Undefined`] or the
    ///    current layout of the image subresources affected by the barrier
    ///  - If the `aspect_mask` member of `subresource_range` includes
    ///    [`VkImageAspectFlag::Depth`], `old_layout` must not be one of
    ///    [`VkImageLayout::StencilAttachmentOptimal`] or [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If the `aspect_mask` member of `subresource_range` includes
    ///    [`VkImageAspectFlag::Stencil`], `old_layout` must not be one of
    ///    [`VkImageLayout::DepthAttachmentOptimal`] or [`VkImageLayout::DepthReadOnlyOptimal`]
    ///  - If the `zero_initialize_device_memory` feature is not enabled, `old_layout` must not be
    ///    [`VkImageLayout::ZeroInitializedExt`]
    ///  - If `old_layout` is [`VkImageLayout::ZeroInitializedExt`], then all subresources must be
    ///    included in the barrier
    ///  - If the `synchronization2` feature is not enabled, `old_layout` must not be
    ///    [`VkImageLayout::AttachmentOptimal`] or [`VkImageLayout::ReadOnlyOptimal`]
    ///  - If the `dynamic_rendering_local_read` feature is not enabled, `old_layout` must not be
    ///    [`VkImageLayout::RenderingLocalRead`]
    ///  - If `src_queue_family_index` is [`VK_QUEUE_FAMILY_EXTERNAL`] and `image` was created with
    ///    [`VkExternalMemoryHandleType::D3d11Texture`] or
    ///    [`VkExternalMemoryHandleType::D3d11TextureKmt`] in
    ///    [`VkExternalMemoryImageCreateInfo::handle_types`], `old_layout` must be
    ///    [`VkImageLayout::General`] or [`VkImageLayout::Undefined`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `old_layout` must be a valid [`VkImageLayout`] value
    pub old_layout: VkImageLayout,

    /// `new_layout` is the new layout in an `image` layout transition.
    ///
    /// # Valid Usage
    ///  - If layouts are not ignored, `new_layout` must not be [`VkImageLayout::Undefined`] or
    ///    [`VkImageLayout::ZeroInitializedExt`] or [`VkImageLayout::Preinitialized`]
    ///  - If the `aspect_mask` member of `subresource_range` includes
    ///    [`VkImageAspectFlag::Depth`], `new_layout` must not be one of
    ///    [`VkImageLayout::StencilAttachmentOptimal`] or [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If the `aspect_mask` member of `subresource_range` includes
    ///    [`VkImageAspectFlag::Stencil`], `new_layout` must not be one of
    ///    [`VkImageLayout::DepthAttachmentOptimal`] or [`VkImageLayout::DepthReadOnlyOptimal`]
    ///  - If the `synchronization2` feature is not enabled, `new_layout` must not be
    ///    [`VkImageLayout::AttachmentOptimal`] or [`VkImageLayout::ReadOnlyOptimal`]
    ///  - If the `attachment_feedback_loop_layout` feature is not enabled, `new_layout` must not
    ///    be [`VkImageLayout::AttachmentFeedbackLoopOptimalExt`]
    ///  - If the `dynamic_rendering_local_read` feature is not enabled, `new_layout` must not be
    ///    [`VkImageLayout::RenderingLocalRead`]
    ///  - If `dst_queue_family_index` is [`VK_QUEUE_FAMILY_EXTERNAL`] and `image` was created with
    ///    [`VkExternalMemoryHandleType::D3d11Texture`] or
    ///    [`VkExternalMemoryHandleType::D3d11TextureKmt`] in
    ///    [`VkExternalMemoryImageCreateInfo::handle_types`], `new_layout` must be
    ///    [`VkImageLayout::General`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `new_layout` must be a valid [`VkImageLayout`] value
    pub new_layout: VkImageLayout,

    /// `src_queue_family_index` is the source queue family for a queue family ownership transfer.
    ///
    /// # Valid Usage
    ///  - If `image` was created with a sharing mode of [`VkSharingMode::Exclusive`], and
    ///    `src_queue_family_index` and `dst_queue_family_index` are not equal,
    ///    `src_queue_family_index` must be [`VK_QUEUE_FAMILY_EXTERNAL`],
    ///    [`VK_QUEUE_FAMILY_FOREIGN_EXT`], or a valid queue family
    ///  - If the `khr_external_memory` extension is not enabled, and the value of
    ///    [`VkApplicationInfo::api_version`] used to create the [`VkInstance`] is not greater than
    ///    or equal to Version 1.1, `src_queue_family_index` must not be
    ///    [`VK_QUEUE_FAMILY_EXTERNAL`]
    ///  - If the `ext_queue_family_foreign` extension is not enabled `src_queue_family_index` must
    ///    not be [`VK_QUEUE_FAMILY_FOREIGN_EXT`]
    ///  - If the `synchronization2` feature is not enabled, and `image` was created with a sharing
    ///    mode of [`VkSharingMode::Concurrent`], at least one of `src_queue_family_index` and
    ///    `dst_queue_family_index` must be [`VK_QUEUE_FAMILY_IGNORED`]
    ///  - If the `synchronization2` feature is not enabled, and `image` was created with a sharing
    ///    mode of [`VkSharingMode::Concurrent`], `src_queue_family_index` must be
    ///    [`VK_QUEUE_FAMILY_IGNORED`] or [`VK_QUEUE_FAMILY_EXTERNAL`]
    pub src_queue_family_index: u32,

    /// `dst_queue_family_index` is the destination queue family for a queue family ownership
    /// transfer.
    ///
    /// # Valid Usage
    ///  - If `image` was created with a sharing mode of [`VkSharingMode::Exclusive`], and
    ///    `src_queue_family_index` and `dst_queue_family_index` are not equal,
    ///    `dst_queue_family_index` must be [`VK_QUEUE_FAMILY_EXTERNAL`],
    ///    [`VK_QUEUE_FAMILY_FOREIGN_EXT`], or a valid queue family
    ///  - If the `khr_external_memory` extension is not enabled, and the value of
    ///    [`VkApplicationInfo::api_version`] used to create the [`VkInstance`] is not greater than
    ///    or equal to Version 1.1, `dst_queue_family_index` must not be
    ///    [`VK_QUEUE_FAMILY_EXTERNAL`]
    ///  - If the `ext_queue_family_foreign` extension is not enabled `dst_queue_family_index` must
    ///    not be [`VK_QUEUE_FAMILY_FOREIGN_EXT`]
    ///  - If the `synchronization2` feature is not enabled, and `image` was created with a sharing
    ///    mode of [`VkSharingMode::Concurrent`], `dst_queue_family_index` must be
    ///    [`VK_QUEUE_FAMILY_IGNORED`] or [`VK_QUEUE_FAMILY_EXTERNAL`]
    pub dst_queue_family_index: u32,

    /// `image` is a handle to the `image` affected by this barrier.
    ///
    /// # Valid Usage
    ///  - If `image` is non-spase then the `image` or each specified disjoint plane must be bound
    ///    completely and contiguously to a single VkDeviceMemory object
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::ColorAttachmentOptimal`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::ColorAttachment`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::DepthStencilAttachmentOptimal`] then `image` must have been created
    ///    with the [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::DepthStencilReadOnlyOptimal`] then `image` must have been created with
    ///    the [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::ShaderReadOnlyOptimal`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::Sampled`] or [`VkImageUsageFlag::InputAttachment`] usage flag
    ///    set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::TransferSrcOptimal`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::TransferSrc`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::TransferDstOptimal`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::TransferDst`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::DepthReadOnlyStencilAttachmentOptimal`] then `image` must have been
    ///    created with the [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::DepthAttachmentStencilReadOnlyOptimal`] then `image` must have been
    ///    created with the [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::DepthReadOnlyOptimal`] then `image` must have been created with at
    ///    least one of the [`VkImageUsageFlag::DepthStencilAttachment`],
    ///    [`VkImageUsageFlag::Sampled`], or [`VkImageUsageFlag::InputAttachment`] usage
    ///    flags set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::DepthAttachmentOptimal`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::StencilReadOnlyOptimal`] then `image` must have been created with at
    ///    least one of the [`VkImageUsageFlag::DepthStencilAttachment`],
    ///    [`VkImageUsageFlag::Sampled`], or [`VkImageUsageFlag::InputAttachment`] usage
    ///    flags set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::StencilAttachmentOptimal`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::AttachmentOptimal`], `image` must have been created with the
    ///    [`VkImageUsageFlag::ColorAttachment`] or
    ///    [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::ReadOnlyOptimal`], `image` must have been created with at least one of
    ///    the [`VkImageUsageFlag::DepthStencilAttachment`], [`VkImageUsageFlag::Sampled`],
    ///    or [`VkImageUsageFlag::InputAttachment`] usage flags set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::FragmentShadingRateAttachmentOptimalKhr`] then `image` must have been
    ///    created with the [`VkImageUsageFlag::FragmentShadingRateAttachmentKhr`] usage flag
    ///    set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::VideoDecodeSrcKhr`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::VideoDecodeSrcKhr`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::VideoDecodeDstKhr`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::VideoDecodeDstKhr`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::VideoDecodeDpbKhr`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::VideoDecodeDpbKhr`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::VideoEncodeSrcKhr`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::VideoEncodeSrcKhr`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::VideoEncodeDstKhr`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::VideoEncodeDstKhr`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::VideoEncodeDpbKhr`] then `image` must have been created with the
    ///    [`VkImageUsageFlag::VideoEncodeDpbKhr`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::VideoEncodeQuantizationMapKhr`] then `image` must have been created
    ///    with the [`VkImageUsageFlag::VideoEncodeQuantizationDeltaMapKhr`] or
    ///    [`VkImageUsageFlag::VideoEncodeEmphasisMapKhr`] usage flags set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::AttachmentFeedbackLoopOptimalExt`] then `image` must have been created
    ///    with either the [`VkImageUsageFlag::ColorAttachment`] or
    ///    [`VkImageUsageFlag::DepthStencilAttachment`] usage flags set, and the
    ///    [`VkImageUsageFlag::InputAttachment`] or [`VkImageUsageFlag::Sampled`] usage flags
    ///    set, and the [`VkImageUsageFlag::AttachmentFeedbackLoopExt`] usage flag set
    ///  - If layouts are not ignored, `old_layout` or `new_layout` is
    ///    [`VkImageLayout::RenderingLocalRead`] then `image` must have been created with either
    ///    the [`VkImageUsageFlag::Storage`] usage flag set, or with both the
    ///    [`VkImageUsageFlag::InputAttachment`] usage flag and either of the
    ///    [`VkImageUsageFlag::ColorAttachment`] or
    ///    [`VkImageUsageFlag::DepthStencilAttachment`] usage flags set
    ///
    /// # Valid Usage (Implicit)
    ///  - `image` must be a valid [`VkImage`] handle
    pub image: VkImage,

    /// `subresource_range` describes the `image` subresource range within `image` that is affected
    /// by this barrier.
    ///
    /// # Valid Usage
    ///  - `subresource_range.base_mip_level` must be less than the `mip_levels` specified in
    ///    [`VkImageCreateInfo`] when `image` was created
    ///  - If `subresource_range.level_count` is not [`VK_REMAINING_MIP_LEVELS`],
    ///    `subresource_range.base_mip_level + subresource_range.level_count` must be less than or
    ///    equal to the `mip_levels` specified in [`VkImageCreateInfo`] when `image` was created
    ///  - If `image` is not a 3D `image` or was created without
    ///    [`VkImageCreateFlag::_2dArrayCompatible`] set, or the `maintenance9` feature is not
    ///    enabled, `subresource_range.base_array_layer` must be less than the `array_layers`
    ///    specified in [`VkImageCreateInfo`] when `image` was created
    ///  - If the `maintenance9` feature is enabled and `image` is a 3D `image` created with
    ///    [`VkImageCreateFlag::_2dArrayCompatible`] set, `subresource_range.base_array_layer`
    ///    must be less than the depth computed from `base_mip_level` and `extent.depth` specified
    ///    in [`VkImageCreateInfo`] when `image` was created, according to the formula defined in
    ///    Image Mip Level Sizing
    ///  - If the `maintenance9` feature is enabled and `image` is a 3D `image` created with
    ///    [`VkImageCreateFlag::_2dArrayCompatible`] set and either
    ///    `subresource_range.base_array_layer` is not equal to 0 or
    ///    `subresource_range.layer_count` is not equal to [`VK_REMAINING_ARRAY_LAYERS`],
    ///    `subresource_range.level_count` must be 1
    ///  - If `image` is not a 3D `image` or was created without
    ///    [`VkImageCreateFlag::_2dArrayCompatible`] set, or the `maintenance9` feature is not
    ///    enabled, and `subresource_range.layer_count` is not [`VK_REMAINING_ARRAY_LAYERS`],
    ///    `subresource_range.base_array_layer + subresource_range.layer_count` must be less than
    ///    or equal to the `array_layers` specified in [`VkImageCreateInfo`] when `image` was
    ///    created
    ///  - If the `maintenance9` feature is enabled, `subresource_range.layer_count` is not
    ///    [`VK_REMAINING_ARRAY_LAYERS`], and `image` is a 3D `image` created with
    ///    [`VkImageCreateFlag::_2dArrayCompatible`] set,
    ///    `subresource_range.base_array_layer + subresource_range.layer_count` must be less than
    ///    or equal to the depth computed from `base_mip_level` and `extent.depth` specified in
    ///    [`VkImageCreateInfo`] when `image` was created, according to the formula defined in
    ///    Image Mip Level Sizing
    ///  - If `image` has a color format that is single-plane, then the `aspect_mask` member of
    ///    `subresource_range` must be [`VkImageAspectFlag::Color`]
    ///  - If `image` has a color format and is not disjoint, then the `aspect_mask` member of
    ///    `subresource_range` must be [`VkImageAspectFlag::Color`]
    ///  - If `image` has a multi-planar format and the `image` is disjoint, then the `aspect_mask`
    ///    member of `subresource_range` must include at least one multi-planar aspect mask bit or
    ///    [`VkImageAspectFlag::Color`]
    ///  - If `image` has a depth/stencil format with both depth and stencil and the
    ///    `separate_depth_stencil_layouts` feature is not enabled, then the `aspect_mask` member
    ///    of `subresource_range` must include both [`VkImageAspectFlag::Depth`] and
    ///    [`VkImageAspectFlag::Stencil`]
    ///  - If `image` has a depth-only format then the `aspect_mask` member of `subresource_range`
    ///    must be [`VkImageAspectFlag::Depth`]
    ///  - If `image` has a stencil-only format then the `aspect_mask` member of
    ///    `subresource_range` must be [`VkImageAspectFlag::Stencil`]
    ///  - `subresource_range.aspect_mask` must be valid for the format the `image` was created
    ///    with
    ///
    /// # Valid Usage (Implicit)
    ///  - `subresource_range` must be a valid [`VkImageSubresourceRange`] structure
    pub subresource_range: VkImageSubresourceRange,
}

impl const Default for VkImageMemoryBarrier {
    fn default() -> Self {
        VkImageMemoryBarrier {
            r#type: VkStructureType::ImageMemoryBarrier,
            next: null(),
            src_access_mask: VkAccessFlags::default(),
            dst_access_mask: VkAccessFlags::default(),
            old_layout: VkImageLayout::Undefined,
            new_layout: VkImageLayout::Undefined,
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            image: VkImage::null(),
            subresource_range: VkImageSubresourceRange::default(),
        }
    }
}

impl NextChain for VkImageMemoryBarrier {
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
