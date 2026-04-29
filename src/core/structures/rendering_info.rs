use crate::{VkRect2D, VkRenderingAttachmentInfo, VkRenderingFlags, VkStructureType};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_3;

/// Structure specifying render pass instance begin info
///
/// # Description
/// If viewMask is not 0, multiview is enabled.
///
/// If there is an instance of VkDeviceGroupRenderPassBeginInfo included in the pNext chain and its deviceRenderAreaCount member is not 0, then renderArea is ignored, and the render area is defined per-device by that structure.
///
/// If multiview is enabled, and the multiviewPerViewRenderAreas feature is enabled, and there is an instance of VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM included in the pNext chain with perViewRenderAreaCount not equal to 0, then the elements of VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM::pPerViewRenderAreas override renderArea and define a render area for each view. In this case, renderArea must be an area at least as large as the union of all the per-view render areas.
///
/// Each element of the pColorAttachments array corresponds to an output location in the shader, i.e. if the shader declares an output variable decorated with a Location value of X, then it uses the attachment provided in pColorAttachments[X]. If the imageView member of any element of pColorAttachments is VK_NULL_HANDLE, and resolveMode is not VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID, writes to the corresponding location by a fragment are discarded.
///
/// The aspectMask of any image view specified for pDepthAttachment or pStencilAttachment is ignored. Instead, depth attachments are automatically treated as if VK_IMAGE_ASPECT_DEPTH_BIT was specified for their aspect masks, and stencil attachments are automatically treated as if VK_IMAGE_ASPECT_STENCIL_BIT was specified for their aspect masks.
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkRenderingInfo {
    /// sType is a VkStructureType value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - sType must be VK_STRUCTURE_TYPE_RENDERING_INFO
    pub r#type: VkStructureType,

    /// pNext is NULL or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each pNext member of any structure (including this one) in the pNext chain must be either NULL or a pointer to a valid instance of VkDeviceGroupRenderPassBeginInfo, VkMultisampledRenderToSingleSampledInfoEXT, VkMultiviewPerViewAttributesInfoNVX, VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM, VkRenderPassPerformanceCountersByRegionBeginInfoARM, VkRenderPassStripeBeginInfoARM, VkRenderPassTileShadingCreateInfoQCOM, VkRenderingFragmentDensityMapAttachmentInfoEXT, VkRenderingFragmentShadingRateAttachmentInfoKHR, or VkTileMemorySizeInfoQCOM
    ///  - The sType value of each structure in the pNext chain must be unique
    pub next: *const c_void,

    /// flags is a bitmask of VkRenderingFlagBits.
    ///
    /// # Valid Usage (Implicit)
    ///  - flags must be a valid combination of VkRenderingFlagBits values
    pub flags: VkRenderingFlags,

    /// renderArea is the render area that is affected by the render pass instance.
    ///
    /// # Valid Usage
    ///  - If VkDeviceGroupRenderPassBeginInfo::deviceRenderAreaCount is 0, renderArea.extent.width must be greater than 0
    ///  - If VkDeviceGroupRenderPassBeginInfo::deviceRenderAreaCount is 0, renderArea.extent.height must be greater than 0
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, renderArea.offset.x must be greater than or equal to 0
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, renderArea.offset.y must be greater than or equal to 0
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, the sum of renderArea.extent.width and renderArea.offset.x must be less than or equal to maxFramebufferWidth
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, the sum of renderArea.extent.height and renderArea.offset.y must be less than or equal to maxFramebufferHeight
    pub render_area: VkRect2D,

    /// layerCount is the number of layers rendered to in each attachment when viewMask is 0.
    ///
    /// # Valid Usage
    ///  - If viewMask is 0, layerCount must not be 0
    pub layer_count: u32,

    /// viewMask is a bitfield of view indices describing which views are active during rendering, when it is not 0.
    pub view_mask: u32,

    /// colorAttachmentCount is the number of elements in pColorAttachments.
    pub color_attachment_count: u32,

    /// pColorAttachments is a pointer to an array of colorAttachmentCount VkRenderingAttachmentInfo structures describing any color attachments used.
    ///
    /// # Valid Usage
    ///  - If none of the following are enabled, the imageView member of pDepthAttachment, pStencilAttachment, and elements of pColorAttachments that are not VK_NULL_HANDLE must have been created with the same sampleCount:
    ///    - The VK_AMD_mixed_attachment_samples extension
    ///    - The VK_NV_framebuffer_mixed_samples extension
    ///    - The multisampledRenderToSingleSampled feature
    ///  - imageView members of elements of pColorAttachments that are not VK_NULL_HANDLE must have been created with the same sampleCount , if the multisampledRenderToSingleSampled feature is not enabled
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of pDepthAttachment, pStencilAttachment, and elements of pColorAttachments that are not VK_NULL_HANDLE must have a sample count that is either VK_SAMPLE_COUNT_1_BIT or equal to VkMultisampledRenderToSingleSampledInfoEXT::rasterizationSamples
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of pDepthAttachment, pStencilAttachment, and elements of pColorAttachments that are not VK_NULL_HANDLE and have a sample count of VK_SAMPLE_COUNT_1_BIT must have been created with VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT in their VkImageCreateInfo::flags
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, the width of the imageView member of each element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to renderArea.offset.x + renderArea.extent.width
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, the height of the imageView member of each element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to renderArea.offset.y + renderArea.extent.height
    ///  - If the pNext chain contains VkDeviceGroupRenderPassBeginInfo, the width of the imageView member of any element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to the sum of the offset.x and extent.width members of each element of pDeviceRenderAreas
    ///  - If the pNext chain contains VkDeviceGroupRenderPassBeginInfo, the height of the imageView member of any element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to the sum of the offset.y and extent.height members of each element of pDeviceRenderAreas
    ///  - If colorAttachmentCount is not 0 and the imageView member of an element of pColorAttachments is not VK_NULL_HANDLE, that imageView must have been created with the VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT usage flag set
    ///  - If colorAttachmentCount is not 0 and there is an element of pColorAttachments with either its resolveMode member set to VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID, or its imageView member not VK_NULL_HANDLE, and its resolveMode member not set to VK_RESOLVE_MODE_NONE, the resolveImageView member of that element of pColorAttachments must have been created with the VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT usage flag set
    ///  - If colorAttachmentCount is not 0 and the imageView member of an element of pColorAttachments is not VK_NULL_HANDLE, the layout member of that element of pColorAttachments must not be VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL or VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL
    ///
    /// # Valid Usage (Implicit)
    ///  - If colorAttachmentCount is not 0, pColorAttachments must be a valid pointer to an array of colorAttachmentCount valid VkRenderingAttachmentInfo structures
    pub color_attachments: *const VkRenderingAttachmentInfo,

    /// pDepthAttachment is a pointer to a VkRenderingAttachmentInfo structure describing a depth attachment.
    ///
    /// # Valid Usage
    ///  - If none of the following are enabled, the imageView member of pDepthAttachment, pStencilAttachment, and elements of pColorAttachments that are not VK_NULL_HANDLE must have been created with the same sampleCount:
    ///    - The VK_AMD_mixed_attachment_samples extension
    ///    - The VK_NV_framebuffer_mixed_samples extension
    ///    - The multisampledRenderToSingleSampled feature
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of pDepthAttachment, pStencilAttachment, and elements of pColorAttachments that are not VK_NULL_HANDLE must have a sample count that is either VK_SAMPLE_COUNT_1_BIT or equal to VkMultisampledRenderToSingleSampledInfoEXT::rasterizationSamples
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of pDepthAttachment, pStencilAttachment, and elements of pColorAttachments that are not VK_NULL_HANDLE and have a sample count of VK_SAMPLE_COUNT_1_BIT must have been created with VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT in their VkImageCreateInfo::flags
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, the width of the imageView member of each element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to renderArea.offset.x + renderArea.extent.width
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, the height of the imageView member of each element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to renderArea.offset.y + renderArea.extent.height
    ///  - If the pNext chain contains VkDeviceGroupRenderPassBeginInfo, the width of the imageView member of any element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to the sum of the offset.x and extent.width members of each element of pDeviceRenderAreas
    ///  - If the pNext chain contains VkDeviceGroupRenderPassBeginInfo, the height of the imageView member of any element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to the sum of the offset.y and extent.height members of each element of pDeviceRenderAreas
    ///  - If neither pDepthAttachment or pStencilAttachment are NULL and the imageView member of either structure is not VK_NULL_HANDLE, the imageView member of each structure must be the same
    ///  - If neither pDepthAttachment or pStencilAttachment are NULL, and the resolveMode member of each is not VK_RESOLVE_MODE_NONE, the resolveImageView member of each structure must be the same
    ///  - If pDepthAttachment is not NULL and pDepthAttachment->imageView is not VK_NULL_HANDLE, pDepthAttachment->imageView must have been created with a format that includes a depth component
    ///  - If pDepthAttachment is not NULL and pDepthAttachment->imageView is not VK_NULL_HANDLE, pDepthAttachment->imageView must have been created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage flag set
    ///  - If pDepthAttachment is not NULL and pDepthAttachment->resolveMode is not VK_RESOLVE_MODE_NONE, pDepthAttachment->resolveImageView must have been created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage flag set
    ///
    /// # Valid Usage (Implicit)
    ///  - If pDepthAttachment is not NULL, pDepthAttachment must be a valid pointer to a valid VkRenderingAttachmentInfo structure
    pub depth_attachment: *const VkRenderingAttachmentInfo,

    /// pStencilAttachment is a pointer to a VkRenderingAttachmentInfo structure describing a stencil attachment.
    ///
    /// # Valid Usage
    ///  - If none of the following are enabled, the imageView member of pDepthAttachment, pStencilAttachment, and elements of pColorAttachments that are not VK_NULL_HANDLE must have been created with the same sampleCount:
    ///    - The VK_AMD_mixed_attachment_samples extension
    ///    - The VK_NV_framebuffer_mixed_samples extension
    ///    - The multisampledRenderToSingleSampled feature
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of pDepthAttachment, pStencilAttachment, and elements of pColorAttachments that are not VK_NULL_HANDLE must have a sample count that is either VK_SAMPLE_COUNT_1_BIT or equal to VkMultisampledRenderToSingleSampledInfoEXT::rasterizationSamples
    ///  - If multisampled-render-to-single-sampled is enabled, then all attachments referenced by imageView members of pDepthAttachment, pStencilAttachment, and elements of pColorAttachments that are not VK_NULL_HANDLE and have a sample count of VK_SAMPLE_COUNT_1_BIT must have been created with VK_IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT in their VkImageCreateInfo::flags
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, the width of the imageView member of each element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to renderArea.offset.x + renderArea.extent.width
    ///  - If the pNext chain does not contain VkDeviceGroupRenderPassBeginInfo or its deviceRenderAreaCount member is equal to 0, the height of the imageView member of each element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to renderArea.offset.y + renderArea.extent.height
    ///  - If the pNext chain contains VkDeviceGroupRenderPassBeginInfo, the width of the imageView member of any element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to the sum of the offset.x and extent.width members of each element of pDeviceRenderAreas
    ///  - If the pNext chain contains VkDeviceGroupRenderPassBeginInfo, the height of the imageView member of any element of pColorAttachments, pDepthAttachment, or pStencilAttachment that is not VK_NULL_HANDLE must be greater than or equal to the sum of the offset.y and extent.height members of each element of pDeviceRenderAreas
    ///  - If neither pDepthAttachment or pStencilAttachment are NULL and the imageView member of either structure is not VK_NULL_HANDLE, the imageView member of each structure must be the same
    ///  - If neither pDepthAttachment or pStencilAttachment are NULL, and the resolveMode member of each is not VK_RESOLVE_MODE_NONE, the resolveImageView member of each structure must be the same
    ///  - If pStencilAttachment is not NULL and pStencilAttachment->imageView is not VK_NULL_HANDLE, pStencilAttachment->imageView must have been created with a format that includes a stencil aspect
    ///  - If pStencilAttachment is not NULL and pStencilAttachment->imageView is not VK_NULL_HANDLE, pStencilAttachment->imageView must have been created with a format that includes a stencil aspect
    ///  - If pStencilAttachment is not NULL and pStencilAttachment->resolveMode is not VK_RESOLVE_MODE_NONE, pStencilAttachment->resolveImageView must have been created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage flag set
    ///
    /// # Valid Usage (Implicit)
    ///  - If pStencilAttachment is not NULL, pStencilAttachment must be a valid pointer to a valid VkRenderingAttachmentInfo structure
    pub stencil_attachment: *const VkRenderingAttachmentInfo,
}
