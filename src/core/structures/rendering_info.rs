use crate::{VkRect2D, VkRenderingAttachmentInfo, VkRenderingFlags, VkStructureType};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_NULL_HANDLE, VK_VERSION_1_3, VkImageAspectFlag, VkImageLayout, VkImageUsageFlag,
    VkImageView, VkPhysicalDeviceLimits, VkRenderingFlag, VkResolveModeFlag, VkSampleCountFlag,
};

/// Structure specifying render pass instance begin info
///
/// # Description
/// If `view_mask` is not 0, multiview is enabled.
///
/// If there is an instance of [`VkDeviceGroupRenderPassBeginInfo`] included in the `next` chain
/// and its `device_render_area_count` member is not 0, then `render_area` is ignored, and the
/// render area is defined per-device by that structure.
///
/// If multiview is enabled, and the `multiview_per_view_render_area` feature is enabled, and there
/// is an instance of [`VkMultiviewPerViewRenderAreasRenderPassBeginInfoQcom`] included in the
/// `next` chain with `per_view_render_area_count` not equal to 0, then the elements of
/// [`VkMultiviewPerViewRenderAreasRenderPassBeginInfoQcom::per_view_render_areas`] override
/// `render_area` and define a render area for each view. In this case, `render_area` must be an
/// area at least as large as the union of all the per-view render areas.
///
/// Each element of the `color_attachments` array corresponds to an output location in the shader,
/// i.e. if the shader declares an output variable decorated with a `location` value of `x`, then
/// it uses the attachment provided in `color_attachments[x]`. If the `image_view` member of any
/// element of `color_attachments` is [`VK_NULL_HANDLE`], and `resolve_mode` is not
/// [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`], writes to the corresponding location
/// by a fragment are discarded.
///
/// The `aspect_mask` of any image view specified for `depth_attachment` or `stencil_attachment` is
/// ignored. Instead, depth attachments are automatically treated as if
/// [`VkImageAspectFlag::Depth`] was specified for their aspect masks, and stencil attachments
/// are automatically treated as if [`VkImageAspectFlag::Stencil`] was specified for their
/// aspect masks.
///
/// # Valid Usage
///  - Valid attachments specified by this structure must not be bound to memory locations that are
///    bound to any other valid attachments specified by this structure
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkRenderingInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::RenderingInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0 and the `image_view` member of a
    ///    [`VkRenderingFragmentDensityMapAttachmentInfoExt`] structure included in the `next`
    ///    chain is not [`VK_NULL_HANDLE`], `image_view` must have a width greater than or equal to
    ///    `ceil((render_area.x + render_area.width) / max_fragment_density_texel_size.width)`
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0 and the `image_view` member of a
    ///    [`VkRenderingFragmentDensityMapAttachmentInfoExt`] structure included in the `next`
    ///    chain is not [`VK_NULL_HANDLE`], `image_view` must have a height greater than or equal
    ///    to `ceil((render_area.y + render_area.height) / max_fragment_density_texel_size.height)`
    ///  - If the `next` chain contains a [`VkDeviceGroupRenderPassBeginInfo`] structure, its
    ///    `device_render_area_count` member is not 0, and the `image_view` member of a
    ///    [`VkRenderingFragmentDensityMapAttachmentInfoExt`] structure included in the `next`
    ///    chain is not [`VK_NULL_HANDLE`], `image_view` must have a width greater than or equal to
    ///    `ceil((device_render_areas.x + device_render_areas.width) /
    ///    max_fragment_density_texel_size.width)` for each element of `device_render_areas`
    ///  - If the `next` chain contains a [`VkDeviceGroupRenderPassBeginInfo`] structure, its
    ///    `device_render_area_count` member is not 0, and the `image_view` member of a
    ///    [`VkRenderingFragmentDensityMapAttachmentInfoExt`] structure included in the `next`
    ///    chain is not [`VK_NULL_HANDLE`], `image_view` must have a height greater than or equal
    ///    to `ceil((device_render_areas.y + device_render_areas.height) /
    ///    max_fragment_density_texel_size.height)` for each element of `device_render_areas`
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must not be equal
    ///    to the `image_view` or `resolve_image_view` member of `depth_attachment`,
    ///    `stencil_attachment`, or any element of `color_attachments`
    ///  - If the `maintenance7` feature is not enabled or the
    ///    `robust_fragment_shading_rate_attachment_access` limit is [`VK_FALSE`] or the
    ///    `image_view` member of a [`VkRenderingFragmentShadingRateAttachmentInfoKhr`] structure
    ///    was created with [`VkImageSubresourceRange::base_mip_level`] greater than 0, the `next`
    ///    chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, and the `image_view` member of a
    ///    [`VkRenderingFragmentShadingRateAttachmentInfoKhr`] structure included in the `next`
    ///    chain is not [`VK_NULL_HANDLE`], `image_view` must have a width greater than or equal to
    ///    `ceil((render_area.x + render_area.width) / shading_rate_attachment_texel_size.width)`
    ///  - If the `maintenance7` feature is not enabled or the
    ///    `robust_fragment_shading_rate_attachment_access` limit is [`VK_FALSE`] or the
    ///    `image_view` member of a [`VkRenderingFragmentShadingRateAttachmentInfoKhr`] structure
    ///    was created with VkImageSubresourceRange::baseMipLevel greater than 0, the `next` chain
    ///    does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count`
    ///    member is equal to 0 and the `image_view` member of a
    ///    [`VkRenderingFragmentShadingRateAttachmentInfoKhr`] structure included in the `next`
    ///    chain is not [`VK_NULL_HANDLE`], `image_view` must have a height greater than or equal
    ///    to
    ///    `ceil((render_area.y + render_area.height) / shading_rate_attachment_texel_size.height)`
    ///  - If the `maintenance7` feature is not enabled or the
    ///    `robust_fragment_shading_rate_attachment_access` limit is [`VK_FALSE`] or the
    ///    `image_view` member of a [`VkRenderingFragmentShadingRateAttachmentInfoKhr`] structure
    ///    was created with VkImageSubresourceRange::baseMipLevel greater than 0, the `next` chain
    ///    contains a [`VkDeviceGroupRenderPassBeginInfo`] structure, its `device_render_area_count`
    ///    member is not 0, and the `image_view` member of a
    ///    [`VkRenderingFragmentShadingRateAttachmentInfoKhr`] structure included in the `next`
    ///    chain is not [`VK_NULL_HANDLE`], `image_view` must have a width greater than or equal to
    ///    `ceil((device_render_areas.x + device_render_areas.width) /
    ///    shading_rate_attachment_texel_size.width)` for each element of `device_render_areas`
    ///  - If the `maintenance7` feature is not enabled or the
    ///    `robust_fragment_shading_rate_attachment_access` limit is [`VK_FALSE`] or the
    ///    `image_view` member of a [`VkRenderingFragmentShadingRateAttachmentInfoKhr`] structure
    ///    was created with VkImageSubresourceRange::baseMipLevel greater than 0, the `next` chain
    ///    contains a [`VkDeviceGroupRenderPassBeginInfo`] structure, its `device_render_area_count`
    ///    member is not 0, and the `image_view` member of a
    ///    [`VkRenderingFragmentShadingRateAttachmentInfoKhr`] structure included in the `next`
    ///    chain is not [`VK_NULL_HANDLE`], `image_view` must have a height greater than or equal
    ///    to `ceil((device_render_areas.y + device_render_areas.height) /
    ///    shading_rate_attachment_texel_size.height)` for each element of `device_render_areas`
    ///  - If the `image_view` member of a [`VkRenderingFragmentShadingRateAttachmentInfoKhr`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is 0,
    ///    `image_view` must have a `layer_count` that is either equal to 1 or greater than or
    ///    equal to [`[`VkRenderingInfo`]::layer_count`]
    ///  - If the `image_view` member of a [`VkRenderingFragmentShadingRateAttachmentInfoKhr`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is not
    ///    0, `image_view` must have a `layer_count` that is either equal to 1 or greater than the
    ///    index of the most significant bit in `view_mask`
    ///  - If the `image_view` member of a [`VkRenderingFragmentShadingRateAttachmentInfoKhr`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must not be equal
    ///    to the `image_view` or `resolve_image_view` member of `depth_attachment`,
    ///    `stencil_attachment`, or any element of `color_attachments`
    ///  - If the `image_view` member of a [`VkRenderingFragmentShadingRateAttachmentInfoKhr`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must not be equal
    ///    to the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain
    ///  - If the `per_view_render_area_count` member of a
    ///    [`VkMultiviewPerViewRenderAreasRenderPassBeginInfoQcom`] structure included in the
    ///    `next` chain is not 0, then the `multiview_per_view_render_area` feature must be enabled
    ///  - If the `per_view_render_area_count` member of a
    ///    [`VkMultiviewPerViewRenderAreasRenderPassBeginInfoQcom`] structure included in the
    ///    `next` chain is not 0, then `render_area` must specify a render area that includes the
    ///    union of all per view render areas
    ///  - If the `next` chain contains a [`VkRenderPassStripeBeginInfoArm`] structure, the union
    ///    of stripe areas defined by the elements of
    ///    [`VkRenderPassStripeBeginInfoArm::stripe_infos`] must cover the `render_area`
    ///  - If the `image_view` member of a [`VkRenderingFragmentShadingRateAttachmentInfoKhr`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must have been
    ///    created with the identity swizzle
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must have been
    ///    created with the identity swizzle
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`],
    ///    [`VkTileShadingRenderPassFlagQcom::EnableQcom`] must not be included in
    ///    [`VkRenderPassTileShadingCreateInfoQcom::flags`]
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkDeviceGroupRenderPassBeginInfo`], [`VkMultisampledRenderToSingleSampledInfoExt`],
    ///    [`VkMultiviewPerViewAttributesInfoNvx`],
    ///    [`VkMultiviewPerViewRenderAreasRenderPassBeginInfoQcom`],
    ///    [`VkRenderPassPerformanceCountersByRegionBeginInfoArm`],
    ///    [`VkRenderPassStripeBeginInfoArm`], [`VkRenderPassTileShadingCreateInfoQcom`],
    ///    [`VkRenderingFragmentDensityMapAttachmentInfoExt`],
    ///    [`VkRenderingFragmentShadingRateAttachmentInfoKhr`], or [`VkTileMemorySizeInfoQcom`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkRenderingFlag`]s.
    ///
    /// # Valid Usage
    ///  - If `flags` contains [`VkRenderingFlag::CustomResolveExt`] or
    ///    [`VkRenderingFlag::FragmentRegionExt`], then the `custom_resolve` feature must
    ///    enabled
    ///  - For any element of `color_attachments`, `depth_attachment`, or `stencil_attachment`, if
    ///    `resolve_mode` contains [`VkResolveModeFlag::CustomExt`], then `flags` must contain
    ///    [`VkRenderingFlag::CustomResolveExt`]
    ///  - If `flags` contains [`VkRenderingFlag::CustomResolveExt`], then for any element of
    ///    `color_attachments`, `depth_attachment`, or `stencil_attachment`, `resolve_mode` must be
    ///    [`VkResolveModeFlag::CustomExt`] or [`VkResolveModeFlag::None`]
    ///  - If the `fragment_density_map_layered` feature is not enabled, `flags` must not contain
    ///    [`VkRenderingFlag::PerLayerFragmentDensityValve`]
    ///  - If `flags` includes [`VkRenderingFlag::ContentsInlineKhr`] then at least one of the
    ///    following features must be enabled:
    ///    - `maintenance7`
    ///    - `nested_command_buffer`
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkRenderingFlag`] values
    pub flags: VkRenderingFlags,

    /// `render_area` is the render area that is affected by the render pass instance.
    ///
    /// # Valid Usage
    ///  - If [`VkDeviceGroupRenderPassBeginInfo::device_render_area_count`] is 0,
    ///    `render_area.extent.width` must be greater than 0
    ///  - If [`VkDeviceGroupRenderPassBeginInfo::device_render_area_count`] is 0,
    ///    `render_area.extent.height` must be greater than 0
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, `render_area.offset.x` must be greater
    ///    than or equal to 0
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, `render_area.offset.y` must be greater
    ///    than or equal to 0
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, the sum of `render_area.extent.width`
    ///    and `render_area.offset.x` must be less than or equal to `max_framebuffer_width`
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, the sum of `render_area.extent.height`
    ///    and `render_area.offset.y` must be less than or equal to `max_framebuffer_height`
    pub render_area: VkRect2D,

    /// `layer_count` is the number of layers rendered to in each attachment when `view_mask` is 0.
    ///
    /// # Valid Usage
    ///  - If `view_mask` is 0, `layer_count` must not be 0
    ///  - `layer_count` must be less than or equal to `max_framebuffer_layers`
    ///  - If `flags` contains [`VkRenderingFlag::PerLayerFragmentDensityValve`], then
    ///    `layer_count` must be less than or equal to `max_fragment_density_map_layers`
    pub layer_count: u32,

    /// `view_mask` is a bitfield of view indices describing which views are active during
    /// rendering, when it is not 0.
    ///
    /// # Valid Usage
    ///  - If the `multiview` feature is not enabled, `view_mask` must be 0
    ///  - The index of the most significant bit in `view_mask` must be less than
    ///    `max_multiview_view_count`
    pub view_mask: u32,

    /// `color_attachment_count` is the number of elements in `color_attachments`.
    ///
    /// # Valid Usage
    ///  - `color_attachment_count` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_color_attachments`]
    pub color_attachment_count: u32,

    /// `color_attachments` is a pointer to an array of `color_attachment_count`
    /// [`VkRenderingAttachmentInfo`] structures describing any color attachments used.
    ///
    /// # Valid Usage
    ///  - If none of the following are enabled, the `image_view` member of `depth_attachment`,
    ///    `stencil_attachment`, and elements of `color_attachments` that are not
    ///    [`VK_NULL_HANDLE`] must have been created with the same `sample_count`:
    ///    - The [`amd_mixed_attachment_samples`] extension
    ///    - The [`nv_framebuffer_mixed_samples`] extension
    ///    - The `multisampled_render_to_single_sampled` feature
    ///  - `image_view` members of elements of `color_attachments` that are not [`VK_NULL_HANDLE`]
    ///    must have been created with the same `sample_count` , if the
    ///    `multisampled_render_to_single_sampled` feature is not enabled
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by
    ///    `image_view` members of elements of `color_attachments` that are not [`VK_NULL_HANDLE`]
    ///    must have a sample count that is either [`VkSampleCountFlag::_1`] or equal to
    ///    [`VkMultisampledRenderToSingleSampledInfoExt::rasterization_samples`]
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by
    ///    `image_view` members of elements of `color_attachments` that are not [`VK_NULL_HANDLE`]
    ///    and have a sample count of [`VkSampleCountFlag::_1`] must have been created with
    ///    [`VkImageCreateFlag::MultisampledRenderToSingleSampledExt`] in their
    ///    [`VkImageCreateInfo::flags`]
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, the width of the `image_view` member of
    ///    each element of `color_attachments` that is not [`VK_NULL_HANDLE`] must be greater than
    ///    or equal to `render_area.offset.x` + `render_area.extent.width`
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, the height of the `image_view` member
    ///    of each element of `color_attachments` that is not [`VK_NULL_HANDLE`] must be greater
    ///    than or equal to `render_area.offset.y` + `render_area.extent.height`
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the width of the
    ///    `image_view` member of any element of `color_attachments` that is not [`VK_NULL_HANDLE`]
    ///    must be greater than or equal to the sum of the `offset.x` and `extent.width` members of
    ///    each element of `device_render_areas`
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the height of the
    ///    `image_view` member of any element of `color_attachments` that is not [`VK_NULL_HANDLE`]
    ///    must be greater than or equal to the sum of the `offset.y` and `extent.height` members
    ///    of each element of `device_render_areas`
    ///  - If `color_attachment_count` is not 0 and the `image_view` member of an element of
    ///    `color_attachments` is not [`VK_NULL_HANDLE`], that `image_view` must have been created
    ///    with the [`VkImageUsageFlag::ColorAttachment`] usage flag set
    ///  - If `color_attachment_count` is not 0 and there is an element of `color_attachments` with
    ///    either its `resolve_mode` member set to
    ///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`], or its `image_view` member
    ///    not [`VK_NULL_HANDLE`], and its `resolve_mode` member not set to
    ///    [`VkResolveModeFlag::None`], the `resolve_image_view` member of that element of
    ///    `color_attachments` must have been created with the
    ///    [`VkImageUsageFlag::ColorAttachment`] usage flag set
    ///  - If `color_attachment_count` is not 0 and the `image_view` member of an element of
    ///    `color_attachments` is not [`VK_NULL_HANDLE`], the layout member of that element of
    ///    `color_attachments` must not be [`VkImageLayout::DepthStencilAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthStencilReadOnlyOptimal`]
    ///  - If `color_attachment_count` is not 0 and the `image_view` member of an element of
    ///    `color_attachments` is not [`VK_NULL_HANDLE`], if the `resolve_mode` member of that
    ///    element of `color_attachments` is not [`VkResolveModeFlag::None`], its
    ///    `resolve_image_layout` member must not be
    ///    [`VkImageLayout::DepthStencilAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthStencilReadOnlyOptimal`]
    ///  - If `view_mask` is 0, each `color_attachment.image_view` and
    ///    `color_attachment.resolve_image_view` that is not [`VK_NULL_HANDLE`] must have a
    ///    `layer_count` that is greater than or equal to [`VkRenderingInfo::layer_count`]
    ///  - If `view_mask` is not 0, each `color_attachment.image_view` and
    ///    `color_attachment.resolve_image_view` that is not [`VK_NULL_HANDLE`] must have a
    ///    `layer_count` that is greater than the index of the most significant bit in `view_mask`
    ///  - If `color_attachment_count` is not 0 and the `image_view` member of an element of
    ///    `color_attachments` is not [`VK_NULL_HANDLE`], the layout member of that element of
    ///    `color_attachments` must not be [`VkImageLayout::DepthReadOnlyStencilAttachmentOptimal`]
    ///    or [`VkImageLayout::DepthAttachmentStencilReadOnlyOptimal`]
    ///  - If `color_attachment_count` is not 0 and the `image_view` member of an element of
    ///    `color_attachments` is not [`VK_NULL_HANDLE`], if the `resolve_mode` member of that
    ///    element of `color_attachments` is not [`VkResolveModeFlag::None`], its
    ///    `resolve_image_layout` member must not be
    ///    [`VkImageLayout::DepthReadOnlyStencilAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthAttachmentStencilReadOnlyOptimal`]
    ///  - If `color_attachment_count` is not 0 and the `image_view` member of an element of
    ///    `color_attachments` is not [`VK_NULL_HANDLE`], the layout member of that element of
    ///    `color_attachments` must not be [`VkImageLayout::DepthAttachmentOptimal`],
    ///    [`VkImageLayout::DepthReadOnlyOptimal`], [`VkImageLayout::StencilAttachmentOptimal`], or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `color_attachment_count` is not 0 and the `image_view` member of an element of
    ///    `color_attachments` is not [`VK_NULL_HANDLE`], if the `resolve_mode` member of that
    ///    element of `color_attachments` is not [`VkResolveModeFlag::None`], its
    ///    `resolve_image_layout` member must not be [`VkImageLayout::DepthAttachmentOptimal`],
    ///    [`VkImageLayout::DepthReadOnlyOptimal`], [`VkImageLayout::StencilAttachmentOptimal`], or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and the
    ///    `fragment_density_map_non_subsampled_images` feature is not enabled, valid `image_view`
    ///    and `resolve_image_view` members of `depth_attachment`, `stencil_attachment`, and each
    ///    element of `color_attachments` must be a [`VkImageView`] created with
    ///    [`VkImageCreateFlag::SubsampledExt`]
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is not
    ///    0, `image_view` must have a `layer_count` greater than the index of the most significant
    ///    bit in `view_mask`
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is 0,
    ///    `image_view` must have a `layer_count` equal to 1
    ///  - If `color_attachment_count` is not 1, the `resolve_mode` member of any element of
    ///    `color_attachments` must not be
    ///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`]
    ///  - If the `resolve_mode` of any element of `color_attachments` is
    ///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`],
    ///    [`VkRenderingFragmentDensityMapAttachmentInfoExt::image_view`] must be
    ///    [`VK_NULL_HANDLE`]
    ///  - If the `resolve_mode` of any element of `color_attachments` is
    ///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`],
    ///    [`VkRenderingFragmentShadingRateAttachmentInfoKhr::image_view`] must be
    ///    [`VK_NULL_HANDLE`]
    ///  - If `color_attachment_count` is not 0 and the `image_view` member of an element of
    ///    `color_attachments` is not [`VK_NULL_HANDLE`], that `image_view` must have been created
    ///    with the identity swizzle
    ///  - If `color_attachment_count` is not 0, and there is an element of `color_attachments`
    ///    with either its `resolve_mode` member set to
    ///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`], or its `image_view` member
    ///    not set to [`VK_NULL_HANDLE`] and its `resolve_mode` member not set to
    ///    [`VkResolveModeFlag::None`], the `resolve_image_view` member of that element of
    ///    `color_attachments` must have been created with the identity swizzle
    ///  - If the `resolve_mode` of any element of `color_attachments` is
    ///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`],
    ///    [`VkTileShadingRenderPassFlag::EnableQcom`] must not be included in
    ///    [`VkRenderPassTileShadingCreateInfoQcom::flags`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `color_attachment_count` is not 0, `color_attachments` must be a valid pointer to an
    ///    array of `color_attachment_count` valid [`VkRenderingAttachmentInfo`] structures
    pub color_attachments: *const VkRenderingAttachmentInfo,

    /// `depth_attachment` is a pointer to a [`VkRenderingAttachmentInfo`] structure describing a
    /// depth attachment.
    ///
    /// # Valid Usage
    ///  - If none of the following are enabled, the `image_view` member of `depth_attachment`,
    ///    `stencil_attachment`, and elements of `color_attachments` that are not
    ///    [`VK_NULL_HANDLE`] must have been created with the same `sample_count`:
    ///    - The [`amd_mixed_attachment_samples`] extension
    ///    - The [`nv_framebuffer_mixed_samples`] extension
    ///    - The `multisampled_render_to_single_sampled` feature
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by
    ///    `image_view` members of `depth_attachment` that is not [`VK_NULL_HANDLE`] must have a
    ///    sample count that is either [`VkSampleCountFlag::_1`] or equal to
    ///    [`VkMultisampledRenderToSingleSampledInfoExt::rasterization_samples`]
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by
    ///    `image_view` members of `depth_attachment`, `stencil_attachment`, and elements of
    ///    `color_attachments` that are not [`VK_NULL_HANDLE`] and have a sample count of
    ///    [`VkSampleCountFlag::_1`] must have been created with
    ///    [`VkImageCreateFlag::MultisampledRenderToSingleSampledExt`] in their
    ///    [`VkImageCreateInfo::flags`]
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, the width of the `image_view` member of
    ///    `depth_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to
    ///    `render_area.offset.x + render_area.extent.width`
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, the height of the `image_view` member
    ///    of `depth_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to
    ///    `render_area.offset.y + render_area.extent.height`
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the width of the
    ///    `image_view` member of `depth_attachment` that is not [`VK_NULL_HANDLE`] must be greater
    ///    than or equal to the sum of the `offset.x` and `extent.width` members of each element of
    ///    `device_render_areas`
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the height of the
    ///    `image_view` member of any element of `color_attachments`, `depth_attachment`, or
    ///    `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to the
    ///    sum of the `offset.y` and `extent.height` members of each element of
    ///    `device_render_areas`
    ///  - If neither `depth_attachment` or `stencil_attachment` are [`null`] and the `image_view`
    ///    member of either structure is not [`VK_NULL_HANDLE`], the `image_view` member of each
    ///    structure must be the same
    ///  - If neither `depth_attachment` or `stencil_attachment` are [`null`], and the
    ///    `resolve_mode` member of each is not [`VkResolveModeFlag::None`], the
    ///    `resolve_image_view` member of each structure must be the same
    ///  - If `depth_attachment` is not [`null`] and `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `depth_attachment.image_view` must have been created with a format
    ///    that includes a depth component
    ///  - If `depth_attachment` is not [`null`] and `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `depth_attachment.image_view` must have been created with the
    ///    [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If `depth_attachment` is not [`null`] and `depth_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `depth_attachment.resolve_image_view` must have been
    ///    created with the [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If `depth_attachment` is not [`null`] and `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `depth_attachment.layout` must not be
    ///    [`VkImageLayout::ColorAttachmentOptimal`]
    ///  - If `depth_attachment` is not [`null`], `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], and `depth_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `depth_attachment.resolve_image_layout` must not be
    ///    [`VkImageLayout::ColorAttachmentOptimal`]
    ///  - If `view_mask` is 0, each `depth_attachment.image_view` and
    ///    `depth_attachment.resolve_image_view` that is not [`VK_NULL_HANDLE`] must have a
    ///    `layer_count` that is greater than or equal to [`VkRenderingInfo::layer_count`]
    ///  - If `view_mask` is not 0, each `depth_attachment.image_view` and
    ///    `depth_attachment.resolve_image_view` that is not [`VK_NULL_HANDLE`] must have a
    ///    `layer_count` that is greater than the index of the most significant bit in `view_mask`
    ///  - If `depth_attachment` is not [`null`], `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], and `depth_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `depth_attachment.resolve_image_layout` must not be
    ///    [`VkImageLayout::DepthReadOnlyStencilAttachmentOptimal`]
    ///  - If `depth_attachment` is not [`null`] and `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `depth_attachment.layout` must not be
    ///    [`VkImageLayout::StencilAttachmentOptimal`] or [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `depth_attachment` is not [`null`], `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], and `depth_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `depth_attachment.resolve_image_layout` must not be
    ///    [`VkImageLayout::StencilAttachmentOptimal`] or [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `depth_attachment` is not [`null`] and `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `depth_attachment.resolve_mode` must be one of the bits set in
    ///    [`VkPhysicalDeviceDepthStencilResolveProperties::supported_depth_resolve_modes`]
    ///  - If `depth_attachment` or `stencil_attachment` are both not [`null`],
    ///    `depth_attachment.image_view` and `stencil_attachment.image_view` are both not
    ///    [`VK_NULL_HANDLE`], and
    ///    [`VkPhysicalDeviceDepthStencilResolveProperties::independent_resolve_none`] is
    ///    [`VK_FALSE`], the `resolve_mode` of both structures must be the same value
    ///  - If `depth_attachment` or `stencil_attachment` are both not [`null`],
    ///    `depth_attachment.image_view` and `stencil_attachment.image_view` are both not
    ///    [`VK_NULL_HANDLE`],
    ///    [`VkPhysicalDeviceDepthStencilResolveProperties::independent_resolve`] is [`VK_FALSE`],
    ///    and the `resolve_mode` of neither structure is [`VkResolveModeFlag::None`], the
    ///    `resolve_mode` of both structures must be the same value
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and the
    ///    `fragment_density_map_non_subsampled_images` feature is not enabled, valid `image_view`
    ///    and `resolve_image_view` members of `depth_attachment`, `stencil_attachment`, and each
    ///    element of `color_attachments` must be a VkImageView created with
    ///    [`VkImageCreateFlag::SubsampledExt`]
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is not
    ///    0, `image_view` must have a `layer_count` greater than the index of the most significant
    ///    bit in `view_mask`
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is 0,
    ///    `image_view` must have a `layer_count` equal to 1
    ///  - `depth_attachment.resolve_mode` must not be
    ///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`]
    ///  - If `depth_attachment` is not [`null`] and `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `depth_attachment.image_view` must have been created with the
    ///    identity swizzle
    ///  - If `depth_attachment` is not [`null`], `depth_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], and `depth_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `depth_attachment.resolve_image_view` must have been
    ///    created with the identity swizzle
    ///
    /// # Valid Usage (Implicit)
    ///  - If `depth_attachment` is not [`null`], `depth_attachment` must be a valid pointer to a
    ///    valid [`VkRenderingAttachmentInfo`] structure
    pub depth_attachment: *const VkRenderingAttachmentInfo,

    /// `stencil_attachment` is a pointer to a [`VkRenderingAttachmentInfo`] structure describing a
    /// stencil attachment.
    ///
    /// # Valid Usage
    ///  - If none of the following are enabled, the `image_view` member of `depth_attachment`,
    ///    `stencil_attachment`, and elements of `color_attachments` that are not
    ///    [`VK_NULL_HANDLE`] must have been created with the same `sample_count`:
    ///    - The [`amd_mixed_attachment_samples`] extension
    ///    - The [`nv_framebuffer_mixed_samples`] extension
    ///    - The `multisampled_render_to_single_sampled` feature
    ///  - If multisampled-render-to-single-sampled is enabled, the the attachment referenced by
    ///    `image_view` member of `stencil_attachment` that is not [`VK_NULL_HANDLE`] must have a
    ///    sample count that is either [`VkSampleCountFlag::_1`] or equal to
    ///    [`VkMultisampledRenderToSingleSampledInfoExt::rasterization_samples`]
    ///  - If multisampled-render-to-single-sampled is enabled, then the attachment referenced by
    ///    `image_view` member of `stencil_attachment` that is not [`VK_NULL_HANDLE`] and have a
    ///    sample count of [`VkSampleCountFlag::_1`] must have been created with
    ///    [`VkImageCreateFlag::MultisampledRenderToSingleSampledExt`] in their
    ///    [`VkImageCreateInfo::flags`]
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, the width of the `image_view` member
    ///    of `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to
    ///    `render_area.offset.x + render_area.extent.width`
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its
    ///    `device_render_area_count` member is equal to 0, the height of the `image_view` member
    ///    of `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to
    ///    `render_area.offset.y + render_area.extent.height`
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the width of the
    ///    `image_view` member of `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be
    ///    greater than or equal to the sum of the `offset.x` and `extent.width` members of each
    ///    element of `device_render_areas`
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the height of the
    ///    `image_view` member of `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be
    ///    greater than or equal to the sum of the `offset.y` and `extent.height` members of each
    ///    element of `device_render_areas`
    ///  - If neither `depth_attachment` or `stencil_attachment` are [`null`] and the `image_view`
    ///    member of either structure is not [`VK_NULL_HANDLE`], the `image_view` member of each
    ///    structure must be the same
    ///  - If neither `depth_attachment` or `stencil_attachment` are [`null`], and the
    ///    `resolve_mode` member of each is not [`VkResolveModeFlag::None`], the
    ///    `resolve_image_view` member of each structure must be the same
    ///  - If `stencil_attachment` is not [`null`] and `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `stencil_attachment.image_view` must have been created with a format
    ///    that includes a stencil aspect
    ///  - If `stencil_attachment` is not [`null`] and `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `stencil_attachment.image_view` must have been created with a format
    ///    that includes a stencil aspect
    ///  - If `stencil_attachment` is not [`null`] and `stencil_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `stencil_attachment.resolve_image_view` must have been
    ///    created with the [`VkImageUsageFlag::DepthStencilAttachment`] usage flag set
    ///  - If `stencil_attachment` is not [`null`] and `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `stencil_attachment.layout` must not be
    ///    [`VkImageLayout::ColorAttachmentOptimal`]
    ///  - If `stencil_attachment` is not [`null`], `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], and `stencil_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `stencil_attachment.resolve_image_layout` must not be
    ///    [`VkImageLayout::ColorAttachmentOptimal`]
    ///  - If `view_mask` is 0, each `stencil_attachment.image_view` and
    ///    `stencil_attachment.resolve_image_view` that is not [`VK_NULL_HANDLE`] must have a
    ///    `layer_count` that is greater than or equal to [`VkRenderingInfo::layer_count`]
    ///  - If `view_mask` is not 0, each `stencil_attachment.image_view` and
    ///    `stencil_attachment.resolve_image_view` that is not [`VK_NULL_HANDLE`] must have a
    ///    `layer_count` that is greater than the index of the most significant bit in `view_mask`
    ///  - If `stencil_attachment` is not [`null`], `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], and `stencil_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `stencil_attachment.resolve_image_layout` must not be
    ///    [`VkImageLayout::DepthAttachmentStencilReadOnlyOptimal`]
    ///  - If `stencil_attachment` is not [`null`] and `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `stencil_attachment.layout` must not be
    ///    [`VkImageLayout::DepthAttachmentOptimal`] or [`VkImageLayout::DepthReadOnlyOptimal`]
    ///  - If `stencil_attachment` is not [`null`], `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], and `stencil_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `stencil_attachment.resolve_image_layout` must not be
    ///    [`VkImageLayout::DepthAttachmentOptimal`] or [`VkImageLayout::DepthReadOnlyOptimal`]
    ///  - If `stencil_attachment` is not [`null`] and `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `stencil_attachment.resolve_mode` must be one of the bits set in
    ///    [`VkPhysicalDeviceDepthStencilResolveProperties::supported_stencil_resolve_modes`]
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and the
    ///    `fragment_density_map_non_subsampled_images` feature is not enabled, valid `image_view`
    ///    and `resolve_image_view` members of `depth_attachment`, `stencil_attachment`, and each
    ///    element of `color_attachments` must be a VkImageView created with
    ///    [`VkImageCreateFlag::SubsampledExt`]
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is not
    ///    0, `image_view` must have a `layer_count` greater than the index of the most significant
    ///    bit in `view_mask`
    ///  - If the `image_view` member of a [`VkRenderingFragmentDensityMapAttachmentInfoExt`]
    ///    structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is 0,
    ///    `image_view` must have a `layer_count` equal to 1
    ///  - `stencil_attachment.resolve_mode` must not be
    ///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`]
    ///  - If `stencil_attachment` is not [`null`] and `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], `stencil_attachment.image_view` must have been created with the
    ///    identity swizzle
    ///  - If `stencil_attachment` is not [`null`], `stencil_attachment.image_view` is not
    ///    [`VK_NULL_HANDLE`], and `stencil_attachment.resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `stencil_attachment.resolve_image_view` must have been
    ///    created with the identity swizzle
    ///
    /// # Valid Usage (Implicit)
    ///  - If `stencil_attachment` is not [`null`], `stencil_attachment` must be a valid pointer to
    ///    a valid [`VkRenderingAttachmentInfo`] structure
    pub stencil_attachment: *const VkRenderingAttachmentInfo,
}

impl Default for VkRenderingInfo {
    fn default() -> Self {
        VkRenderingInfo {
            r#type: VkStructureType::RenderingInfo,
            next: null(),
            flags: VkRenderingFlags::default(),
            render_area: VkRect2D::default(),
            layer_count: 0,
            view_mask: 0,
            color_attachment_count: 0,
            color_attachments: null(),
            depth_attachment: null(),
            stencil_attachment: null(),
        }
    }
}
