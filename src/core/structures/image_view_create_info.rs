use crate::{
    VkComponentMapping, VkFormat, VkImage, VkImageSubresourceRange, VkImageViewCreateFlags,
    VkImageViewType, VkStructureType, util::NextChain,
};
use std::{os::raw::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkComponentSwizzle, VkDeviceMemory,
    VkImageAspectFlag, VkImageUsageFlag, VkImageView, VkImageViewCreateFlag, VkSampleCountFlag,
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
///  - If `aspect_mask` includes only [`VkImageAspectFlag::Stencil`], the implicit usage is
///    equal to [`VkImageStencilUsageCreateInfo::stencil_usage`].
///  - If `aspect_mask` includes only [`VkImageAspectFlag::Depth`], the implicit usage is equal
///    to [`VkImageCreateInfo::usage`].
///  - If both aspects are included in `aspect_mask`, the implicit usage is equal to the
///    intersection of [`VkImageCreateInfo::usage`] and
///    [`VkImageStencilUsageCreateInfo::stencil_usage`].
///
/// If `image` is a 3D image, its Z range can be restricted to a subset by adding a
/// [`VkImageViewSlicedCreateInfoExt`] to the `next` chain.
///
/// If `image` was created with the [`VkImageCreateFlag::MutableFormat`] flag, and if the format
/// of the image is not multi-planar format can be different from the image’s format, but if image
/// was created without the [`VkImageCreateFlag::BlockTexelViewCompatible`] flag and they are
/// not equal they must be compatible. Views of compatible formats will have the same mapping
/// between texel coordinates and memory locations irrespective of the format, with only the
/// interpretation of the bit pattern changing.
///
/// If `image` was created with a multi-planar format, and the image view’s `aspect_mask` is one of
/// [`VkImageAspectFlag::Plane0`], [`VkImageAspectFlag::Plane1`] or
/// [`VkImageAspectFlag::Plane2`], the view’s aspect mask is considered to be equivalent to
/// [`VkImageAspectFlag::Color`] when used as a framebuffer attachment.
///
/// If `image` was created with the [`VkImageCreateFlag::BlockTexelViewCompatible`] flag,
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
/// [`VkImageAspectFlag::Color`], and usage includes [`VkImageUsageFlag::Sampled`], then the
/// format must be identical to the image format and the sampler to be used with the image view
/// must enable sampler Y′CBCR conversion.
///
/// When such an image is used in a video coding operation, the sampler Y′CBCR conversion has no
/// effect.
///
/// If image was created with the [`VkImageCreateFlag::MutableFormat`] and the image has a
/// multi-planar format, and if `subresource_range.aspect_mask` is
/// [`VkImageAspectFlag::Plane0`], [`VkImageAspectFlag::Plane1`], or
/// [`VkImageAspectFlag::Plane2`], format must be compatible with the corresponding plane of the
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
///  - If `usage` contains [`VkImageUsageFlag::Sampled`], then the format features of the
///    resultant image view must contain [`VkFormatFeatureFlag::SampledImage`]
///  - If `usage` contains [`VkImageUsageFlag::Storage`], then the image view’s format features
///    must contain [`VkFormatFeatureFlag::StorageImage`]
///  - If `usage` contains [`VkImageUsageFlag::ColorAttachment`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::ColorAttachment`] or
///    [`VkFormatFeatureFlag::_2LinearColorAttachmentNv`]
///  - If `usage` contains [`VkImageUsageFlag::DepthStencilAttachment`], then the image view’s
///    format features must contain [`VkFormatFeatureFlag::DepthStencilAttachment`]
///  - If `usage` contains [`VkImageUsageFlag::VideoDecodeDstKhr`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::VideoDecodeOutputKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoDecodeDpbKhr`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::VideoDecodeDpbKhr`]
///  - `usage` must not include [`VkImageUsageFlag::VideoDecodeSrcKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoEncodeSrcKhr`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::VideoEncodeInputKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoEncodeDpbKhr`], then the image view’s format
///    features must contain [`VkFormatFeatureFlag::VideoEncodeDpbKhr`]
///  - `usage` must not include [`VkImageUsageFlag::VideoEncodeDstKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoEncodeQuantizationDeltaMapKhr`], then the
///    image view’s format features must contain
///    [`VkFormatFeatureFlag::_2VideoEncodeQuantizationDeltaMapKhr`]
///  - If `usage` contains [`VkImageUsageFlag::VideoEncodeEmphasisMapKhr`], then the image
///    view’s format features must contain [`VkFormatFeatureFlag::_2VideoEncodeEmphasisMapKhr`]
///  - If `usage` contains [`VkImageUsageFlag::InputAttachment`], and any of the following is
///    true, then the image view’s format features must contain at least one of
///    [`VkFormatFeatureFlag::ColorAttachment`] or
///    [`VkFormatFeatureFlag::DepthStencilAttachment`] or
///    [`VkFormatFeatureFlag::_2LinearColorAttachmentNv`]
///    - the `external_format_resolve` feature is not enabled
///    - the `null_color_attachment_with_external_format_resolve` property is [`VK_FALSE`]
///    - `image` was created with an [`VkExternalFormatAndroid::external_format`] value of 0
///  - If the `attachment_fragment_shading_rate` feature is enabled, and the usage for the image
///    view includes [`VkImageUsageFlag::FragmentShadingRateAttachmentKhr`] then the image
///    view’s format features must contain
///    [`VkFormatFeatureFlag::FragmentShadingRateAttachmentKhr`]
///  - If `view_type` is [`VkImageViewType::_1d`], [`VkImageViewType::_2d`], or
///    [`VkImageViewType::_3d`]; and `subresource_range.layer_count` is
///    [`VK_REMAINING_ARRAY_LAYERS`], then the remaining number of layers must be 1
///  - If `view_type` is [`VkImageViewType::Cube`] and `subresource_range.layer_count` is
///    [`VK_REMAINING_ARRAY_LAYERS`], the remaining number of layers must be 6
///  - If `view_type` is [`VkImageViewType::CubeArray`] and `subresource_range.layer_count` is
///    [`VK_REMAINING_ARRAY_LAYERS`], the remaining number of layers must be a multiple of 6
///  - If `flags` includes [`VkImageViewCreateFlag::DescriptorBufferCaptureReplayExt`], the
///    `descriptor_buffer_capture_replay` feature must be enabled
///
/// Provided by [`VK_VERSION_1_0`]
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
    ///    [`VkImageUsageFlag::Sampled`], then the `next` chain must include a
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
    ///    [`VkImageAspectFlag::Stencil`], the `usage` member of the
    ///    [`VkImageViewUsageCreateInfo`] structure must not include any bits that were not set in
    ///    the `usage` member of the [`VkImageStencilUsageCreateInfo`] structure used to create
    ///    `image`
    ///  - If the `next` chain includes a [`VkImageViewUsageCreateInfo`] structure, `image` was
    ///    created with a [`VkImageStencilUsageCreateInfo`] structure included in the `next` chain
    ///    of [`VkImageCreateInfo`], and `subresource_range.aspect_mask` includes bits other than
    ///    [`VkImageAspectFlag::Stencil`], the usage member of the
    ///    [`VkImageViewUsageCreateInfo`] structure must not include any bits that were not set in
    ///    the `usage` member of the [`VkImageCreateInfo`] structure used to create `image`
    ///  - If the `next` chain includes a [`VkExportMetalObjectCreateInfoExt`] structure, its
    ///    `export_object_type` member must be [`VkExportMetalObjectTypeFlag::MetalTextureExt`]
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

    /// `flags` is a bitmask of [`VkImageViewCreateFlag`]s specifying additional parameters of the
    /// image view.
    ///
    /// # Valid Usage
    ///  - If the `fragment_density_map_dynamic` feature is not enabled, `flags` must not contain
    ///    [`VkImageViewCreateFlag::FragmentDensityMapDynamicExt`]
    ///  - If the `fragment_density_map_deferred` feature is not enabled, `flags` must not contain
    ///    [`VkImageViewCreateFlag::FragmentDensityMapDeferredExt`]
    ///  - If `flags` contains [`VkImageViewCreateFlag::FragmentDensityMapDeferredExt`], `flags`
    ///    must not contain [`VkImageViewCreateFlag::FragmentDensityMapDynamicExt`]
    ///  - If `flags` does not contain [`VkImageViewCreateFlag::FragmentDensityMapDynamicExt`],
    ///    and `image` was created with the [`VkImageUsageFlag::FragmentDensityMapExt`] usage
    ///    flag set, its flags must not contain any of [`VkImageCreateFlag::Protected`],
    ///    [`VkImageCreateFlag::SparseBinding`], [`VkImageCreateFlag::SparseResidency`], or
    ///    [`VkImageCreateFlag::SparseAliased`]
    ///  - If the `next` chain includes a [`VkOpaqueCaptureDescriptorDataCreateInfoExt`] structure,
    ///    `flags` must contain [`VkImageViewCreateFlag::DescriptorBufferCaptureReplayExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkImageViewCreateFlag`] values
    pub flags: VkImageViewCreateFlags,

    /// `image` is a [`VkImage`] on which the view will be created.
    ///
    /// # Valid Usage
    ///  - `image` must have been created with a usage value containing at least one of the
    ///    following:
    ///    - [`VkImageUsageFlag::Sampled`]
    ///    - [`VkImageUsageFlag::Storage`]
    ///    - [`VkImageUsageFlag::ColorAttachment`]
    ///    - [`VkImageUsageFlag::DepthStencilAttachment`]
    ///    - [`VkImageUsageFlag::InputAttachment`]
    ///    - [`VkImageUsageFlag::TransientAttachment`]
    ///    - [`VkImageUsageFlag::FragmentShadingRateAttachmentKhr`]
    ///    - [`VkImageUsageFlag::FragmentDensityMapExt`]
    ///    - [`VkImageUsageFlag::VideoDecodeDstKhr`]
    ///    - [`VkImageUsageFlag::VideoDecodeDpbKhr`]
    ///    - [`VkImageUsageFlag::VideoEncodeSrcKhr`]
    ///    - [`VkImageUsageFlag::VideoEncodeDpbKhr`]
    ///    - [`VkImageUsageFlag::SampleWeightQcom`]
    ///    - [`VkImageUsageFlag::SampleBlockMatchQcom`]
    ///    - [`VkImageUsageFlag::VideoEncodeQuantizationDeltaMapKhr`]
    ///    - [`VkImageUsageFlag::VideoEncodeEmphasisMapKhr`]
    ///  - If `format` has a `_422` or `_420` suffix then `image` must have been created with a
    ///    `width` that is a multiple of 2
    ///  - If `format` has a `_420` suffix then `image` must have been created with a `height` that
    ///    is a multiple of 2
    ///  - If `image` is non-sparse then the image or each specified disjoint plane must be bound
    ///    completely and contiguously to a single [`VkDeviceMemory`] object
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure, then
    ///    `image` must have been created with the [`VkImageUsageFlag::SampleWeightQcom`] usage
    ///    flag set
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure and if
    ///    `view_type` is [`VkImageViewType::_1dArray`], then `image` must have been created with
    ///    `image_type` [`VkImageType::_1d`]
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure and
    ///    `view_type` is [`VkImageViewType::_1dArray`], then `image` must have been created with
    ///    `width` equal to or greater than
    ///    `num_phases * max(align(filter_size.width, 4), filter_size.height)`
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure and if
    ///    `view_type` is [`VkImageViewType::_2dArray`], then `image` must have been created with
    ///    `image_type` [`VkImageType::_2d`]
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure and
    ///    `view_type` is [`VkImageViewType::_2dArray`], then `image` must have been created with
    ///    `width` equal to or greater than `filter_size.width`
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure and
    ///    `view_type` is [`VkImageViewType::_2dArray`], then image must have been created with
    ///    `height` equal to or greater than `filter_size.height`
    ///
    /// # Valid Usage (Implicit)
    ///  - `image` must be a valid [`VkImage`] handle
    pub image: VkImage,

    /// `view_type` is a [`VkImageViewType`] value specifying the type of the image view.
    ///
    /// # Valid Usage
    ///  - If `image` was not created with [`VkImageCreateFlag::CubeCompatible`] then
    ///    `view_type` must not be [`VkImageViewType::Cube`] or [`VkImageViewType::CubeArray`]
    ///  - If the `image_cube_array` feature is not enabled, `view_type` must not be
    ///    [`VkImageViewType::CubeArray`]
    ///  - If `image` was created with [`VkImageType::_3d`] but without
    ///    [`VkImageCreateFlag::_2dArrayCompatible`] set then `view_type` must not be
    ///    [`VkImageViewType::_2dArray`]
    ///  - If `image` was created with [`VkImageType::_3d`] but without
    ///    [`VkImageCreateFlag::_2dArrayCompatible`] or
    ///    [`VkImageCreateFlag::_2dViewCompatibleExt`] set, then `view_type` must not be
    ///    [`VkImageViewType::_2d`]
    ///  - If `image` was created with a samples value not equal to [`VkSampleCountFlag::_1`]
    ///    then `view_type` must be either [`VkImageViewType::_2d`] or
    ///    [`VkImageViewType::_2dArray`]
    ///  - `view_type` must be compatible with the type of image as shown in the view type
    ///    compatibility table
    ///  - If `image` was created with the
    ///    [`VkImageUsageFlag::FragmentShadingRateAttachmentKhr`] usage flag set, `view_type`
    ///    must be [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`]
    ///  - If `image` was created with the [`VkImageUsageFlag::VideoDecodeDstKhr`] usage flag
    ///    set, [`VkImageUsageFlag::VideoDecodeSrcKhr`], or
    ///    [`VkImageUsageFlag::VideoDecodeDpbKhr`], then the `view_type` must be
    ///    [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`]
    ///  - If `image` was created with the [`VkImageUsageFlag::VideoEncodeDstKhr`] usage flag
    ///    set, [`VkImageUsageFlag::VideoEncodeSrcKhr`], or
    ///    [`VkImageUsageFlag::VideoEncodeDpbKhr`], then the `view_type` must be
    ///    [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`]
    ///  - If `image` was created with the
    ///    [`VkImageUsageFlag::VideoEncodeQuantizationDeltaMapKhr`] or
    ///    [`VkImageUsageFlag::VideoEncodeEmphasisMapKhr`] usage flags set, then `view_type`
    ///    must be [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`]
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure, then
    ///    `view_type` must be [`VkImageViewType::_1dArray`] or [`VkImageViewType::_2dArray`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `view_type` must be a valid [`VkImageViewType`] value
    pub view_type: VkImageViewType,

    /// `format` is a [`VkFormat`] specifying the format and type used to interpret texel blocks of
    /// the image.
    ///
    /// # Valid Usage
    ///  - If `image` was created with the [`VkImageCreateFlag::MutableFormat`] flag, but
    ///    without the [`VkImageCreateFlag::BlockTexelViewCompatible`] flag, and if the format
    ///    of the image is not a multi-planar format, `format` must be compatible with the format
    ///    used to create `image`
    ///  - If `image` was created with the [`VkImageCreateFlag::BlockTexelViewCompatible`] flag,
    ///    `format` must be compatible with, or must be an uncompressed format that is
    ///    size-compatible with, the format used to create `image`
    ///  - If a [`VkImageFormatListCreateInfo`] structure was included in the `next` chain of the
    ///    [`VkImageCreateInfo`] structure used when creating image and
    ///    [`VkImageFormatListCreateInfo::view_format_count`] is not zero then `format` must be one
    ///    of the formats in [`VkImageFormatListCreateInfo::view_formats`]
    ///  - If `image` was created with the [`VkImageCreateFlag::MutableFormat`] flag, if the
    ///    format of the image is a multi-planar format, and if `subresource_range.aspect_mask` is
    ///    one of the multi-planar aspect mask bits, then `format` must be compatible with the
    ///    [`VkFormat`] for the plane of the image format indicated by
    ///    `subresource_range.aspect_mask`
    ///  - If `image` was not created with the [`VkImageCreateFlag::MutableFormat`] flag, or if
    ///    the format of the image is a multi-planar format and if `subresource_range.aspect_mask`
    ///    is [`VkImageAspectFlag::Color`], `format` must be identical to the format used to
    ///    create `image`
    ///  - If the `next` chain includes a [`VkSamplerYcbcrConversionInfo`] structure with a
    ///    conversion value other than [`VK_NULL_HANDLE`], format must be the same used in
    ///    [`VkSamplerYcbcrConversionCreateInfo::format`]
    ///  - If `image` has an Android external format, `format` must be [`VkFormat::Undefined`]
    ///  - If `image` has an QNX Screen external format, `format` must be [`VkFormat::Undefined`]
    ///  - If the `shading_rate_image` feature is enabled, and `image` was created with the
    ///    [`VkImageUsageFlag::ShadingRateImageNv`] usage flag set, `format` must be
    ///    [`VkFormat::R8UInt`]
    ///  - If the `invocation_mask` feature is enabled, and `image` was created with the
    ///    [`VkImageUsageFlag::InvocationMaskHuawei`] usage flag set, `format` must be
    ///    [`VkFormat::R8UInt`]
    ///  - If the [`khr_portability_subset`] extension is enabled, and
    ///    [`VkPhysicalDevicePortabilitySubsetFeaturesKHR::image_view_format_reinterpretation`] is
    ///    [`VK_FALSE`], the [`VkFormat`] in `format` must not contain a different number of
    ///    components, or a different number of bits in each component, than the format of the
    ///    [`VkImage`] in `image`
    ///  - If Vulkan 1.3 is not supported and the `ycbcr2_plane444_formats` feature is not enabled,
    ///    `format` must not be [`VkFormat::G8B8R8_2Plane444UNorm`],
    ///    [`VkFormat::G10X6B10X6R10X6_2Plane444UNorm3Pack16`],
    ///    [`VkFormat::G12X4B12X4R12X4_2Plane444UNorm3Pack16`], or
    ///    [`VkFormat::G16B16R16_2Plane444UNorm`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `format` must be a valid [`VkFormat`] value
    pub format: VkFormat,

    /// `components` is a [`VkComponentMapping`] structure specifying a remapping of color
    /// components (or of depth or stencil components after they have been converted into color
    /// components).
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkSamplerYcbcrConversionInfo`] structure with a
    ///    conversion value other than [`VK_NULL_HANDLE`], all members of `components` must have
    ///    the identity swizzle
    ///  - If `image` has an Android external format, all members of `components` must be the
    ///    identity swizzle
    ///  - If `image` has an QNX Screen external format, all members of `components` must be the
    ///    identity swizzle
    ///  - If the [`khr_portability_subset`] extension is enabled, and
    ///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::image_view_format_swizzle`] is
    ///    [`VK_FALSE`], all elements of `components` must have the identity swizzle
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure, then
    ///    `components` must be [`VkComponentSwizzle::Identity`] for all components
    ///
    /// # Valid Usage (Implicit)
    ///  - `components` must be a valid [`VkComponentMapping`] structure
    pub components: VkComponentMapping,

    /// `subresource_range` is a [`VkImageSubresourceRange`] structure selecting the set of mipmap
    /// levels and array layers to be accessible to the view.
    ///
    /// # Valid Usage
    ///  - If `image` was created with [`VkImageType::_3d`] and `view_type` is
    ///    [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`] then
    ///    `subresource_range.level_count` must be 1
    ///  - `subresource_range.base_mip_level` must be less than the `mip_levels` specified in
    ///    [`VkImageCreateInfo`] when `image` was created
    ///  - If `subresource_range.level_count` is not [`VK_REMAINING_MIP_LEVELS`],
    ///    `subresource_range.base_mip_level + subresource_range.level_count` must be less than or
    ///    equal to the `mip_levels` specified in [`VkImageCreateInfo`] when image was created
    ///  - If `image` was created with the [`VkImageUsageFlag::FragmentDensityMapExt`] usage
    ///    flag set, `subresource_range.level_count` must be 1
    ///  - If `image` is not a 3D image created with [`VkImageCreateFlag::_2dArrayCompatible`]
    ///    or [`VkImageCreateFlag::_2dViewCompatible`] set, or `view_type` is not
    ///    [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`],
    ///    `subresource_range.base_array_layer` must be less than the `array_layers` specified in
    ///    [`VkImageCreateInfo`] when `image` was created
    ///  - If `subresource_range.layer_count` is not [`VK_REMAINING_ARRAY_LAYERS`], `image` is not
    ///    a 3D image created with [`VkImageCreateFlag::_2dArrayCompatible`] or
    ///    [`VkImageCreateFlag::_2dViewCompatibleExt`] set, or `view_type` is not
    ///    [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`],
    ///    `subresource_range.layer_count` must be non-zero and
    ///    `subresource_range.base_array_layer + subresource_range.layer_count` must be less than
    ///    or equal to the `array_layers` specified in [`VkImageCreateInfo`] when image was created
    ///  - If `image` is a 3D image created with [`VkImageCreateFlag::_2dArrayCompatible`] set,
    ///    and `view_type` is [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`],
    ///    `subresource_range.base_array_layer` must be less than the depth computed from
    ///    `base_mip_level` and `extent.depth` specified in [`VkImageCreateInfo`] when `image` was
    ///    created
    ///  - If `subresource_range.layer_count` is not [`VK_REMAINING_ARRAY_LAYERS`], `image` is a 3D
    ///    `image` created with [`VkImageCreateFlag::_2dArrayCompatible`] set, and `view_type`
    ///    is [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`],
    ///    `subresource_range.layer_count` must be non-zero and
    ///    `subresource_range.base_array_layer + subresource_range.layer_count` must be less than
    ///    or equal to the depth computed from `base_mip_level` and `extent.depth` specified in
    ///    [`VkImageCreateInfo`] when `image` was created
    ///  - If `image` was created with the [`VkImageCreateFlag::BlockTexelViewCompatible`] flag
    ///    and `format` is a non-compressed format, the `level_count` member of `subresource_range`
    ///    must be 1
    ///  - If `image` was created with the [`VkImageCreateFlag::BlockTexelViewCompatible`] flag,
    ///    the
    ///    [`VkPhysicalDeviceMaintenance6Properties::block_texel_view_compatible_multiple_layers`]
    ///    property is not [`VK_TRUE`], and `format` is a non-compressed format, then the
    ///    `layer_count` member of `subresource_range` must be 1
    ///  - `subresource_range.aspect_mask` must only have at most 1 valid multi-planar aspect mask
    ///    bit
    ///  - If the `attachment_fragment_shading_rate` feature is enabled, the usage for the image
    ///    view includes [`VkImageUsageFlag::FragmentShadingRateAttachmentKhr`], and
    ///    `layered_shading_rate_attachments` is [`VK_FALSE`], `subresource_range.layer_count` must
    ///    be 1
    ///  - If `image` was created with flags containing [`VkImageCreateFlag::SubsampledExt`] and
    ///    the [`VkImageUsageFlag::Sampled`] usage flag set, `subresource_range.layer_count`
    ///    must be less than or equal to
    ///    [`VkPhysicalDeviceFragmentDensityMap2PropertiesExt::max_subsampled_array_layers`]
    ///  - If `view_type` is [`VkImageViewType::_1d`], [`VkImageViewType::_2d`], or
    ///    [`VkImageViewType::_3d`]; and `subresource_range.layer_count` is not
    ///    [`VK_REMAINING_ARRAY_LAYERS`], then `subresource_range.layer_count` must be 1
    ///  - If `view_type` is [`VkImageViewType::Cube`] and `subresource_range.layer_count` is not
    ///    [`VK_REMAINING_ARRAY_LAYERS`], `subresource_range.layer_count` must be 6
    ///  - If `view_type` is [`VkImageViewType::CubeArray`] and `subresource_range.layer_count` is
    ///    not [`VK_REMAINING_ARRAY_LAYERS`], `subresource_range.layer_count` must be a multiple of
    ///    6
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure, then
    ///    `subresource_range.aspect_mask` must be [`VkImageAspectFlag::Color`]
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure, then
    ///    `subresource_range.level_count` must be 1
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure and
    ///    `view_type` is [`VkImageViewType::_1dArray`], then `subresource_range.layer_count` must
    ///    be equal to 2
    ///  - If the `next` chain includes [`VkImageViewSampleWeightCreateInfoQcom`] structure and
    ///    `view_type` is [`VkImageViewType::_2dArray`], then `subresource_range.layer_count` must
    ///    be equal or greater than `num_phases`
    ///  - `subresource_range.aspect_mask` must be valid for the format the image was created with
    ///
    /// # Valid Usage (Implicit)
    ///  - `subresource_range` must be a valid [`VkImageSubresourceRange`] structure
    pub subresource_range: VkImageSubresourceRange,
}

const impl Default for VkImageViewCreateInfo {
    fn default() -> Self {
        VkImageViewCreateInfo {
            r#type: VkStructureType::ImageViewCreateInfo,
            next: null(),
            flags: VkImageViewCreateFlags::empty(),
            image: VkImage::null(),
            view_type: VkImageViewType::_1d,
            format: VkFormat::Undefined,
            components: VkComponentMapping::default(),
            subresource_range: VkImageSubresourceRange::default(),
        }
    }
}

impl NextChain for VkImageViewCreateInfo {
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
