use crate::{VkRect2D, VkRenderingAttachmentInfo, VkRenderingFlags, VkStructureType};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_3, VkImageAspectFlag, VkResolveModeFlag};

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
/// it uses the attachment provided in `color_attachments[x]`. If the imageView member of any
/// element of `color_attachments` is [`VK_NULL_HANDLE`], and `resolve_mode` is not
/// [`VkResolveModeFlag::ExternalFormatDownsampleBitAndroid`], writes to the corresponding location
/// by a fragment are discarded.
///
/// The `aspect_mask` of any image view specified for `depth_attachment` or `stencil_attachment` is
/// ignored. Instead, depth attachments are automatically treated as if
/// [`VkImageAspectFlag::DepthBit`] was specified for their aspect masks, and stencil attachments
/// are automatically treated as if [`VkImageAspectFlag::StencilBit`] was specified for their
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
    /// `r#type` is a VkStructureType value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be VK_STRUCTURE_TYPE_RENDERING_INFO
    pub r#type: VkStructureType,

    /// `next` is NULL or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0 and the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], imageView must have a width greater than or equal to `ceil((render_area.x + render_area.width) / max_fragment_density_texel_size.width)`
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0 and the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], imageView must have a height greater than or equal to `ceil((render_area.y + render_area.height) / max_fragment_density_texel_size.height)`
    ///  - If the `next` chain contains a [`VkDeviceGroupRenderPassBeginInfo`] structure, its `device_render_area_count` member is not 0, and the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], imageView must have a width greater than or equal to `ceil((device_render_areas.x + device_render_areas.width) / max_fragment_density_texel_size.width)` for each element of pDeviceRenderAreas
    ///  - If the `next` chain contains a [`VkDeviceGroupRenderPassBeginInfo`] structure, its `device_render_area_count` member is not 0, and the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], imageView must have a height greater than or equal to `ceil((device_render_areas.y + device_render_areas.height) / max_fragment_density_texel_size.height)` for each element of pDeviceRenderAreas
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must not be equal to the imageView or resolveImageView member of `depth_attachment`, `stencil_attachment`, or any element of `color_attachments`
    ///  - If the maintenance7 feature is not enabled or the robustFragmentShadingRateAttachmentAccess limit is VK_FALSE or the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure was created with VkImageSubresourceRange::baseMipLevel greater than 0, the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, and the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure included in the `next` chain is not [`VK_NULL_HANDLE`], imageView must have a width greater than or equal to `ceil((render_area.x + render_area.width) / shading_rate_attachment_texel_size.width)`
    ///  - If the maintenance7 feature is not enabled or the robustFragmentShadingRateAttachmentAccess limit is VK_FALSE or the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure was created with VkImageSubresourceRange::baseMipLevel greater than 0, the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0 and the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure included in the `next` chain is not [`VK_NULL_HANDLE`], imageView must have a height greater than or equal to `ceil((render_area.y + render_area.height) / shading_rate_attachment_texel_size.height)`
    ///  - If the maintenance7 feature is not enabled or the robustFragmentShadingRateAttachmentAccess limit is VK_FALSE or the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure was created with VkImageSubresourceRange::baseMipLevel greater than 0, the `next` chain contains a [`VkDeviceGroupRenderPassBeginInfo`] structure, its `device_render_area_count` member is not 0, and the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure included in the `next` chain is not [`VK_NULL_HANDLE`], imageView must have a width greater than or equal to `ceil((device_render_areas.x + device_render_areas.width) / shading_rate_attachment_texel_size.width)` for each element of pDeviceRenderAreas
    ///  - If the maintenance7 feature is not enabled or the robustFragmentShadingRateAttachmentAccess limit is VK_FALSE or the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure was created with VkImageSubresourceRange::baseMipLevel greater than 0, the `next` chain contains a [`VkDeviceGroupRenderPassBeginInfo`] structure, its `device_render_area_count` member is not 0, and the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure included in the `next` chain is not [`VK_NULL_HANDLE`], imageView must have a height greater than or equal to `ceil((device_render_areas.y + device_render_areas.height) / shading_rate_attachment_texel_size.height)` for each element of pDeviceRenderAreas
    ///  - If the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is 0, imageView must have a `layer_count` that is either equal to 1 or greater than or equal to VkRenderingInfo::`layer_count`
    ///  - If the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is not 0, imageView must have a `layer_count` that is either equal to 1 or greater than the index of the most significant bit in `view_mask`
    ///  - If the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must not be equal to the imageView or resolveImageView member of `depth_attachment`, `stencil_attachment`, or any element of `color_attachments`
    ///  - If the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must not be equal to the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain
    ///  - If the `per_view_render_area_count` member of a [`VkMultiviewPerViewRenderAreasRenderPassBeginInfoQcom`] structure included in the `next` chain is not 0, then the `multiview_per_view_render_area` feature must be enabled
    ///  - If the `per_view_render_area_count` member of a [`VkMultiviewPerViewRenderAreasRenderPassBeginInfoQcom`] structure included in the `next` chain is not 0, then `render_area` must specify a render area that includes the union of all per view render areas
    ///  - If the `next` chain contains a VkRenderPassStripeBeginInfoARM structure, the union of stripe areas defined by the elements of VkRenderPassStripeBeginInfoARM::pStripeInfos must cover the `render_area`
    ///  - If the imageView member of a VkRenderingFragmentShadingRateAttachmentInfoKHR structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must have been created with the identity swizzle
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], it must have been created with the identity swizzle
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], VK_TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM must not be included in VkRenderPassTileShadingCreateInfoQCOM::`flags`
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be either NULL or a pointer to a valid instance of [`VkDeviceGroupRenderPassBeginInfo`], VkMultisampledRenderToSingleSampledInfoEXT, VkMultiviewPerViewAttributesInfoNVX, [`VkMultiviewPerViewRenderAreasRenderPassBeginInfoQcom`], VkRenderPassPerformanceCountersByRegionBeginInfoARM, VkRenderPassStripeBeginInfoARM, VkRenderPassTileShadingCreateInfoQCOM, VkRenderingFragmentDensityMapAttachmentInfoEXT, VkRenderingFragmentShadingRateAttachmentInfoKHR, or VkTileMemorySizeInfoQCOM
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of VkRenderingFlagBits.
    ///
    /// # Valid Usage
    ///  - If `flags` contains VK_RENDERING_CUSTOM_RESOLVE_BIT_EXT or VK_RENDERING_FRAGMENT_REGION_BIT_EXT, then the customResolve feature must enabled
    ///  - For any element of `color_attachments`, `depth_attachment`, or `stencil_attachment`, if `resolve_mode` contains [`VkResolveModeFlag::CUSTOM_BIT_EXT, then `flags` must contain VK_RENDERING_CUSTOM_RESOLVE_BIT_EXT
    ///  - If `flags` contains VK_RENDERING_CUSTOM_RESOLVE_BIT_EXT, then for any element of `color_attachments`, `depth_attachment`, or `stencil_attachment`, `resolve_mode` must be [`VkResolveModeFlag::CUSTOM_BIT_EXT or [`VkResolveModeFlag::NONE
    ///  - If the fragmentDensityMapLayered feature is not enabled, `flags` must not contain VK_RENDERING_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE
    ///  - If `flags` includes VK_RENDERING_CONTENTS_INLINE_BIT_KHR then at least one of the following features must be enabled:
    ///    - maintenance7
    ///    - nestedCommandBuffer
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of VkRenderingFlagBits values
    pub flags: VkRenderingFlags,

    /// `render_area` is the render area that is affected by the render pass instance.
    ///
    /// # Valid Usage
    ///  - If [`VkDeviceGroupRenderPassBeginInfo`]::`device_render_area_count` is 0, `render_area`.extent.width must be greater than 0
    ///  - If [`VkDeviceGroupRenderPassBeginInfo`]::`device_render_area_count` is 0, `render_area`.extent.height must be greater than 0
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, `render_area`.offset.x must be greater than or equal to 0
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, `render_area`.offset.y must be greater than or equal to 0
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, the sum of `render_area`.extent.width and `render_area`.offset.x must be less than or equal to maxFramebufferWidth
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, the sum of `render_area`.extent.height and `render_area`.offset.y must be less than or equal to maxFramebufferHeight
    pub render_area: VkRect2D,

    /// `layer_count` is the number of layers rendered to in each attachment when `view_mask` is 0.
    ///
    /// # Valid Usage
    ///  - If `view_mask` is 0, `layer_count` must not be 0
    ///  - `layer_count` must be less than or equal to maxFramebufferLayers
    ///  - If `flags` contains VK_RENDERING_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE, then `layer_count` must be less than or equal to maxFragmentDensityMapLayers
    pub layer_count: u32,

    /// `view_mask` is a bitfield of view indices describing which views are active during rendering, when it is not 0.
    ///
    /// # Valid Usage
    ///  - If the multiview feature is not enabled, `view_mask` must be 0
    ///  - The index of the most significant bit in `view_mask` must be less than maxMultiviewViewCount
    pub view_mask: u32,

    /// `color_attachment_count` is the number of elements in `color_attachments`.
    ///
    /// # Valid Usage
    ///  - `color_attachment_count` must be less than or equal to VkPhysicalDeviceLimits::maxColorAttachments
    pub color_attachment_count: u32,

    /// `color_attachments` is a pointer to an array of `color_attachment_count` VkRenderingAttachmentInfo structures describing any color attachments used.
    ///
    /// # Valid Usage
    ///  - If none of the following are enabled, the imageView member of `depth_attachment`, `stencil_attachment`, and elements of `color_attachments` that are not [`VK_NULL_HANDLE`] must have been created with the same sampleCount:
    ///    - The VK_AMD_mixed_attachment_samples extension
    ///    - The VK_NV_framebuffer_mixed_samples extension
    ///    - The multisampledRenderToSingleSampled feature
    ///  - imageView members of elements of `color_attachments` that are not [`VK_NULL_HANDLE`] must have been created with the same sampleCount , if the multisampledRenderToSingleSampled feature is not enabled
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of `depth_attachment`, `stencil_attachment`, and elements of `color_attachments` that are not [`VK_NULL_HANDLE`] must have a sample count that is either VK_SAMPLE_COUNT_1_BIT or equal to VkMultisampledRenderToSingleSampledInfoEXT::rasterizationSamples
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of `depth_attachment`, `stencil_attachment`, and elements of `color_attachments` that are not [`VK_NULL_HANDLE`] and have a sample count of VK_SAMPLE_COUNT_1_BIT must have been created with VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT in their VkImageCreateInfo::`flags`
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, the width of the imageView member of each element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to `render_area`.offset.x + `render_area`.extent.width
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, the height of the imageView member of each element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to `render_area`.offset.y + `render_area`.extent.height
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the width of the imageView member of any element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to the sum of the offset.x and extent.width members of each element of pDeviceRenderAreas
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the height of the imageView member of any element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to the sum of the offset.y and extent.height members of each element of pDeviceRenderAreas
    ///  - If `color_attachment_count` is not 0 and the imageView member of an element of `color_attachments` is not [`VK_NULL_HANDLE`], that imageView must have been created with the VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT usage flag set
    ///  - If `color_attachment_count` is not 0 and there is an element of `color_attachments` with either its `resolve_mode` member set to [`VkResolveModeFlag::ExternalFormatDownsampleBitAndroid`], or its imageView member not [`VK_NULL_HANDLE`], and its `resolve_mode` member not set to [`VkResolveModeFlag::NONE, the resolveImageView member of that element of `color_attachments` must have been created with the VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT usage flag set
    ///  - If `color_attachment_count` is not 0 and the imageView member of an element of `color_attachments` is not [`VK_NULL_HANDLE`], the layout member of that element of `color_attachments` must not be VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL or VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL
    ///  - If `color_attachment_count` is not 0 and the imageView member of an element of `color_attachments` is not [`VK_NULL_HANDLE`], if the `resolve_mode` member of that element of `color_attachments` is not [`VkResolveModeFlag::NONE, its resolveImageLayout member must not be VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL or VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL
    ///  - If `view_mask` is 0, each pColorAttachment->imageView and pColorAttachment->resolveImageView that is not [`VK_NULL_HANDLE`] must have a `layer_count` that is greater than or equal to VkRenderingInfo::`layer_count`
    ///  - If `view_mask` is not 0, each pColorAttachment->imageView and pColorAttachment->resolveImageView that is not [`VK_NULL_HANDLE`] must have a `layer_count` that is greater than the index of the most significant bit in `view_mask`
    ///  - If `color_attachment_count` is not 0 and the imageView member of an element of `color_attachments` is not [`VK_NULL_HANDLE`], the layout member of that element of `color_attachments` must not be VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL or VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL
    ///  - If `color_attachment_count` is not 0 and the imageView member of an element of `color_attachments` is not [`VK_NULL_HANDLE`], if the `resolve_mode` member of that element of `color_attachments` is not [`VkResolveModeFlag::NONE, its resolveImageLayout member must not be VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL or VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL
    ///  - If `color_attachment_count` is not 0 and the imageView member of an element of `color_attachments` is not [`VK_NULL_HANDLE`], the layout member of that element of `color_attachments` must not be VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL, VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL, VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL, or VK_IMAG_LAYOUT_STENCIL_READ_ONLY_OPTIMAL
    ///  - If `color_attachment_count` is not 0 and the imageView member of an element of `color_attachments` is not [`VK_NULL_HANDLE`], if the `resolve_mode` member of that element of `color_attachments` is not [`VkResolveModeFlag::NONE, its resolveImageLayout member must not be VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL, VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL, VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL, or VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], and the fragmentDensityMapNonSubsampledImages feature is not enabled, valid imageView and resolveImageView members of `depth_attachment`, `stencil_attachment`, and each element of `color_attachments` must be a VkImageView created with VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is not 0, imageView must have a `layer_count` greater than the index of the most significant bit in `view_mask`
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is 0, imageView must have a `layer_count` equal to 1
    ///  - If `color_attachment_count` is not 1, the `resolve_mode` member of any element of `color_attachments` must not be [`VkResolveModeFlag::ExternalFormatDownsampleBitAndroid`]
    ///  - If the `resolve_mode` of any element of `color_attachments` is [`VkResolveModeFlag::ExternalFormatDownsampleBitAndroid`], VkRenderingFragmentDensityMapAttachmentInfoEXT::imageView must be [`VK_NULL_HANDLE`]
    ///  - If the `resolve_mode` of any element of `color_attachments` is [`VkResolveModeFlag::ExternalFormatDownsampleBitAndroid`], VkRenderingFragmentShadingRateAttachmentInfoKHR::imageView must be [`VK_NULL_HANDLE`]
    ///  - If `color_attachment_count` is not 0 and the imageView member of an element of `color_attachments` is not [`VK_NULL_HANDLE`], that imageView must have been created with the identity swizzle
    ///  - If `color_attachment_count` is not 0, and there is an element of `color_attachments` with either its `resolve_mode` member set to [`VkResolveModeFlag::ExternalFormatDownsampleBitAndroid`], or its imageView member not set to [`VK_NULL_HANDLE`] and its `resolve_mode` member not set to [`VkResolveModeFlag::NONE, the resolveImageView member of that element of `color_attachments` must have been created with the identity swizzle
    ///  - If the `resolve_mode` of any element of `color_attachments` is [`VkResolveModeFlag::ExternalFormatDownsampleBitAndroid`], VK_TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM must not be included in VkRenderPassTileShadingCreateInfoQCOM::`flags`
    ///
    /// # Valid Usage (Implicit)
    ///  - If `color_attachment_count` is not 0, `color_attachments` must be a valid pointer to an array of `color_attachment_count` valid VkRenderingAttachmentInfo structures
    pub color_attachments: *const VkRenderingAttachmentInfo,

    /// `depth_attachment` is a pointer to a VkRenderingAttachmentInfo structure describing a depth attachment.
    ///
    /// # Valid Usage
    ///  - If none of the following are enabled, the imageView member of `depth_attachment`, `stencil_attachment`, and elements of `color_attachments` that are not [`VK_NULL_HANDLE`] must have been created with the same sampleCount:
    ///    - The VK_AMD_mixed_attachment_samples extension
    ///    - The VK_NV_framebuffer_mixed_samples extension
    ///    - The multisampledRenderToSingleSampled feature
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of `depth_attachment`, `stencil_attachment`, and elements of `color_attachments` that are not [`VK_NULL_HANDLE`] must have a sample count that is either VK_SAMPLE_COUNT_1_BIT or equal to VkMultisampledRenderToSingleSampledInfoEXT::rasterizationSamples
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of `depth_attachment`, `stencil_attachment`, and elements of `color_attachments` that are not [`VK_NULL_HANDLE`] and have a sample count of VK_SAMPLE_COUNT_1_BIT must have been created with VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT in their VkImageCreateInfo::`flags`
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, the width of the imageView member of each element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to `render_area`.offset.x + `render_area`.extent.width
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, the height of the imageView member of each element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to `render_area`.offset.y + `render_area`.extent.height
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the width of the imageView member of any element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to the sum of the offset.x and extent.width members of each element of pDeviceRenderAreas
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the height of the imageView member of any element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to the sum of the offset.y and extent.height members of each element of pDeviceRenderAreas
    ///  - If neither `depth_attachment` or `stencil_attachment` are NULL and the imageView member of either structure is not [`VK_NULL_HANDLE`], the imageView member of each structure must be the same
    ///  - If neither `depth_attachment` or `stencil_attachment` are NULL, and the `resolve_mode` member of each is not [`VkResolveModeFlag::NONE, the resolveImageView member of each structure must be the same
    ///  - If `depth_attachment` is not NULL and `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], `depth_attachment`->imageView must have been created with a format that includes a depth component
    ///  - If `depth_attachment` is not NULL and `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], `depth_attachment`->imageView must have been created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage flag set
    ///  - If `depth_attachment` is not NULL and `depth_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `depth_attachment`->resolveImageView must have been created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage flag set
    ///  - If `depth_attachment` is not NULL and `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], `depth_attachment`->layout must not be VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL
    ///  - If `depth_attachment` is not NULL, `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], and `depth_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `depth_attachment`->resolveImageLayout must not be VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL
    ///  - If `view_mask` is 0, each `depth_attachment`->imageView and `depth_attachment`->resolveImageView that is not [`VK_NULL_HANDLE`] must have a `layer_count` that is greater than or equal to VkRenderingInfo::`layer_count`
    ///  - If `view_mask` is not 0, each `depth_attachment`->imageView and `depth_attachment`->resolveImageView that is not [`VK_NULL_HANDLE`] must have a `layer_count` that is greater than the index of the most significant bit in `view_mask`
    ///  - If `depth_attachment` is not NULL, `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], and `depth_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `depth_attachment`->resolveImageLayout must not be VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL
    ///  - If `depth_attachment` is not NULL and `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], `depth_attachment`->layout must not be VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL or VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL
    ///  - If `depth_attachment` is not NULL, `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], and `depth_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `depth_attachment`->resolveImageLayout must not be VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL or VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL
    ///  - If `depth_attachment` is not NULL and `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], `depth_attachment`->`resolve_mode` must be one of the bits set in VkPhysicalDeviceDepthStencilResolveProperties::supportedDepthResolveModes
    ///  - If `depth_attachment` or `stencil_attachment` are both not NULL, `depth_attachment`->imageView and `stencil_attachment`->imageView are both not [`VK_NULL_HANDLE`], and VkPhysicalDeviceDepthStencilResolveProperties::independentResolveNone is VK_FALSE, the `resolve_mode` of both structures must be the same value
    ///  - If `depth_attachment` or `stencil_attachment` are both not NULL, pDepthAttachmet->imageView and `stencil_attachment`->imageView are both not [`VK_NULL_HANDLE`], VkPhysicalDeviceDepthStencilResolveProperties::independentResolve is VK_FALSE, and the `resolve_mode` of neither structure is [`VkResolveModeFlag::NONE, the `resolve_mode` of both structures must be the same value
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], and the fragmentDensityMapNonSubsampledImages feature is not enabled, valid imageView and resolveImageView members of `depth_attachment`, `stencil_attachment`, and each element of `color_attachments` must be a VkImageView created with VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is not 0, imageView must have a `layer_count` greater than the index of the most significant bit in `view_mask`
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is 0, imageView must have a `layer_count` equal to 1
    ///  - `depth_attachment`->`resolve_mode` must not be [`VkResolveModeFlag::ExternalFormatDownsampleBitAndroid`]
    ///  - If `depth_attachment` is not NULL and `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], `depth_attachment`->imageView must have been created with the identity swizzle
    ///  - If `depth_attachment` is not NULL, `depth_attachment`->imageView is not [`VK_NULL_HANDLE`], and `depth_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `depth_attachment`->resolveImageView must have been created with the identity swizzle
    ///
    /// # Valid Usage (Implicit)
    ///  - If `depth_attachment` is not NULL, `depth_attachment` must be a valid pointer to a valid VkRenderingAttachmentInfo structure
    pub depth_attachment: *const VkRenderingAttachmentInfo,

    /// `stencil_attachment` is a pointer to a VkRenderingAttachmentInfo structure describing a stencil attachment.
    ///
    /// # Valid Usage
    ///  - If none of the following are enabled, the imageView member of `depth_attachment`, `stencil_attachment`, and elements of `color_attachments` that are not [`VK_NULL_HANDLE`] must have been created with the same sampleCount:
    ///    - The VK_AMD_mixed_attachment_samples extension
    ///    - The VK_NV_framebuffer_mixed_samples extension
    ///    - The multisampledRenderToSingleSampled feature
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of `depth_attachment`, `stencil_attachment`, and elements of `color_attachments` that are not [`VK_NULL_HANDLE`] must have a sample count that is either VK_SAMPLE_COUNT_1_BIT or equal to VkMultisampledRenderToSingleSampledInfoEXT::rasterizationSamples
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of `depth_attachment`, `stencil_attachment`, and elements of `color_attachments` that are not [`VK_NULL_HANDLE`] and have a sample count of VK_SAMPLE_COUNT_1_BIT must have been created with VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT in their VkImageCreateInfo::`flags`
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, the width of the imageView member of each element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to `render_area`.offset.x + `render_area`.extent.width
    ///  - If the `next` chain does not contain [`VkDeviceGroupRenderPassBeginInfo`] or its `device_render_area_count` member is equal to 0, the height of the imageView member of each element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to `render_area`.offset.y + `render_area`.extent.height
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the width of the imageView member of any element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to the sum of the offset.x and extent.width members of each element of pDeviceRenderAreas
    ///  - If the `next` chain contains [`VkDeviceGroupRenderPassBeginInfo`], the height of the imageView member of any element of `color_attachments`, `depth_attachment`, or `stencil_attachment` that is not [`VK_NULL_HANDLE`] must be greater than or equal to the sum of the offset.y and extent.height members of each element of pDeviceRenderAreas
    ///  - If neither `depth_attachment` or `stencil_attachment` are NULL and the imageView member of either structure is not [`VK_NULL_HANDLE`], the imageView member of each structure must be the same
    ///  - If neither `depth_attachment` or `stencil_attachment` are NULL, and the `resolve_mode` member of each is not [`VkResolveModeFlag::NONE, the resolveImageView member of each structure must be the same
    ///  - If `stencil_attachment` is not NULL and `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], `stencil_attachment`->imageView must have been created with a format that includes a stencil aspect
    ///  - If `stencil_attachment` is not NULL and `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], `stencil_attachment`->imageView must have been created with a format that includes a stencil aspect
    ///  - If `stencil_attachment` is not NULL and `stencil_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `stencil_attachment`->resolveImageView must have been created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage flag set
    ///  - If `stencil_attachment` is not NULL and `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], `stencil_attachment`->layout must not be VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL
    ///  - If `stencil_attachment` is not NULL, `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], and `stencil_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `stencil_attachment`->resolveImageLayout must not be VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL
    ///  - If `view_mask` is 0, each `stencil_attachment`->imageView and `stencil_attachment`->resolveImageView that is not [`VK_NULL_HANDLE`] must have a `layer_count` that is greater than or equal to VkRenderingInfo::`layer_count`
    ///  - If `view_mask` is not 0, each `stencil_attachment`->imageView and `stencil_attachment`->resolveImageView that is not [`VK_NULL_HANDLE`] must have a `layer_count` that is greater than the index of the most significant bit in `view_mask`
    ///  - If `stencil_attachment` is not NULL, `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], and `stencil_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `stencil_attachment`->resolveImageLayout must not be VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL
    ///  - If `stencil_attachment` is not NULL and `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], `stencil_attachment`->layout must not be VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL or VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL
    ///  - If `stencil_attachment` is not NULL, `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], and `stencil_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `stencil_attachment`->resolveImageLayout must not be VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL or VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL
    ///  - If `stencil_attachment` is not NULL and `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], `stencil_attachment`->`resolve_mode` must be one of the bits set in VkPhysicalDeviceDepthStencilResolveProperties::supportedStencilResolveModes
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], and the fragmentDensityMapNonSubsampledImages feature is not enabled, valid imageView and resolveImageView members of `depth_attachment`, `stencil_attachment`, and each element of `color_attachments` must be a VkImageView created with VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is not 0, imageView must have a `layer_count` greater than the index of the most significant bit in `view_mask`
    ///  - If the imageView member of a VkRenderingFragmentDensityMapAttachmentInfoEXT structure included in the `next` chain is not [`VK_NULL_HANDLE`], and `view_mask` is 0, imageView must have a `layer_count` equal to 1
    ///  - `stencil_attachment`->`resolve_mode` must not be [`VkResolveModeFlag::ExternalFormatDownsampleBitAndroid`]
    ///  - If `stencil_attachment` is not NULL and `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], `stencil_attachment`->imageView must have been created with the identity swizzle
    ///  - If `stencil_attachment` is not NULL, `stencil_attachment`->imageView is not [`VK_NULL_HANDLE`], and `stencil_attachment`->`resolve_mode` is not [`VkResolveModeFlag::NONE, `stencil_attachment`->resolveImageView must have been created with the identity swizzle
    ///
    /// # Valid Usage (Implicit)
    ///  - If `stencil_attachment` is not NULL, `stencil_attachment` must be a valid pointer to a valid VkRenderingAttachmentInfo structure
    pub stencil_attachment: *const VkRenderingAttachmentInfo,
}
