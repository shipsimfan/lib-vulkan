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
    VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkObjectType, VkPhysicalDeviceLimits, VkSampler,
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
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be either [`null`] or a pointer to a valid instance of VkDebugUtilsObjectNameInfoEXT, VkOpaqueCaptureDescriptorDataCreateInfoEXT, VkSamplerBlockMatchWindowCreateInfoQCOM, VkSamplerBorderColorComponentMappingCreateInfoEXT, VkSamplerCubicWeightsCreateInfoQCOM, VkSamplerCustomBorderColorCreateInfoEXT, VkSamplerCustomBorderColorIndexCreateInfoEXT, VkSamplerReductionModeCreateInfo, or VkSamplerYcbcrConversionInfo
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// flags is a bitmask of VkSamplerCreateFlagBits describing additional parameters of the sampler.
    ///
    /// # Valid Usage
    ///  - If the nonSeamlessCubeMap feature is not enabled, flags must not include VK_SAMPLER_CREATE_NON_SEAMLESS_CUBE_MAP_BIT_EXT
    ///  - If flags includes VK_SAMPLER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT, the descriptorBufferCaptureReplay feature must be enabled
    ///  - If the `next` chain includes a VkOpaqueCaptureDescriptorDataCreateInfoEXT structure, flags must contain VK_SAMPLER_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT
    ///
    /// # Valid Usage (Implicit)
    ///  - flags must be a valid combination of VkSamplerCreateFlagBits values
    pub flags: VkSamplerCreateFlags,

    /// `mag_filter` is a VkFilter value specifying the magnification filter to apply to lookups.
    ///  - If sampler Y′CBCR conversion is enabled and the potential format features of the sampler Y′CBCR conversion do not support VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT, `mag_filter` must be equal to the sampler Y′CBCR conversion’s chromaFilter
    ///  - If unnormalizedCoordinates is [`VK_TRUE`], `min_filter` and `mag_filter` must be equal
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then `min_filter` and `mag_filter` must be equal
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, then `mag_filter` must be [`VkFilter::NEAREST
    ///
    /// # Valid Usage (Implicit)
    ///  - `mag_filter` must be a valid VkFilter value
    pub mag_filter: VkFilter,

    /// `min_filter` is a VkFilter value specifying the minification filter to apply to lookups.
    ///
    /// # Valid Usage
    ///  - If sampler Y′CBCR conversion is enabled and the potential format features of the sampler Y′CBCR conversion do not support VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT, `min_filter` must be equal to the sampler Y′CBCR conversion’s chromaFilter
    ///  - If unnormalizedCoordinates is [`VK_TRUE`], `min_filter` and `mag_filter` must be equal
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then `min_filter` and `mag_filter` must be equal
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, then `min_filter` must be [`VkFilter::NEAREST
    ///
    /// # Valid Usage (Implicit)
    ///  - `min_filter` must be a valid VkFilter value
    pub min_filter: VkFilter,

    /// mipmapMode is a VkSamplerMipmapMode value specifying the mipmap filter to apply to lookups.
    ///
    /// # Valid Usage
    ///  - If unnormalizedCoordinates is [`VK_TRUE`], mipmapMode must be VK_SAMPLER_MIPMAP_MODE_NEAREST
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then mipmapMode must be VK_SAMPLER_MIPMAP_MODE_NEAREST
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, then mipmapMode must be VK_SAMPLER_MIPMAP_MODE_NEAREST
    ///
    /// # Valid Usage (Implicit)
    ///  - mipmapMode must be a valid VkSamplerMipmapMode value
    pub mipmap_mode: VkSamplerMipmapMode,

    /// addressModeU is a VkSamplerAddressMode value specifying the wrapping operation used when the i coordinate used to sample the image would be out of bounds.
    ///
    /// # Valid Usage
    ///  - If unnormalizedCoordinates is [`VK_TRUE`], addressModeU must each be either VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE or VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER
    ///  - If sampler Y′CBCR conversion is enabled, addressModeU must be VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE
    ///  - If the samplerMirrorClampToEdge feature is not enabled, and if the VK_KHR_sampler_mirror_clamp_to_edge extension is not enabled, addressModeU must not be VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then addressModeU must be either VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE or VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, then addressModeU must be either VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE or VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER
    ///
    /// # Valid Usage (Implicit)
    ///  - addressModeU must be a valid VkSamplerAddressMode value
    pub address_mode_u: VkSamplerAddressMode,

    /// addressModeV is a VkSamplerAddressMode value specifying the wrapping operation used when the j coordinate used to sample the image would be out of bounds.
    ///
    /// # Valid Usage
    ///  - If unnormalizedCoordinates is [`VK_TRUE`], addressModeV must each be either VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE or VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER
    ///  - If sampler Y′CBCR conversion is enabled, addressModeV must be VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE
    ///  - If the samplerMirrorClampToEdge feature is not enabled, and if the VK_KHR_sampler_mirror_clamp_to_edge extension is not enabled, addressModeV must not be VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then addressModeV must be either VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE or VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, then addressModeV must be either VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE or VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER
    ///
    /// # Valid Usage (Implicit)
    ///  - addressModeV must be a valid VkSamplerAddressMode value
    pub address_mode_v: VkSamplerAddressMode,

    /// addressModeW is a VkSamplerAddressMode value spcifying the wrapping operation used when the k coordinate used to sample the image would be out of bounds. If unnormalizedCoordinates is [`VK_TRUE`], addressModeW is ignored.
    ///
    /// # Valid Usage
    ///  - If sampler Y′CBCR conversion is enabled, addressModeW must be VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE
    ///  - If the samplerMirrorClampToEdge feature is not enabled, and if the VK_KHR_sampler_mirror_clamp_to_edge extension is not enabled, addressModeW must not be VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE
    ///
    /// # Valid Usage (Implicit)
    ///  - addressModeW must be a valid VkSamplerAddressMode value
    pub address_mode_w: VkSamplerAddressMode,

    /// mipLodBias is the bias to be added to mipmap LOD calculation and bias provided by image sampling functions in SPIR-V, as described in the LOD Operation section.
    ///
    /// # Valid Usage
    ///  - The absolute value of mipLodBias must be less than or equal to VkPhysicalDeviceLimits::maxSamplerLodBias
    ///  - If the VK_KHR_portability_subset extension is enabled, and VkPhysicalDevicePortabilitySubsetFeaturesKHR::samplerMipLodBias is VK_FALSE, mipLodBias must be zero
    pub mip_lod_bias: c_float,

    /// anisotropyEnable is [`VK_TRUE`] to enable anisotropic filtering, as described in the Texel Anisotropic Filtering section, or VK_FALSE otherwise.
    ///
    /// # Valid Usage
    ///  - If the samplerAnisotropy feature is not enabled, anisotropyEnable must be VK_FALSE
    ///  - If unnormalizedCoordinates is [`VK_TRUE`], anisotropyEnable must be VK_FALSE
    ///  - If sampler Y′CBCR conversion is enabled anisotropyEnable must be VK_FALSE
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then anisotropyEnable must be VK_FALSE
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, then anisotropyEnable must be VK_FALSE
    pub anisotropy_enable: VkBool32,

    /// maxAnisotropy is the anisotropy value clamp used by the sampler when anisotropyEnable is [`VK_TRUE`]. If anisotropyEnable is VK_FALSE, maxAnisotropy is ignored.
    ///
    /// # Valid Usage
    ///  - If anisotropyEnable is [`VK_TRUE`], maxAnisotropy must be between 1.0 and VkPhysicalDeviceLimits::maxSamplerAnisotropy, inclusive
    ///  - If either `mag_filter` or `min_filter` is [`VkFilter::CUBIC_EXT, anisotropyEnable must be VK_FALSE
    pub max_anisotropy: c_float,

    /// `compare_enable` is [`VK_TRUE`] to enable comparison against a reference value during lookups, or VK_FALSE otherwise.
    ///
    /// > Note: Some implementations will default to shader state if this member does not match.
    ///
    /// # Valid Usage
    ///  - If unnormalizedCoordinates is [`VK_TRUE`], `compare_enable` must be VK_FALSE
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then `compare_enable` must be VK_FALSE
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, then `compare_enable` must be VK_FALSE
    pub compare_enable: VkBool32,

    /// compareOp is a VkCompareOp value specifying the comparison operator to apply to fetched data before filtering as described in the Depth Compare Operation section.
    ///
    /// # Valid Usage
    ///  - If `compare_enable` is [`VK_TRUE`], compareOp must be a valid VkCompareOp value
    pub compare_op: VkCompareOp,

    /// minLod is used to clamp the minimum of the computed LOD value.
    ///
    /// # Valid Usage
    ///  - If unnormalizedCoordinates is [`VK_TRUE`], minLod must be zero
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then minLod must be zero
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, then minLod must be zero
    pub min_lod: c_float,

    /// maxLod is used to clamp the maximum of the computed LOD value. To avoid clamping the maximum value, set maxLod to the constant VK_LOD_CLAMP_NONE.
    ///
    /// # Valid Usage
    ///  - maxLod must be greater than or equal to minLod
    ///  - If unnormalizedCoordinates is [`VK_TRUE`], maxLod must be zero
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then maxLod must be zero
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, then maxLod must be zero
    pub max_lod: c_float,

    /// borderColor is a VkBorderColor value specifying the predefined border color to use.
    ///
    /// # Valid Usage
    ///  - If any of addressModeU, addressModeV or addressModeW are VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER, borderColor must be a valid VkBorderColor value
    ///  - If borderColor is one of VK_BORDER_COLOR_FLOAT_CUSTOM_EXT or VK_BORDER_COLOR_INT_CUSTOM_EXT, then a VkSamplerCustomBorderColorCreateInfoEXT must be included in the `next` chain
    ///  - If the customBorderColors feature is not enabled, borderColor must not be VK_BORDER_COLOR_FLOAT_CUSTOM_EXT or VK_BORDER_COLOR_INT_CUSTOM_EXT
    ///  - If borderColor is one of VK_BORDER_COLOR_FLOAT_CUSTOM_EXT or VK_BORDER_COLOR_INT_CUSTOM_EXT, and VkSamplerCustomBorderColorCreateInfoEXT::format is not VK_FORMAT_UNDEFINED, VkSamplerCustomBorderColorCreateInfoEXT::customBorderColor must be within the range of values representable in format
    ///  - If flags includes VK_SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM, and if addressModeU or addressModeV is VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER, then borderColor must be VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK
    pub border_color: VkBorderColor,

    /// unnormalizedCoordinates controls whether to use unnormalized or normalized texel coordinates to address texels of the image. When unnormalizedCoordinates is [`VK_TRUE`], the range of the image coordinates used to lookup the texel is in the range of zero to the image size in each dimension. When unnormalizedCoordinates is VK_FALSE, the range of image coordinates is zero to one.
    ///
    /// When unnormalizedCoordinates is [`VK_TRUE`], images the sampler is used with in the shader have the following requirements:
    ///  - The viewType must be either VK_IMAGE_VIEW_TYPE_1D or VK_IMAGE_VIEW_TYPE_2D.
    ///  - The image view must have a single layer and a single mip level.
    ///  - When unnormalizedCoordinates is [`VK_TRUE`], image built-in functions in the shader that use the sampler have the following requirements:
    ///  - The functions must not use projection.
    ///  - The functions must not use offsets.
    ///
    /// # Valid Usage
    ///  - If sampler Y′CBCR conversion is enabled, unnormalizedCoordinates must be VK_FALSE
    ///  - If flags includes VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT, then unnormalizedCoordinates must be VK_FALSE
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
