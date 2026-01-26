use crate::{
    VkComponentMapping, VkFormat, VkImage, VkImageSubresourceRange, VkImageViewCreateFlags,
    VkImageViewType, VkStructureType,
};
use std::{os::raw::c_void, ptr::null};

/// Structure specifying parameters of a newly created image view
///
/// # Description
/// Some of the image creation parameters are inherited by the view. In particular, image view creation inherits the implicit parameter usage specifying the allowed usages of the image view that, by default, takes the value of the corresponding usage parameter specified in VkImageCreateInfo at image creation time. The implicit usage can be overridden by adding a VkImageViewUsageCreateInfo structure to the pNext chain, but the view usage must be a subset of the image usage. If image has a depth-stencil format and was created with a VkImageStencilUsageCreateInfo structure included in the pNext chain of VkImageCreateInfo, the usage is calculated based on the subresource.aspectMask provided:
///  - If aspectMask includes only VK_IMAGE_ASPECT_STENCIL_BIT, the implicit usage is equal to VkImageStencilUsageCreateInfo::stencilUsage.
///  - If aspectMask includes only VK_IMAGE_ASPECT_DEPTH_BIT, the implicit usage is equal to VkImageCreateInfo::usage.
///  - If both aspects are included in aspectMask, the implicit usage is equal to the intersection of VkImageCreateInfo::usage and VkImageStencilUsageCreateInfo::stencilUsage.
///
/// If image is a 3D image, its Z range can be restricted to a subset by adding a VkImageViewSlicedCreateInfoEXT to the pNext chain.
///
/// If image was created with the VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT flag, and if the format of the image is not multi-planar format can be different from the image’s format, but if image was created without the VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT flag and they are not equal they must be compatible. Image format compatibility is defined in the Format Compatibility Classes section. Views of compatible formats will have the same mapping between texel coordinates and memory locations irrespective of the format, with only the interpretation of the bit pattern changing.
///
/// If image was created with a multi-planar format, and the image view’s aspectMask is one of VK_IMAGE_ASPECT_PLANE_0_BIT, VK_IMAGE_ASPECT_PLANE_1_BIT or VK_IMAGE_ASPECT_PLANE_2_BIT, the view’s aspect mask is considered to be equivalent to VK_IMAGE_ASPECT_COLOR_BIT when used as a framebuffer attachment.
///
/// If image was created with the VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT flag, format must be compatible with the image’s format as described above; or must be an uncompressed format, in which case it must be size-compatible with the image’s format. In this case, the resulting image view’s texel dimensions equal the dimensions of the selected mip level divided by the compressed texel block size and rounded up.
///
/// The VkComponentMapping components member describes a remapping from components of the image to components of the vector returned by shader image instructions. This remapping must be the identity swizzle for any VkImageView used with a combined image sampler that enables sampler Y′CBCR conversion, input attachment descriptors, framebuffer attachments, and storage image descriptors.
///
/// If the image view is to be used with a sampler which supports sampler Y′CBCR conversion, an identically defined object of type VkSamplerYcbcrConversion to that used to create the sampler must be passed to vkCreateImageView in a VkSamplerYcbcrConversionInfo included in the pNext chain of VkImageViewCreateInfo. Conversely, if a VkSamplerYcbcrConversion object is passed to vkCreateImageView, an identically defined VkSamplerYcbcrConversion object must be used when sampling the image.
///
/// If the image has a multi-planar format, subresourceRange.aspectMask is VK_IMAGE_ASPECT_COLOR_BIT, and usage includes VK_IMAGE_USAGE_SAMPLED_BIT, then the format must be identical to the image format and the sampler to be used with the image view must enable sampler Y′CBCR conversion.
///
/// When such an image is used in a video coding operation, the sampler Y′CBCR conversion has no effect.
///
/// If image was created with the VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT and the image has a multi-planar format, and if subresourceRange.aspectMask is VK_IMAGE_ASPECT_PLANE_0_BIT, VK_IMAGE_ASPECT_PLANE_1_BIT, or VK_IMAGE_ASPECT_PLANE_2_BIT, format must be compatible with the corresponding plane of the image, and the sampler to be used with the image view must not enable sampler Y′CBCR conversion. The width and height of the single-plane image view must be derived from the multi-planar image’s dimensions in the manner listed for plane compatibility for the plane.
///
/// Any view of an image plane will have the same mapping between texel coordinates and memory locations as used by the components of the color aspect, subject to the formulae relating texel coordinates to lower-resolution planes as described in Chroma Reconstruction. That is, if an R or B plane has a reduced resolution relative to the G plane of the multi-planar image, the image view operates using the (uplane, vplane) unnormalized coordinates of the reduced-resolution plane, and these coordinates access the same memory locations as the (ucolor, vcolor) unnormalized coordinates of the color aspect for which chroma reconstruction operations operate on the same (uplane, vplane) or (iplane, jplane) coordinates.
///
/// # Valid Usage
///  - The format features of the resultant image view must contain at least one bit
///  - If usage contains VK_IMAGE_USAGE_SAMPLED_BIT, then the format features of the resultant image view must contain VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
///  - If usage contains VK_IMAGE_USAGE_STORAGE_BIT, then the image view’s format features must contain VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
///  - If usage contains VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT, then the image view’s format features must contain VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT or VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV
///  - If usage contains VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, then the image view’s format features must contain VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT
///  - If usage contains VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR, then the image view’s format features must contain VK_FORMAT_FEATURE_VIDEO_DECODE_OUTPUT_BIT_KHR
///  - If usage contains VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR, then the image view’s format features must contain VK_FORMAT_FEATURE_VIDEO_DECODE_DPB_BIT_KHR
///  - usage must not include VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR
///  - If usage contains VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR, then the image view’s format features must contain VK_FORMAT_FEATURE_VIDEO_ENCODE_INPUT_BIT_KHR
///  - If usage contains VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR, then the image view’s format features must contain VK_FORMAT_FEATURE_VIDEO_ENCODE_DPB_BIT_KHR
///  - usage must not include VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR
///  - If usage contains VK_IMAGE_USAGE_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR, then the image view’s format features must contain VK_FORMAT_FEATURE_2_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR
///  - If usage contains VK_IMAGE_USAGE_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR, then the image view’s format features must contain VK_FORMAT_FEATURE_2_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR
///  - If usage contains VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT, and any of the following is true, then the image view’s format features must contain at least one of VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT or VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT or VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV
///    - the externalFormatResolve feature is not enabled
///    - the nullColorAttachmentWithExternalFormatResolve property is VK_FALSE
///    - image was created with an VkExternalFormatANDROID::externalFormat value of 0
///  - If the attachmentFragmentShadingRate feature is enabled, and the usage for the image view includes VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR, then the image view’s format features must contain VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR
///  - If viewType is VK_IMAGE_VIEW_TYPE_1D, VK_IMAGE_VIEW_TYPE_2D, or VK_IMAGE_VIEW_TYPE_3D; and subresourceRange.layerCount is VK_REMAINING_ARRAY_LAYERS, then the remaining number of layers must be 1
///  - If viewType is VK_IMAGE_VIEW_TYPE_CUBE and subresourceRange.layerCount is VK_REMAINING_ARRAY_LAYERS, the remaining number of layers must be 6
///  - If viewType is VK_IMAGE_VIEW_TYPE_CUBE_ARRAY and subresourceRange.layerCount is VK_REMAINING_ARRAY_LAYERS, the remaining number of layers must be a multiple of 6
///  - If flags includes VK_IMAGE_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT, the descriptorBufferCaptureReplay feature must be enabled
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageViewCreateInfo {
    /// sType is a VkStructureType value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - sType must be VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO
    pub r#type: VkStructureType,

    /// pNext is NULL or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the image view requires a sampler Y′CBCR conversion and usage contains VK_IMAGE_USAGE_SAMPLED_BIT, then the pNext chain must include a VkSamplerYcbcrConversionInfo structure with a conversion value other than VK_NULL_HANDLE
    ///  - If image has an Android external format, the pNext chain must include a VkSamplerYcbcrConversionInfo structure with a conversion object created with the same external format as image
    ///  - If image has an QNX Screen external format, the pNext chain must include a VkSamplerYcbcrConversionInfo structure with a conversion object created with the same external format as image
    ///  - If the pNext chain includes a VkImageViewUsageCreateInfo structure, and image was not created with a VkImageStencilUsageCreateInfo structure included in the pNext chain of VkImageCreateInfo, its usage member must not include any bits that were not set in the usage member of the VkImageCreateInfo structure used to create image
    ///  - If the pNext chain includes a VkImageViewUsageCreateInfo structure, image was created with a VkImageStencilUsageCreateInfo structure included in the pNext chain of VkImageCreateInfo, and subresourceRange.aspectMask includes VK_IMAGE_ASPECT_STENCIL_BIT, the usage member of the VkImageViewUsageCreateInfo structure must not include any bits that were not set in the usage member of the VkImageStencilUsageCreateInfo structure used to create image
    ///  - If the pNext chain includes a VkImageViewUsageCreateInfo structure, image was created with a VkImageStencilUsageCreateInfo structure included in the pNext chain of VkImageCreateInfo, and subresourceRange.aspectMask includes bits other than VK_IMAGE_ASPECT_STENCIL_BIT, the usage member of the VkImageViewUsageCreateInfo structure must not include any bits that were not set in the usage member of the VkImageCreateInfo structure used to create image
    ///  - If the pNext chain includes a VkExportMetalObjectCreateInfoEXT structure, its exportObjectType member must be VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then textureSampleWeighted feature must be enabled
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure then VkImageViewSampleWeightCreateInfoQCOM::filterSize.height must be less than or equal to VkPhysicalDeviceImageProcessingPropertiesQCOM::maxWeightFilterDimension.height
    ///
    /// # Valid Usage (Implicit)
    ///  - Each pNext member of any structure (including this one) in the pNext chain must be either NULL or a pointer to a valid instance of VkExportMetalObjectCreateInfoEXT, VkImageViewASTCDecodeModeEXT, VkImageViewMinLodCreateInfoEXT, VkImageViewSampleWeightCreateInfoQCOM, VkImageViewSlicedCreateInfoEXT, VkImageViewUsageCreateInfo, VkOpaqueCaptureDescriptorDataCreateInfoEXT, or VkSamplerYcbcrConversionInfo
    ///  - The sType value of each structure in the pNext chain must be unique, with the exception of structures of type VkExportMetalObjectCreateInfoEXT
    pub next: *const c_void,

    /// flags is a bitmask of VkImageViewCreateFlagBits specifying additional parameters of the image view.
    ///
    /// # Valid Usage
    ///  - If the fragmentDensityMapDynamic feature is not enabled, flags must not contain VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT
    ///  - If the fragmentDensityMapDeferred feature is not enabled, flags must not contain VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT
    ///  - If flags contains VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT, flags must not contain VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT
    ///  - If flags does not contain VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT, and image was created with the VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT usage flag set, its flags must not contain any of VK_IMAGE_CREATE_PROTECTED_BIT, VK_IMAGE_CREATE_SPARSE_BINDING_BIT, VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT, or VK_IMAGE_CREATE_SPARSE_ALIASED_BIT
    ///  - If the pNext chain includes a VkOpaqueCaptureDescriptorDataCreateInfoEXT structure, flags must contain VK_IMAGE_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT
    ///
    /// # Valid Usage (Implicit)
    ///  - flags must be a valid combination of VkImageViewCreateFlagBits values
    pub flags: VkImageViewCreateFlags,

    /// image is a VkImage on which the view will be created.
    ///
    /// # Valid Usage
    ///  - image must have been created with a usage value containing at least one of the following:
    ///    - [`VK_IMAGE_USAGE_SAMPLED_BIT`]
    ///    - [`VK_IMAGE_USAGE_STORAGE_BIT`]
    ///    - [`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`]
    ///    - [`VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`]
    ///    - [`VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`]
    ///    - [`VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT`]
    ///    - [`VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
    ///    - [`VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT`]
    ///    - [`VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`]
    ///    - [`VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR`]
    ///    - [`VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`]
    ///    - [`VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`]
    ///    - [`VK_IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM`]
    ///    - [`VK_IMAGE_USAGE_SAMPLE_BLOCK_MATCH_BIT_QCOM`]
    ///    - [`VK_IMAGE_USAGE_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR`]
    ///    - [`VK_IMAGE_USAGE_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR`]
    ///  - If format has a _422 or _420 suffix then image must have been created with a width that is a multiple of 2
    ///  - If format has a _420 suffix then image must have been created with a height that is a multiple of 2
    ///  - If image is non-sparse then the image or each specified disjoint plane must be bound completely and contiguously to a single VkDeviceMemory object
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then image must have been created with the VK_IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM usage flag set
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure and if viewType is VK_IMAGE_VIEW_TYPE_1D_ARRAY, then image must have been created with imageType VK_IMAGE_TYPE_1D
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_1D_ARRAY, then image must have been created with width equal to or greater than \((numPhases \times \mathbin{max}\left( \mathbin{align}\left(filterSize.width,4\right), filterSize.height\right))\)
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure and if viewType is VK_IMAGE_VIEW_TYPE_2D_ARRAY, then image must have been created with imageType VK_IMAGE_TYPE_2D
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_2D_ARRAY, then image must have been created with width equal to or greater than filterSize.width
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_2D_ARRAY, then image must have been created with height equal to or greater than filterSize.height
    ///
    /// # Valid Usage (Implicit)
    ///  - image must be a valid VkImage handle
    pub image: VkImage,

    /// viewType is a VkImageViewType value specifying the type of the image view.
    ///
    /// # Valid Usage
    ///  - If image was not created with VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT then viewType must not be VK_IMAGE_VIEW_TYPE_CUBE or VK_IMAGE_VIEW_TYPE_CUBE_ARRAY
    ///  - If the imageCubeArray feature is not enabled, viewType must not be VK_IMAGE_VIEW_TYPE_CUBE_ARRAY
    ///  - If image was created with VK_IMAGE_TYPE_3D but without VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT set then viewType must not be VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///  - If image was created with VK_IMAGE_TYPE_3D but without VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT or VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT set, then viewType must not be VK_IMAGE_VIEW_TYPE_2D
    ///  - If image was created with a samples value not equal to VK_SAMPLE_COUNT_1_BIT then viewType must be either VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///  - viewType must be compatible with the type of image as shown in the view type compatibility table
    ///  - If image was created with the VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR usage flag set, viewType must be VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///  - If image was created with the VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR usage flag set, VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR, or VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR, then the viewType must be VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///  - If image was created with the VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR usage flag set, VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR, or VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR, then the viewType must be VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///  - If image was created with the VK_IMAGE_USAGE_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR or VK_IMAGE_USAGE_VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR usage flags set, then viewType must be VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///  - If the pNet chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then viewType must be VK_IMAGE_VIEW_TYPE_1D_ARRAY or VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///
    /// # Valid Usage (Implicit)
    ///  - viewType must be a valid VkImageViewType value
    pub view_type: VkImageViewType,

    /// format is a VkFormat specifying the format and type used to interpret texel blocks of the image.
    ///
    /// # Valid Usage
    ///  - If image was created with the VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT flag, but without the VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT flag, and if the format of the image is not a multi-planar format, format must be compatible with the format used to create image, as defined in Format Compatibility Classes
    ///  - If image was created with the VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT flag, format must be compatible with, or must be an uncompressed format that is size-compatible with, the format used to create image
    ///  - If a VkImageFormatListCreateInfo structure was included in the pNext chain of the VkImageCreateInfo structure used when creating image and VkImageFormatListCreateInfo::viewFormatCount is not zero then format must be one of the formats in VkImageFormatListCreateInfo::pViewFormats
    ///  - If image was created with the VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT flag, if the format of the image is a multi-planar format, and if subresourceRange.aspectMask is one of the multi-planar aspect mask bits, then format must be compatible with the VkFormat for the plane of the image format indicated by subresourceRange.aspectMask, as defined in Compatible Formats of Planes of Multi-Planar Formats
    ///  - If image was not created with the VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT flag, or if the format of the image is a multi-planar format and if subresourceRange.aspectMask is VK_IMAGE_ASPECT_COLOR_BIT, format must be identical to the format used to create image
    ///  - If the pNext chain includes a VkSamplerYcbcrConversionInfo structure with a conversion value other than VK_NULL_HANDLE, format must be the same used in VkSamplerYcbcrConversionCreateInfo::format
    ///  - If image has an Android external format, format must be VK_FORMAT_UNDEFINED
    ///  - If image has an QNX Screen external format, format must be VK_FORMAT_UNDEFINED
    ///  - If the shadingRateImage feature is enabled, and image was created with the VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV usage flag set, format must be VK_FORMAT_R8_UINT
    ///  - If the invocationMask feature is enabled, and image was created with the VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI usage flag set, format must be VK_FORMAT_R8_UINT
    ///  - If the VK_KHR_portability_subset extension is enabled, and VkPhysicalDevicePortabilitySubsetFeaturesKHR::imageViewFormatReinterpretation is VK_FALSE, the VkFormat in format must not contain a different number of components, or a different number of bits in each component, than the format of the VkImage in image
    ///  - If Vulkan 1.3 is not supported and the ycbcr2plane444Formats feature is not enabled, format must not be VK_FORMAT_G8_B8R8_2PLANE_444_UNORM, VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16, VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16, or VK_FORMAT_G16_B16R16_2PLANE_444_UNORM
    ///
    /// # Valid Usage (Implicit)
    ///  - format must be a valid VkFormat value
    pub format: VkFormat,

    /// components is a VkComponentMapping structure specifying a remapping of color components (or of depth or stencil components after they have been converted into color components).
    ///
    /// # Valid Usage
    ///  - If the pNext chain includes a VkSamplerYcbcrConversionInfo structure with a conversion value other than VK_NULL_HANDLE, all members of components must have the identity swizzle
    ///  - If image has an Android external format, all members of components must be the identity swizzle
    ///  - If image has an QNX Screen external format, all members of components must be the identity swizzle
    ///  - If the VK_KHR_portability_subset extension is enabled, and VkPhysicalDevicePortabilitySubsetFeaturesKHR::imageViewFormatSwizzle is VK_FALSE, all elements of components must have the identity swizzle
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then components must be VK_COMPONENT_SWIZZLE_IDENTITY for all components
    ///
    /// # Valid Usage (Implicit)
    ///  - components must be a valid VkComponentMapping structure
    pub components: VkComponentMapping,

    /// subresourceRange is a VkImageSubresourceRange structure selecting the set of mipmap levels and array layers to be accessible to the view.
    ///
    /// # Valid Usage
    ///  - If image was created with VK_IMAGE_TYPE_3D and viewType is VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY then subresourceRange.levelCount must be 1
    ///  - subresourceRange.baseMipLevel must be less than the mipLevels specified in VkImageCreateInfo when image was created
    ///  - If subresourceRange.levelCount is not VK_REMAINING_MIP_LEVELS, subresourceRange.baseMipLevel + subresourceRange.levelCount must be less than or equal to the mipLevels specified in VkImageCreateInfo when image was created
    ///  - If image was created with the VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT usage flag set, subresourceRange.levelCount must be 1
    ///  - If image is not a 3D image created with VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT or VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT set, or viewType is not VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY, subresourceRange.baseArrayLayer must be less than the arrayLayers specified in VkImageCreateInfo when image was created
    ///  - If subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, image is not a 3D image created with VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT or VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT set, or viewType is not VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY, subresourceRange.layerCount must be non-zero and subresourceRange.baseArrayLayer + subresourceRange.layerCount must be less than or equal to the arrayLayers specified in VkImageCreateInfo when image was created
    ///  - If image is a 3D image created with VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT set, and viewType is VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY, subresourceRange.baseArrayLayer must be less than the depth computed from baseMipLevel and extent.depth specified in VkImageCreateInfo when image was created, according to the formula defined in Image Mip Level Sizing
    ///  - If subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, image is a 3D image created with VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT set, and viewType is VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY, subresourceRange.layerCount must be non-zero and subresourceRange.baseArrayLayer + subresourceRange.layerCount must be less than or equal to the depth computed from baseMipLevel and extent.depth specified in VkImageCreateInfo when image was created, according to the formula defined in Image Mip Level Sizing
    ///  - If image was created with the VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT flag and format is a non-compressed format, the levelCount member of subresourceRange must be 1
    ///  - If image was created with the VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT flag, the VkPhysicalDeviceMaintenance6Properties::blockTexelViewCompatibleMultipleLayers property is not VK_TRUE, and format is a non-compressed format, then the layerCount member of subresourceRange must be 1
    ///  - subresourceRange.aspectMask must only have at most 1 valid multi-planar aspect mask bit
    ///  - If the attachmentFragmentShadingRate feature is enabled, the usage for the image view includes VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR, and layeredShadingRateAttachments is VK_FALSE, subresourceRange.layerCount must be 1
    ///  - If image was created with flags containing VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT and the VK_IMAGE_USAGE_SAMPLED_BIT usage flag set, subresourceRange.layerCount must be less than or equal to VkPhysicalDeviceFragmentDensityMap2PropertiesEXT::maxSubsampledArrayLayers
    ///  - If viewType is VK_IMAGE_VIEW_TYPE_1D, VK_IMAGE_VIEW_TYPE_2D, or VK_IMAGE_VIEW_TYPE_3D; and subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, then subresourceRange.layerCount must be 1
    ///  - If viewType is VK_IMAGE_VIEW_TYPE_CUBE and subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, subresourceRange.layerCount must be 6
    ///  - If viewType is VK_IMAGE_VIEW_TYPE_CUBE_ARRAY and subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, subresourceRange.layerCount must be a multiple of 6
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then subresourceRange.aspectMask must be VK_IMAGE_ASPECT_COLOR_BIT
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then subresourceRange.levelCount must be 1
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_1D_ARRAY, then subresourceRange.layerCount must be equal to 2
    ///  - If the pNext chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_2D_ARRAY, then subresourceRange.layerCount must be equal or greater than numPhases
    ///  - subresourceRange.aspectMask must be valid for the format the image was created with
    ///
    /// # Valid Usage (Implicit)
    ///  - subresourceRange must be a valid VkImageSubresourceRange structure
    pub subresource_range: VkImageSubresourceRange,
}

impl Default for VkImageViewCreateInfo {
    fn default() -> Self {
        VkImageViewCreateInfo {
            r#type: VkStructureType::ImageViewCreateInfo,
            next: null(),
            flags: VkImageViewCreateFlags::new(),
            image: VkImage::null(),
            view_type: VkImageViewType::a,
            format: VkFormat::Undefined,
            components: VkComponentMapping::default(),
            subresource_range: VkImageSubresourceRange::default(),
        }
    }
}
