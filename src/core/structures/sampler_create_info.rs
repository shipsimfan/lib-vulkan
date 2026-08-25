use crate::{
    VK_FALSE, VkBool32, VkBorderColor, VkCompareOp, VkFilter, VkSamplerAddressMode,
    VkSamplerCreateFlags, VkSamplerMipmapMode, VkStructureType, util::NextChain,
};
use std::{
    ffi::{c_float, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_LOD_CLAMP_NONE, VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkFormat, VkImageViewType,
    VkObjectType, VkPhysicalDeviceLimits, VkSampler, VkSamplerCreateFlag,
    ext_debug_utils::VkDebugUtilsObjectNameInfoExt,
};

/// Structure specifying parameters of a newly created sampler
///
/// # Description
/// The maximum number of sampler objects which can be simultaneously created on a device is
/// implementation-dependent and specified by the `max_sampler_allocation_count` member of the
/// [`VkPhysicalDeviceLimits`] structure.
///
/// Since [`VkSampler`] is a non-dispatchable handle type, implementations may return the same
/// handle for sampler state vectors that are identical. In such cases, all such objects would only
/// count once against the `max_sampler_allocation_count` limit.
///
/// When this structure is used to write a descriptor via [`VkWriteSamplerDescriptorsExt`],
/// applications can give the descriptor a debug name in a similar way to naming an object, via the
/// [`VkDebugUtilsObjectNameInfoExt`] structure. However, as there is no actual object,
/// [`VkDebugUtilsObjectNameInfoExt`] must be passed via the `next` chain of this structure, with a
/// `object_type` of [`VkObjectType::Unknown`] and a `object_handle` of [`VK_NULL_HANDLE`]. The
/// name is attached to the unique set of descriptor bits written by the implementation, and
/// writing the same bits again with new debug info may rename the original descriptor.
///
/// # Valid Usage
///  - The maximum number of samplers with custom border colors which can be simultaneously created
///    on a device is implementation-dependent and specified by the
///    `max_custom_border_color_samplers` member of the
///    [`VkPhysicalDeviceCustomBorderColorPropertiesExt`] structure
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkSamplerCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::SamplerCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If sampler Y′CBCR conversion is enabled and the `next` chain includes a
    ///    [`VkSamplerReductionModeCreateInfo`] structure, then the sampler reduction mode must be
    ///    [`VkSamplerReductionMode::WeightedAverage`]
    ///  - If the `sampler_filter_minmax` feature is not enabled and the `next` chain includes a
    ///    [`VkSamplerReductionModeCreateInfo`] structure, then the sampler reduction mode must be
    ///    [`VkSamplerReductionMode::WeightedAverage`]
    ///  - If the [`ext_filter_cubic`] extension is not enabled and either `mag_filter` or
    ///    `min_filter` is [`VkFilter::CubicImg`], the `reduction_mode` member of
    ///    [`VkSamplerReductionModeCreateInfo`] must be [`VkSamplerReductionMode::WeightedAverage`]
    ///  - If `compare_enable` is [`VK_TRUE`], the `reduction_mode` member of
    ///    [`VkSamplerReductionModeCreateInfo`] must be [`VkSamplerReductionMode::WeightedAverage`]
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkDebugUtilsObjectNameInfoExt`],
    ///    [`VkOpaqueCaptureDescriptorDataCreateInfoExt`],
    ///    [`VkSamplerBlockMatchWindowCreateInfoQcom`],
    ///    [`VkSamplerBorderColorComponentMappingCreateInfoExt`],
    ///    [`VkSamplerCubicWeightsCreateInfoQcom`], [`VkSamplerCustomBorderColorCreateInfoExt`],
    ///    [`VkSamplerCustomBorderColorIndexCreateInfoExt`], [`VkSamplerReductionModeCreateInfo`],
    ///    or [`VkSamplerYcbcrConversionInfo`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkSamplerCreateFlag`]s describing additional parameters of the
    /// sampler.
    ///
    /// # Valid Usage
    ///  - If the `non_seamless_cube_map` feature is not enabled, `flags` must not include
    ///    [`VkSamplerCreateFlag::NonSeamlessCubeMapExt`]
    ///  - If `flags` includes [`VkSamplerCreateFlag::DescriptorBufferCaptureReplayExt`], the
    ///    `descriptor_buffer_capture_replay` feature must be enabled
    ///  - If the `next` chain includes a [`VkOpaqueCaptureDescriptorDataCreateInfoExt`] structure,
    ///    `flags` must contain [`VkSamplerCreateFlag::DescriptorBufferCaptureReplayExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkSamplerCreateFlag`] values
    pub flags: VkSamplerCreateFlags,

    /// `mag_filter` is a [`VkFilter`] value specifying the magnification filter to apply to
    /// lookups.
    ///
    ///  - If sampler Y′CBCR conversion is enabled and the potential format features of the sampler
    ///    Y′CBCR conversion do not support
    ///    [`VkFormatFeature::SampledImageYcbcrConversionSeparateReconstructionFilter`],
    ///    `mag_filter` must be equal to the sampler Y′CBCR conversion’s `chroma_filter`
    ///  - If `unnormalized_coordinates` is [`VK_TRUE`], `min_filter` and `mag_filter` must be
    ///    equal
    ///  - If `flags` includes [`VkSamplerCreateFlag::SubsampledExt`], then `min_filter` and
    ///    `mag_filter` must be equal
    ///  - If `flags` includes [`VkSamplerCreateFlag::ImageProcessingQcom`], then `mag_filter` must
    ///    be [`VkFilter::Nearest`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `mag_filter` must be a valid [`VkFilter`] value
    pub mag_filter: VkFilter,

    /// `min_filter` is a [`VkFilter`] value specifying the minification filter to apply to
    /// lookups.
    ///
    /// # Valid Usage
    ///  - If sampler Y′CBCR conversion is enabled and the potential format features of the sampler
    ///    Y′CBCR conversion do not support
    ///    [`VkFormatFeature::SampledImageYcbcrConversionSeparateReconstructionFilter`],
    ///    `min_filter` must be equal to the sampler Y′CBCR conversion’s `chroma_filter`
    ///  - If `unnormalized_coordinates` is [`VK_TRUE`], `min_filter` and `mag_filter` must be
    ///    equal
    ///  - If `flags` includes [`VkSamplerCreateFlag::SubsampledExt`], then `min_filter` and
    ///    `mag_filter` must be equal
    ///  - If `flags` includes [`VkSamplerCreateFlag::ImageProcessingQcom`], then `min_filter` must
    ///    be [`VkFilter::Nearest`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `min_filter` must be a valid [`VkFilter`] value
    pub min_filter: VkFilter,

    /// `mimmap_mode` is a [`VkSamplerMipmapMode`] value specifying the mipmap filter to apply to
    /// lookups.
    ///
    /// # Valid Usage
    ///  - If `unnormalized_coordinates` is [`VK_TRUE`], `mimmap_mode` must be
    ///    [`VkSamplerMipmapMode::Nearest`]
    ///  - If flags includes [`VkSamplerCreateFlag::SubsampledExt`], then `mimmap_mode` must be
    ///    [`VkSamplerMipmapMode::Nearest`]
    ///  - If flags includes [`VkSamplerCreateFlag::ImageProcessingQcom`], then `mimmap_mode` must
    ///    be [`VkSamplerMipmapMode::Nearest`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `mimmap_mode` must be a valid [`VkSamplerMipmapMode`] value
    pub mipmap_mode: VkSamplerMipmapMode,

    /// `address_mode_u` is a [`VkSamplerAddressMode`] value specifying the wrapping operation used
    /// when the `i` coordinate used to sample the image would be out of bounds.
    ///
    /// # Valid Usage
    ///  - If `unnormalized_coordinates` is [`VK_TRUE`], `address_mode_u` must each be either
    ///    [`VkSamplerAddressMode::ClampToEdge`] or [`VkSamplerAddressMode::ClampToBorder`]
    ///  - If sampler Y′CBCR conversion is enabled, `address_mode_u` must be
    ///    [`VkSamplerAddressMode::ClampToEdge`]
    ///  - If the `sampler_mirror_clamp_to_edge` feature is not enabled, and if the
    ///    [`khr_sampler_mirror_clamp_to_edge`] extension is not enabled, `address_mode_u` must not
    ///    be [`VkSamplerAddressMode::MirrorClampToEdge`]
    ///  - If flags includes [`VkSamplerCreateFlag::SubsampledExt`], then `address_mode_u` must be
    ///    either [`VkSamplerAddressMode::ClampToEdge`] or [`VkSamplerAddressMode::ClampToBorder`]
    ///  - If flags includes [`VkSamplerCreateFlag::ImageProcessingQcom`], then `address_mode_u`
    ///    must be either [`VkSamplerAddressMode::ClampToEdge`] or
    ///    [`VkSamplerAddressMode::ClampToBorder`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `address_mode_u` must be a valid [`VkSamplerAddressMode`] value
    pub address_mode_u: VkSamplerAddressMode,

    /// `address_mode_v` is a [`VkSamplerAddressMode`] value specifying the wrapping operation used
    /// when the `j` coordinate used to sample the image would be out of bounds.
    ///
    /// # Valid Usage
    ///  - If `unnormalized_coordinates` is [`VK_TRUE`], `address_mode_v` must each be either
    ///    [`VkSamplerAddressMode::ClampToEdge`] or [`VkSamplerAddressMode::ClampToBorder`]
    ///  - If sampler Y′CBCR conversion is enabled, `address_mode_v` must be
    ///    [`VkSamplerAddressMode::ClampToEdge`]
    ///  - If the `sampler_mirror_clamp_to_edge` feature is not enabled, and if the
    ///    [`khr_sampler_mirror_clamp_to_edge`] extension is not enabled, `address_mode_v` must not
    ///    be [`VkSamplerAddressMode::MirrorClampToEdge`]
    ///  - If flags includes [`VkSamplerCreateFlag::SubsampledExt`], then `address_mode_v` must be
    ///    either [`VkSamplerAddressMode::ClampToEdge`] or [`VkSamplerAddressMode::ClampToBorder`]
    ///  - If flags includes [`VkSamplerCreateFlag::ImageProcessingQcom`], then `address_mode_v`
    ///    must be either [`VkSamplerAddressMode::ClampToEdge`] or
    ///    [`VkSamplerAddressMode::ClampToBorder`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `address_mode_v` must be a valid [`VkSamplerAddressMode`] value
    pub address_mode_v: VkSamplerAddressMode,

    /// `address_mode_w` is a [`VkSamplerAddressMode`] value spcifying the wrapping operation used
    /// when the `k` coordinate used to sample the image would be out of bounds. If
    /// `unnormalized_coordinates` is [`VK_TRUE`], `address_mode_w` is ignored.
    ///
    /// # Valid Usage
    ///  - If sampler Y′CBCR conversion is enabled, `address_mode_w` must be
    ///    [`VkSamplerAddressMode::ClampToEdge`]
    ///  - If the `sampler_mirror_clamp_to_edge` feature is not enabled, and if the
    ///    [`khr_sampler_mirror_clamp_to_edge`] extension is not enabled, `address_mode_w` must not
    ///    be [`VkSamplerAddressMode::MirrorClampToEdge`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `address_mode_w` must be a valid [`VkSamplerAddressMode`] value
    pub address_mode_w: VkSamplerAddressMode,

    /// `mip_lod_bias` is the bias to be added to mipmap LOD calculation and bias provided by image
    /// sampling functions in SPIR-V, as described in the LOD Operation section.
    ///
    /// # Valid Usage
    ///  - The absolute value of `mip_lod_bias` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_sampler_lod_bias`]
    ///  - If the [`khr_portability_subset`] extension is enabled, and
    ///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::sampler_mip_lod_bias`] is [`VK_FALSE`],
    ///    `mip_lod_bias` must be zero
    pub mip_lod_bias: c_float,

    /// `anisotropy_enable` is [`VK_TRUE`] to enable anisotropic filtering, or [`VK_FALSE`]
    /// otherwise.
    ///
    /// # Valid Usage
    ///  - If the `sampler_anisotropy` feature is not enabled, `anisotropy_enable` must be
    ///    [`VK_FALSE`]
    ///  - If `unnormalized_coordinates` is [`VK_TRUE`], `anisotropy_enable` must be [`VK_FALSE`]
    ///  - If sampler Y′CBCR conversion is enabled `anisotropy_enable` must be [`VK_FALSE`]
    ///  - If `flags` includes [`VkSamplerCreateFlag::SubsampledExt`], then `anisotropy_enable`
    ///    must be [`VK_FALSE`]
    ///  - If `flags` includes [`VkSamplerCreateFlag::ImageProcessingQcom`], then
    ///    `anisotropy_enable` must be [`VK_FALSE`]
    pub anisotropy_enable: VkBool32,

    /// `max_anisotropy` is the anisotropy value clamp used by the sampler when `anisotropy_enable`
    /// is [`VK_TRUE`]. If `anisotropy_enable` is [`VK_FALSE`], `max_anisotropy` is ignored.
    ///
    /// # Valid Usage
    ///  - If `anisotropy_enable` is [`VK_TRUE`], `max_anisotropy` must be between 1.0 and
    ///    [`VkPhysicalDeviceLimits::max_sampler_anisotropy`], inclusive
    ///  - If either `mag_filter` or `min_filter` is [`VkFilter::CubicExt`], `anisotropy_enable`
    ///    must be [`VK_FALSE`]
    pub max_anisotropy: c_float,

    /// `compare_enable` is [`VK_TRUE`] to enable comparison against a reference value during
    /// lookups, or [`s`] otherwise.
    ///
    /// > Note: Some implementations will default to shader state if this member does not match.
    ///
    /// # Valid Usage
    ///  - If `unnormalized_coordinates` is [`VK_TRUE`], `compare_enable` must be [`VK_FALSE`]
    ///  - If `flags` includes [`VkSamplerCreateFlag::SubsampledExt`], then `compare_enable` must
    ///    be [`VK_FALSE`]
    ///  - If `flags` includes [`VkSamplerCreateFlag::ImageProcessingQcom`], then `compare_enable`
    ///    must be [`VK_FALSE`]
    pub compare_enable: VkBool32,

    /// `compare_op` is a [`VkCompareOp`] value specifying the comparison operator to apply to
    /// fetched data before filtering.
    ///
    /// # Valid Usage
    ///  - If `compare_enable` is [`VK_TRUE`], `compare_op` must be a valid [`VkCompareOp`] value
    pub compare_op: VkCompareOp,

    /// `min_lod` is used to clamp the minimum of the computed LOD value.
    ///
    /// # Valid Usage
    ///  - If `unnormalized_coordinates` is [`VK_TRUE`], `min_lod` must be zero
    ///  - If `flags` includes [`VkSamplerCreateFlag::SubsampledExt`], then `min_lod` must be zero
    ///  - If `flags` includes [`VkSamplerCreateFlag::ImageProcessingQcom`], then `min_lod` must be
    ///    zero
    pub min_lod: c_float,

    /// `max_lod` is used to clamp the maximum of the computed LOD value. To avoid clamping the
    /// maximum value, set `max_lod` to the constant [`VK_LOD_CLAMP_NONE`].
    ///
    /// # Valid Usage
    ///  - `max_lod` must be greater than or equal to `min_lod`
    ///  - If `unnormalized_coordinates` is [`VK_TRUE`], `max_lod` must be zero
    ///  - If `flags` includes [`VkSamplerCreateFlag::SubsampledExt`], then `max_lod` must be zero
    ///  - If `flags` includes [`VkSamplerCreateFlag::ImageProcessingQcom`], then `max_lod` must be
    ///    zero
    pub max_lod: c_float,

    /// `border_color` is a [`VkBorderColor`] value specifying the predefined border color to use.
    ///
    /// # Valid Usage
    ///  - If any of `address_mode_u`, `address_mode_v` or `address_mode_w` are
    ///    [`VkSamplerAddressMode::ClampToBorder`], `border_color` must be a valid
    ///    [`VkBorderColor`] value
    ///  - If `border_color` is one of [`VkBorderColor::FloatCustomExt`] or
    ///    [`VkBorderColor::IntCustomExt`], then a [`VkSamplerCustomBorderColorCreateInfoExt`] must
    ///    be included in the `next` chain
    ///  - If the `custom_border_colors` feature is not enabled, `border_color` must not be
    ///    [`VkBorderColor::FloatCustomExt`] or [`VkBorderColor::IntCustomExt`]
    ///  - If `border_color` is one of [`VkBorderColor::FloatCustomExt`] or
    ///    [`VkBorderColor::IntCustomExt`], and [`VkSamplerCustomBorderColorCreateInfoExt::format`]
    ///    is not [`VkFormat::Undefined`],
    ///    [`VkSamplerCustomBorderColorCreateInfoExt::custom_border_color`] must be within the
    ///    range of values representable in `format`
    ///  - If `flags` includes [`VkSamplerCreateFlag::ImageProcessingQcom`], and if
    ///    `address_mode_u` or `address_mode_v` is [`VkSamplerAddressMode::ClampToBorder`], then
    ///    `border_color` must be [`VkBorderColor::FloatTransparentBlack`]
    pub border_color: VkBorderColor,

    /// `unnormalized_coordinates` controls whether to use unnormalized or normalized texel
    /// coordinates to address texels of the image. When `unnormalized_coordinates` is [`VK_TRUE`],
    /// the range of the image coordinates used to lookup the texel is in the range of zero to the
    /// image size in each dimension. When `unnormalized_coordinates` is [`VK_FALSE`], the range of
    /// image coordinates is zero to one.
    ///
    /// When `unnormalized_coordinates` is [`VK_TRUE`], images the sampler is used with in the
    /// shader have the following requirements:
    ///  - The `view_type` must be either [`VkImageViewType::_1d`] or [`VkImageViewType::_2d`].
    ///  - The image view must have a single layer and a single mip level.
    ///  - When `unnormalized_coordinates` is [`VK_TRUE`], image built-in functions in the shader
    ///    that use the sampler have the following requirements:
    ///    - The functions must not use projection.
    ///    - The functions must not use offsets.
    ///
    /// # Valid Usage
    ///  - If sampler Y′CBCR conversion is enabled, `unnormalized_coordinates` must be [`VK_FALSE`]
    ///  - If `flags` includes [`VkSamplerCreateFlag::SubsampledExt`], then
    ///    `unnormalized_coordinates` must be [`VK_FALSE`]
    pub unnormalized_coordinates: VkBool32,
}

const impl Default for VkSamplerCreateInfo {
    fn default() -> Self {
        VkSamplerCreateInfo {
            r#type: VkStructureType::SamplerCreateInfo,
            next: null(),
            flags: VkSamplerCreateFlags::default(),
            mag_filter: VkFilter::Nearest,
            min_filter: VkFilter::Nearest,
            mipmap_mode: VkSamplerMipmapMode::Nearest,
            address_mode_u: VkSamplerAddressMode::Repeat,
            address_mode_v: VkSamplerAddressMode::Repeat,
            address_mode_w: VkSamplerAddressMode::Repeat,
            mip_lod_bias: 0.0,
            anisotropy_enable: VK_FALSE,
            max_anisotropy: 1.0,
            compare_enable: VK_FALSE,
            compare_op: VkCompareOp::Always,
            min_lod: 0.0,
            max_lod: 1.0,
            border_color: VkBorderColor::IntOpaqueBlack,
            unnormalized_coordinates: VK_FALSE,
        }
    }
}

impl NextChain for VkSamplerCreateInfo {
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
