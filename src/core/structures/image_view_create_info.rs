use crate::{
    VkComponentMapping, VkFormat, VkImage, VkImageSubresourceRange, VkImageViewCreateFlags,
    VkImageViewType, VkStructureType,
};
use std::{os::raw::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_NULL_HANDLE, VkImageAspectFlag, VkImageUsageFlag, VkImageView,
    VkImageViewCreateFlag,
};

/// Structure specifying parameters of a newly created image view
///
/// # Description
/// Some of the image creation parameters are inherited by the view. In particular, image view
/// creation inherits the implicit parameter usage specifying the allowed usages of the image view
/// that, by default, takes the value of the corresponding usage parameter specified in
/// [`VkImageCreateInfo`] at image creation time. The implicit usage can be overridden by adding a
/// [`VkImageViewUsageCreateInfo`] structure to the `next` chain, but the view usage must be a
/// subset of the image usage. If image has a depth-stencil format and was created with a
/// [`VkImageStencilUsageCreateInfo`] structure included in the `next` chain of
/// [`VkImageCreateInfo`], the usage is calculated based on the `subresource.aspect_mask` provided:
///  - If `aspect_mask` includes only [`VkImageAspectFlag::StencilBit`], the implicit usage is
///    equal to [`VkImageStencilUsageCreateInfo::stencil_usage`].
///  - If `aspect_mask` includes only [`VkImageAspectFlag::DepthBit`], the implicit usage is equal
///    to [`VkImageCreateInfo::usage`].
///  - If both aspects are included in `aspect_mask`, the implicit usage is equal to the
///    intersection of [`VkImageCreateInfo::usage`] and
///    [`VkImageStencilUsageCreateInfo::stencil_usage`].
///
/// If `image` is a 3D image, its Z range can be restricted to a subset by adding a
/// [`VkImageViewSlicedCreateInfoExt`] to the `next` chain.
///
/// If `image` was created with the [`VkImageCreateFlag::MutableFormatBit`] flag, and if the format
/// of the image is not multi-planar format can be different from the image’s format, but if image
/// was created without the [`VkImageCreateFlag::BlockTexelViewCompatibleBit`] flag and they are
/// not equal they must be compatible. Views of compatible formats will have the same mapping
/// between texel coordinates and memory locations irrespective of the format, with only the
/// interpretation of the bit pattern changing.
///
/// If `image` was created with a multi-planar format, and the image view’s `aspect_mask` is one of
/// [`VkImageAspectFlag::Plane0Bit`], [`VkImageAspectFlag::Plane1Bit`] or
/// [`VkImageAspectFlag::Plane2Bit`], the view’s aspect mask is considered to be equivalent to
/// [`VkImageAspectFlag::ColorBit`] when used as a framebuffer attachment.
///
/// If `image` was created with the [`VkImageCreateFlag::BlockTexelViewCompatibleBit`] flag,
/// `format` must be compatible with the image’s format as described above; or must be an
/// uncompressed format, in which case it must be size-compatible with the image’s format. In this
/// case, the resulting image view’s texel dimensions equal the dimensions of the selected mip
/// level divided by the compressed texel block size and rounded up.
///
/// The [`VkComponentMapping`] components member describes a remapping from components of the image
/// to components of the vector returned by shader image instructions. This remapping must be the
/// identity swizzle for any [`VkImageView`] used with a combined image sampler that enables
/// sampler Y′CBCR conversion, input attachment descriptors, framebuffer attachments, and storage
/// image descriptors.
///
/// If the image view is to be used with a sampler which supports sampler Y′CBCR conversion, an
/// identically defined object of type [`VkSamplerYcbcrConversion`] to that used to create the
/// sampler must be passed to [`VkCreateImageView`] in a [`VkSamplerYcbcrConversionInfo`] included
/// in the `next` chain of [`VkImageViewCreateInfo`]. Conversely, if a [`VkSamplerYcbcrConversion`]
/// object is passed to [`VkCreateImageView`], an identically defined [`VkSamplerYcbcrConversion`]
/// object must be used when sampling the image.
///
/// If the image has a multi-planar format, `subresource_range.aspect_mask` is
/// [`VkImageAspectFlag::ColorBit`], and usage includes [`VkImageUsageFlag::SampledBit`], then the
/// format must be identical to the image format and the sampler to be used with the image view
/// must enable sampler Y′CBCR conversion.
///
/// When such an image is used in a video coding operation, the sampler Y′CBCR conversion has no
/// effect.
///
/// If image was created with the [`VkImageCreateFlag::MutableFormatBit`] and the image has a
/// multi-planar format, and if `subresource_range.aspect_mask` is
/// [`VkImageAspectFlag::Plane0Bit`], [`VkImageAspectFlag::Plane1Bit`], or
/// [`VkImageAspectFlag::Plane2Bit`], format must be compatible with the corresponding plane of the
/// image, and the sampler to be used with the image view must not enable sampler Y′CBCR
/// conversion. The width and height of the single-plane image view must be derived from the
/// multi-planar image’s dimensions in the manner listed for plane compatibility for the plane.
///
/// Any view of an image plane will have the same mapping between texel coordinates and memory
/// locations as used by the components of the color aspect, subject to the formulae relating texel
/// coordinates to lower-resolution planes as described in Chroma Reconstruction. That is, if an R
/// or B plane has a reduced resolution relative to the G plane of the multi-planar image, the
/// image view operates using the (uplane, vplane) unnormalized coordinates of the
/// reduced-resolution plane, and these coordinates access the same memory locations as the
/// (ucolor, vcolor) unnormalized coordinates of the color aspect for which chroma reconstruction
/// operations operate on the same (uplane, vplane) or (iplane, jplane) coordinates.
///
/// # Valid Usage
///  - The format features of the resultant image view must contain at least one bit
///  - If `usage` contains [`VkImageUsageFlag::SampledBit`], then the format features of the
///    resultant image view must contain [`VkFormatFeatureFlag::SampledImageBit`]
///  - If `usage` contains [`VkImageUsageFlag::StorageBit`], then the image view’s format features
///    must contain [`VkFormatFeatureFlag::StorageImageBit`]
///  - If `usage` contains [`VkImageUsageFlag::ColorAttachmentBit`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::ColorAttachmentBit`] or
///    [`VkFormatFeatureFlag::_2LinearColorAttachmentBitNv`]
///  - If `usage` contains [`VkImageUsageFlag::DepthStencilAttachmentBit`], then the image view’s
///    format features must contain [`VkFormatFeatureFlag::DepthStencilAttachmentBit`]
///  - If `usage` contains [`VkImageUsageFlag::VideoDecodeDstBitKhr`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::VideoDecodeOutputBitKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoDecodeDpbBitKhr`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::VideoDecodeDpbBitKhr`]
///  - `usage` must not include [`VkImageUsageFlag::VideoDecodeSrcBitKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoEncodeSrcBitKhr`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::VideoEncodeInputBitKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoEncodeDpbBitKhr`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::VideoEncodeDpbBitKhr`]
///  - `usage` must not include [`VkImageUsageFlag::VideoEncodeDstBitKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoEncodeQuantizationDeltaMapBitKhr`], then the
///    image view’s format features must contain
///    [`VkFormatFeatureFlag::_2VideoEncodeQuantizationDeltaMapBitKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoEncodeEmphasisMapBitKhr`], then the image
///    view’s format features must contain [`VkFormatFeatureFlag::_2VideoEncodeEmphasisMapBitKhr`]
///  - If `usage` contains [`VkImageUsageFlag::InputAttachmentBit`], and any of the following is
///    true, then the image view’s format features must contain at least one of
///    [`VkFormatFeatureFlag::ColorAttachmentBit`] or
///    [`VkFormatFeatureFlag::DepthStencilAttachmentBit`] or
///    [`VkFormatFeatureFlag::_2LinearColorAttachmentBitNv`]
///    - the `external_format_resolve` feature is not enabled
///    - the `null_color_attachment_with_external_format_resolve` property is [`VK_FALSE`]
///    - `image` was created with an [`VkExternalFormatAndroid::external_format`] value of 0
///  - If the `attachment_fragment_shading_rate` feature is enabled, and the usage for the image
///    view includes [`VkImageUsageFlag::FragmentShadingRateAttachmentBitKhr`] then the image
///    view’s format features must contain
///    [`VkFormatFeatureFlag::FragmentShadingRateAttachmentBitKhr`]
///  - If `view_type` is [`VkImageViewType::_1d`], [`VkImageViewType::_2d`], or
///    [`VkImageViewType::_3d`]; and `subresource_range.layer_count` is
///    [`VK_REMAINING_ARRAY_LAYERS`], then the remaining number of layers must be 1
///  - If `view_type` is [`VkImageViewType::Cube`] and `subresource_range.layer_count` is
///    [`VK_REMAINING_ARRAY_LAYERS`], the remaining number of layers must be 6
///  - If `view_type` is [`VkImageViewType::CubeArray`] and `subresource_range.layer_count` is
///    [`VK_REMAINING_ARRAY_LAYERS`], the remaining number of layers must be a multiple of 6
///  - If `flags` includes [`VkImageViewCreateFlag::DescriptorBufferCaptureReplayBitExt`], the
///    `descriptor_buffer_capture_replay` feature must be enabled
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageViewCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::ImageViewCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the image view requires a sampler Y′CBCR conversion and `usage` contains
    ///    [`VkImageUsageFlag::SampledBit`], then the `next` chain must include a
    ///    [`VkSamplerYcbcrConversionInfo`] structure with a conversion value other than
    ///    [`VK_NULL_HANDLE`]
    ///  - If `image` has an Android external format, the `next` chain must include a
    ///    [`VkSamplerYcbcrConversionInfo`] structure with a conversion object created with the
    ///    same external format as `image`
    ///  - If `image` has an QNX Screen external format, the `next` chain must include a
    ///    [`VkSamplerYcbcrConversionInfo`] structure with a conversion object created with the
    ///    same external format as `image`
    ///  - If the `next` chain includes a [`VkImageViewUsageCreateInfo`] structure, and `image` was
    ///    not created with a [`VkImageStencilUsageCreateInfo`] structure included in the `next`
    ///    chain of [`VkImageCreateInfo`], its usage member must not include any bits that were not
    ///    set in the `usage` member of the [`VkImageCreateInfo`] structure used to create `image`
    ///  - If the `next` chain includes a [`VkImageViewUsageCreateInfo`] structure, `image` was
    ///    created with a [`VkImageStencilUsageCreateInfo`] structure included in the `next` chain
    ///    of [`VkImageCreateInfo`], and `subresource_range.aspect_mask` includes
    ///    [`VkImageAspectFlag::StencilBit`], the `usage` member of the
    ///    [`VkImageViewUsageCreateInfo`] structure must not include any bits that were not set in
    ///    the `usage` member of the [`VkImageStencilUsageCreateInfo`] structure used to create
    ///    `image`
    ///  - If the `next` chain includes a [`VkImageViewUsageCreateInfo`] structure, `image` was
    ///    created with a [`VkImageStencilUsageCreateInfo`] structure included in the `next` chain
    ///    of [`VkImageCreateInfo`], and `subresource_range.aspect_mask` includes bits other than
    ///    [`VkImageAspectFlag::StencilBit`], the usage member of the
    ///    [`VkImageViewUsageCreateInfo`] structure must not include any bits that were not set in
    ///    the `usage` member of the [`VkImageCreateInfo`] structure used to create `image`
    ///  - If the `next` chain includes a [`VkExportMetalObjectCreateInfoExt`] structure, its
    ///    `export_object_type` member must be [`VkExportMetalObjectTypeFlag::MetalTextureBitExt`]
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure, then
    ///    `texture_sample_weighted` feature must be enabled
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure then
    ///    `VkImageViewSampleWeightCreateInfoQcom::filter_size.height` must be less than or equal
    ///    to `VkPhysicalDeviceImageProcessingPropertiesQcom::max_weight_filter_dimension.height`
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkExportMetalObjectCreateInfoExt`], [`VkImageViewASTCDecodeModeExt`],
    ///    [`VkImageViewMinLodCreateInfoExt`], [`VkImageViewSampleWeightCreateInfoQcom`],
    ///    [`VkImageViewSlicedCreateInfoExt`], [`VkImageViewUsageCreateInfo`],
    ///    [`VkOpaqueCaptureDescriptorDataCreateInfoExt`], or [`VkSamplerYcbcrConversionInfo`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique, with the
    ///    exception of structures of type [`VkExportMetalObjectCreateInfoExt`]
    pub next: *const c_void,

    /// flags is a bitmask of VkImageViewCreateFlagBits specifying additional parameters of the image view.
    ///
    /// # Valid Usage
    ///  - If the fragmentDensityMapDynamic feature is not enabled, flags must not contain VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT
    ///  - If the fragmentDensityMapDeferred feature is not enabled, flags must not contain VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT
    ///  - If flags contains VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT, flags must not contain VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT
    ///  - If flags does not contain VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT, and image was created with the [`VkImageUsageFlag::FRAGMENT_DENSITY_MAP_BIT_EXT usage flag set, its flags must not contain any of VK_IMAGE_CREATE_PROTECTED_BIT, VK_IMAGE_CREATE_SPARSE_BINDING_BIT, VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT, or VK_IMAGE_CREATE_SPARSE_ALIASED_BIT
    ///  - If the `next` chain includes a VkOpaqueCaptureDescriptorDataCreateInfoEXT structure, flags must contain VK_IMAGE_VIEW_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT
    ///
    /// # Valid Usage (Implicit)
    ///  - flags must be a valid combination of VkImageViewCreateFlagBits values
    pub flags: VkImageViewCreateFlags,

    /// image is a VkImage on which the view will be created.
    ///
    /// # Valid Usage
    ///  - image must have been created with a usage value containing at least one of the following:
    ///    - [`[`VkImageUsageFlag::SAMPLED_BIT`]
    ///    - [`[`VkImageUsageFlag::STORAGE_BIT`]
    ///    - [`[`VkImageUsageFlag::COLOR_ATTACHMENT_BIT`]
    ///    - [`[`VkImageUsageFlag::DEPTH_STENCIL_ATTACHMENT_BIT`]
    ///    - [`[`VkImageUsageFlag::INPUT_ATTACHMENT_BIT`]
    ///    - [`[`VkImageUsageFlag::TRANSIENT_ATTACHMENT_BIT`]
    ///    - [`[`VkImageUsageFlag::FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`]
    ///    - [`[`VkImageUsageFlag::FRAGMENT_DENSITY_MAP_BIT_EXT`]
    ///    - [`[`VkImageUsageFlag::VIDEO_DECODE_DST_BIT_KHR`]
    ///    - [`[`VkImageUsageFlag::VIDEO_DECODE_DPB_BIT_KHR`]
    ///    - [`[`VkImageUsageFlag::VIDEO_ENCODE_SRC_BIT_KHR`]
    ///    - [`[`VkImageUsageFlag::VIDEO_ENCODE_DPB_BIT_KHR`]
    ///    - [`[`VkImageUsageFlag::SAMPLE_WEIGHT_BIT_QCOM`]
    ///    - [`[`VkImageUsageFlag::SAMPLE_BLOCK_MATCH_BIT_QCOM`]
    ///    - [`[`VkImageUsageFlag::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR`]
    ///    - [`[`VkImageUsageFlag::VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR`]
    ///  - If format has a _422 or _420 suffix then image must have been created with a width that is a multiple of 2
    ///  - If format has a _420 suffix then image must have been created with a height that is a multiple of 2
    ///  - If image is non-sparse then the image or each specified disjoint plane must be bound completely and contiguously to a single VkDeviceMemory object
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then image must have been created with the [`VkImageUsageFlag::SAMPLE_WEIGHT_BIT_QCOM usage flag set
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure and if viewType is VK_IMAGE_VIEW_TYPE_1D_ARRAY, then image must have been created with imageType VK_IMAGE_TYPE_1D
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_1D_ARRAY, then image must have been created with width equal to or greater than \((numPhases \times \mathbin{max}\left( \mathbin{align}\left(filterSize.width,4\right), filterSize.height\right))\)
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure and if viewType is VK_IMAGE_VIEW_TYPE_2D_ARRAY, then image must have been created with imageType VK_IMAGE_TYPE_2D
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_2D_ARRAY, then image must have been created with width equal to or greater than filterSize.width
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_2D_ARRAY, then image must have been created with height equal to or greater than filterSize.height
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
    ///  - If image was created with the [`VkImageUsageFlag::FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR usage flag set, viewType must be VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///  - If image was created with the [`VkImageUsageFlag::VIDEO_DECODE_DST_BIT_KHR usage flag set, [`VkImageUsageFlag::VIDEO_DECODE_SRC_BIT_KHR, or [`VkImageUsageFlag::VIDEO_DECODE_DPB_BIT_KHR, then the viewType must be VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///  - If image was created with the [`VkImageUsageFlag::VIDEO_ENCODE_DST_BIT_KHR usage flag set, [`VkImageUsageFlag::VIDEO_ENCODE_SRC_BIT_KHR, or [`VkImageUsageFlag::VIDEO_ENCODE_DPB_BIT_KHR, then the viewType must be VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY
    ///  - If image was created with the [`VkImageUsageFlag::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR or [`VkImageUsageFlag::VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR usage flags set, then viewType must be VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY
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
    ///  - If a VkImageFormatListCreateInfo structure was included in the `next` chain of the VkImageCreateInfo structure used when creating image and VkImageFormatListCreateInfo::viewFormatCount is not zero then format must be one of the formats in VkImageFormatListCreateInfo::pViewFormats
    ///  - If image was created with the VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT flag, if the format of the image is a multi-planar format, and if subresourceRange.`aspect_mask` is one of the multi-planar aspect mask bits, then format must be compatible with the VkFormat for the plane of the image format indicated by subresourceRange.`aspect_mask`, as defined in Compatible Formats of Planes of Multi-Planar Formats
    ///  - If image was not created with the VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT flag, or if the format of the image is a multi-planar format and if subresourceRange.`aspect_mask` is VK_IMAGE_ASPECT_COLOR_BIT, format must be identical to the format used to create image
    ///  - If the `next` chain includes a VkSamplerYcbcrConversionInfo structure with a conversion value other than VK_NULL_HANDLE, format must be the same used in VkSamplerYcbcrConversionCreateInfo::format
    ///  - If image has an Android external format, format must be VK_FORMAT_UNDEFINED
    ///  - If image has an QNX Screen external format, format must be VK_FORMAT_UNDEFINED
    ///  - If the shadingRateImage feature is enabled, and image was created with the [`VkImageUsageFlag::SHADING_RATE_IMAGE_BIT_NV usage flag set, format must be VK_FORMAT_R8_UINT
    ///  - If the invocationMask feature is enabled, and image was created with the [`VkImageUsageFlag::INVOCATION_MASK_BIT_HUAWEI usage flag set, format must be VK_FORMAT_R8_UINT
    ///  - If the VK_KHR_portability_subset extension is enabled, and VkPhysicalDevicePortabilitySubsetFeaturesKHR::imageViewFormatReinterpretation is VK_FALSE, the VkFormat in format must not contain a different number of components, or a different number of bits in each component, than the format of the VkImage in image
    ///  - If Vulkan 1.3 is not supported and the ycbcr2plane444Formats feature is not enabled, format must not be VK_FORMAT_G8_B8R8_2PLANE_444_UNORM, VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16, VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16, or VK_FORMAT_G16_B16R16_2PLANE_444_UNORM
    ///
    /// # Valid Usage (Implicit)
    ///  - format must be a valid VkFormat value
    pub format: VkFormat,

    /// components is a VkComponentMapping structure specifying a remapping of color components (or of depth or stencil components after they have been converted into color components).
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a VkSamplerYcbcrConversionInfo structure with a conversion value other than VK_NULL_HANDLE, all members of components must have the identity swizzle
    ///  - If image has an Android external format, all members of components must be the identity swizzle
    ///  - If image has an QNX Screen external format, all members of components must be the identity swizzle
    ///  - If the VK_KHR_portability_subset extension is enabled, and VkPhysicalDevicePortabilitySubsetFeaturesKHR::imageViewFormatSwizzle is VK_FALSE, all elements of components must have the identity swizzle
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then components must be VK_COMPONENT_SWIZZLE_IDENTITY for all components
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
    ///  - If image was created with the [`VkImageUsageFlag::FRAGMENT_DENSITY_MAP_BIT_EXT usage flag set, subresourceRange.levelCount must be 1
    ///  - If image is not a 3D image created with VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT or VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT set, or viewType is not VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY, subresourceRange.baseArrayLayer must be less than the arrayLayers specified in VkImageCreateInfo when image was created
    ///  - If subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, image is not a 3D image created with VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT or VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT set, or viewType is not VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY, subresourceRange.layerCount must be non-zero and subresourceRange.baseArrayLayer + subresourceRange.layerCount must be less than or equal to the arrayLayers specified in VkImageCreateInfo when image was created
    ///  - If image is a 3D image created with VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT set, and viewType is VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY, subresourceRange.baseArrayLayer must be less than the depth computed from baseMipLevel and extent.depth specified in VkImageCreateInfo when image was created, according to the formula defined in Image Mip Level Sizing
    ///  - If subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, image is a 3D image created with VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT set, and viewType is VK_IMAGE_VIEW_TYPE_2D or VK_IMAGE_VIEW_TYPE_2D_ARRAY, subresourceRange.layerCount must be non-zero and subresourceRange.baseArrayLayer + subresourceRange.layerCount must be less than or equal to the depth computed from baseMipLevel and extent.depth specified in VkImageCreateInfo when image was created, according to the formula defined in Image Mip Level Sizing
    ///  - If image was created with the VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT flag and format is a non-compressed format, the levelCount member of subresourceRange must be 1
    ///  - If image was created with the VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT flag, the VkPhysicalDeviceMaintenance6Properties::blockTexelViewCompatibleMultipleLayers property is not VK_TRUE, and format is a non-compressed format, then the layerCount member of subresourceRange must be 1
    ///  - subresourceRange.`aspect_mask` must only have at most 1 valid multi-planar aspect mask bit
    ///  - If the attachmentFragmentShadingRate feature is enabled, the usage for the image view includes [`VkImageUsageFlag::FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR, and layeredShadingRateAttachments is VK_FALSE, subresourceRange.layerCount must be 1
    ///  - If image was created with flags containing VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT and the [`VkImageUsageFlag::SAMPLED_BIT usage flag set, subresourceRange.layerCount must be less than or equal to VkPhysicalDeviceFragmentDensityMap2PropertiesEXT::maxSubsampledArrayLayers
    ///  - If viewType is VK_IMAGE_VIEW_TYPE_1D, VK_IMAGE_VIEW_TYPE_2D, or VK_IMAGE_VIEW_TYPE_3D; and subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, then subresourceRange.layerCount must be 1
    ///  - If viewType is VK_IMAGE_VIEW_TYPE_CUBE and subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, subresourceRange.layerCount must be 6
    ///  - If viewType is VK_IMAGE_VIEW_TYPE_CUBE_ARRAY and subresourceRange.layerCount is not VK_REMAINING_ARRAY_LAYERS, subresourceRange.layerCount must be a multiple of 6
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then subresourceRange.`aspect_mask` must be VK_IMAGE_ASPECT_COLOR_BIT
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure, then subresourceRange.levelCount must be 1
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_1D_ARRAY, then subresourceRange.layerCount must be equal to 2
    ///  - If the `next` chain includes VkImageViewSampleWeightCreateInfoQCOM structure and viewType is VK_IMAGE_VIEW_TYPE_2D_ARRAY, then subresourceRange.layerCount must be equal or greater than numPhases
    ///  - subresourceRange.`aspect_mask` must be valid for the format the image was created with
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
            view_type: VkImageViewType::_1d,
            format: VkFormat::Undefined,
            components: VkComponentMapping::default(),
            subresource_range: VkImageSubresourceRange::default(),
        }
    }
}
