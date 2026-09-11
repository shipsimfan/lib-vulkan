use crate::{VkCommandBuffer, VkFilter, VkImage, VkImageBlit, VkImageLayout};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_REMAINING_ARRAY_LAYERS, VK_VERSION_1_0, VkCommandPool, VkDevice, VkDeviceMemory, VkFormat,
    VkImageCreateFlag, VkImageCreateInfo, VkImageType, VkImageUsageFlag, VkQueueFlag,
    VkSampleCountFlag, VkSamplerAddressMode, VkSamplerMipmapMode,
};

/// Copy regions of an image, potentially performing format conversion
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command will be recorded.
///  - `src_image` is the source image.
///  - `src_image_layout` is the layout of the source image subresources for the blit.
///  - `dst_image` is the destination image.
///  - `dst_image_layout` is the layout of the destination image subresources for the blit.
///  - `region_count` is the number of regions to blit.
///  - `regions` is a pointer to an array of [`VkImageBlit`] structures specifying the regions to
///    blit.
///  - `filter` is a [`VkFilter`] specifying the filter to apply if the blits require scaling.
///
/// # Description
/// [`VkCmdBlitImage`] must not be used for multisampled source or destination images. Use
/// [`VkCmdResolveImage`] for this purpose.
///
/// As the sizes of the source and destination extents can differ in any dimension, texels in the
/// source extent are scaled and filtered to the destination extent. Scaling occurs via the
/// following operations:
///  - For each destination texel, the integer coordinate of that texel is converted to an
///    unnormalized texture coordinate, using the effective inverse of the equations described in
///    unnormalized to integer conversion:
///    - `ubase = i + ½`
///    - `vbase = j + ½`
///    - `wbase = k + ½`
///  - These base coordinates are then offset by the first destination offset:
///    - `uoffset = ubase - xdst0`
///    - `voffset = vbase - ydst0`
///    - `woffset = wbase - zdst0`
///    - `aoffset = a - base_array_count_dst`
///  - The scale is determined from the source and destination regions, and applied to the offset
///    coordinates:
///    - `scaleu = (xsrc1 - xsrc0) / (xdst1 - xdst0)`
///    - `scalev = (ysrc1 - ysrc0) / (ydst1 - ydst0)`
///    - `scalew = (zsrc1 - zsrc0) / (zdst1 - zdst0)`
///    - `uscaled = uoffset × scaleu`
///    - `vscaled = voffset × scalev`
///    - `wscaled = woffset × scalew`
///  - Finally the source offset is added to the scaled coordinates, to determine the final
///    unnormalized coordinates used to sample from `src_image`:
///    - `u = uscaled + xsrc0`
///    - `v = vscaled + ysrc0`
///    - `w = wscaled + zsrc0`
///    - `q = mip_level`
///    - `a = aoffset + base_array_count_src`
///
/// These coordinates are used to sample from the source image with the filter mode equal to that
/// of `filter`, a mipmap mode of [`VkSamplerMipmapMode::Nearest`] and an address mode of
/// [`VkSamplerAddressMode::ClampToEdge`]. Implementations must clamp at the edge of the source
/// image, and may additionally clamp to the edge of the source region.
///
/// Blits are done layer by layer starting with the `base_array_layer` member of `src_subresource`
/// for the source and `dst_subresource` for the destination. `layer_count` layers are blitted to
/// the destination image.
///
/// When blitting 3D textures, slices in the destination region bounded by `dst_offsets[0].z` and
/// `dst_offsets[1].z` are sampled from slices in the source region bounded by `src_offsets[0].z`
/// and `src_offsets[1].z`. If the filter parameter is [`VkFilter::Linear`] then the value sampled
/// from the source image is taken by doing linear filtering using the interpolated z coordinate
/// represented by w in the previous equations. If the filter parameter is [`VkFilter::Nearest`]
/// then the value sampled from the source image is taken from the single nearest slice, with an
/// implementation-dependent arithmetic rounding mode.
///
/// The following filtering and conversion rules apply:
///  - Integer formats can only be converted to other integer formats with the same signedness.
///  - No format conversion is supported between depth/stencil images. The formats must match.
///  - Format conversions on unorm, snorm, scaled and packed float formats of the copied aspect of
///    the image are performed by first converting the pixels to float values.
///  - For sRGB source formats, nonlinear RGB values are converted to linear representation prior
///    to filtering.
///  - After filtering, the float values are first clamped and then cast to the destination image
///    format. In case of sRGB destination format, linear RGB values are converted to nonlinear
///    representation before writing the pixel to the image.
///
/// Signed and unsigned integers are converted by first clamping to the representable range of the
/// destination format, then casting the value.
///
/// # Valid Usage
///  - If `command_buffer` is an unprotected command buffer and `protected_no_fault` is not
///    supported, `src_image` must not be a protected image
///  - If `command_buffer` is an unprotected command buffer and `protected_no_fault` is not
///    supported, `dst_image` must not be a protected image
///  - If `command_buffer` is a protected command buffer and `protected_no_fault` is not supported,
///    `dst_image` must not be an unprotected image
///  - The union of all destination regions, specified by the elements of `regions`, must not
///    overlap in memory with any texel that may be sampled during the blit operation
///  - The format features of `src_image` must contain [`VkFormatFeature::BlitSrc`]
///  - `src_image` must not use a format that requires a sampler Y′CBCR conversion
///  - `src_image` must have been created with the [`VkImageUsageFlag::TransferSrc`] usage flag set
///  - If `src_image` is non-sparse then it must be bound completely and contiguously to a single
///    [`VkDeviceMemory`] object
///  - `src_image_layout` must specify the layout of the image subresources of `src_image`
///    specified in `regions` at the time this command is executed on a [`VkDevice`]
///  - `src_image_layout` must be [`VkImageLayout::SharedPresentKhr`],
///    [`VkImageLayout::TransferSrcOptimal`] or [`VkImageLayout::General`]
///  - If `src_image` and `dst_image` are the same, and an element of `regions` contains the
///    `src_subresource` and `dst_subresource` with matching `mip_level` and overlapping array
///    layers, then the `src_image_layout` and `dst_image_layout` must be
///    [`VkImageLayout::General`] or [`VkImageLayout::SharedPresentKhr`]
///  - The format features of `dst_image` must contain [`VkFormatFeature::BlitDst`]
///  - `dst_image` must not use a format that requires a sampler Y′CBCR conversion
///  - `dst_image` must have been created with the [`VkImageUsageFlag::TransferDst`] usage flag set
///  - If `dst_image` is non-sparse then it must be bound completely and contiguously to a single
///    [`VkDeviceMemory`] object
///  - `dst_image_layout` must specify the layout of the image subresources of `dst_image`
///    specified in `regions` at the time this command is executed on a [`VkDevice`]
///  - `dst_image_layout` must be [`VkImageLayout::SharedPresentKhr`],
///    [`VkImageLayout::TransferDstOptimal`] or [`VkImageLayout::General`]
///  - If either of `src_image` or `dst_image` was created with a signed integer [`VkFormat`], the
///    other must also have been created with a signed integer [`VkFormat`]
///  - If either of `src_image` or `dst_image` was created with an unsigned integer color
///    [`VkFormat`], the other must also have been created with an unsigned integer color
///    [`VkFormat`]
///  - If either of `src_image` or `dst_image` was created with a depth/stencil format, the other
///    must have exactly the same format
///  - If `src_image` was created with a depth/stencil format, filter must be [`VkFilter::Nearest`]
///  - `src_image` must have been created with a samples value of [`VkSampleCountFlag::_1`]
///  - `dst_image` must have been created with a samples value of [`VkSampleCountFlag::_1`]
///  - If `filter` is [`VkFilter::Linear`], then the format features of `src_image` must contain
///    [`VkFormatFeature::SampledImageFilterLinear`]
///  - If `filter` is [`VkFilter::CubicExt`], then the format features of `src_image` must contain
///    [`VkFormatFeature::SampledImageFilterCubic`]
///  - If `filter` is [`VkFilter::CubicExt`], `src_image` must be of type [`VkImageType::_2d`]
///  - The `src_subresource.mip_level` member of each element of `regions` must be less than the
///    `mip_level`s specified in [`VkImageCreateInfo`] when `src_image` was created
///  - The `dst_subresource.mip_level` member of each element of `regions` must be less than the
///    `mip_level`s specified in [`VkImageCreateInfo`] when `dst_image` was created
///  - If `src_subresource.layer_count` is not [`VK_REMAINING_ARRAY_LAYERS`],
///    `src_subresource.base_array_layer + src_subresource.layer_count` of each element of
///    `regions` must be less than or equal to the `array_layers` specified in
///    [`VkImageCreateInfo`] when `src_image` was created
///  - If `dst_subresource.layer_count` is not [`VK_REMAINING_ARRAY_LAYERS`],
///    `dst_subresource.base_array_layer + dst_subresource.layer_count` of each element of
///    `regions` must be less than or equal to the `array_layers` specified in
///    [`VkImageCreateInfo`] when `dst_image` was created
///  - `dst_image` and `src_image` must not have been created with `flags` containing
///    [`VkImageCreateFlag::SubsampledExt`]
///  - If the `maintenance8` feature is enabled and `src_image` is of type [`VkImageType::_3d`],
///    then for each element of `regions`, `src_subresource.base_array_layer` must be 0, and
///    `src_subresource.layer_count` and `dst_subresource.layer_count` must each be 1
///  - If the `maintenance8` feature is enabled and `dst_image` is of type [`VkImageType::_3d`],
///    then for each element of `regions`, `dst_subresource.base_array_layer` must be 0, and
///    `src_subresource.layer_count` and `dst_subresource.layer_count` must each be 1
///  - If the `maintenance8` feature is enabled, `dst_image` is [`VkImageType::_3d`], and
///    `src_image` is not of type [`VkImageType::_3d`], then for each element of `regions`, the
///    absolute difference of the z member of each member of `dst_offsets` must equal
///    `src_subresource.layer_count`
///  - If the `maintenance8` feature is enabled, `src_image` is [`VkImageType::_3d`], and
///    `dst_image` is not of type [`VkImageType::_3d`], then for each element of `regions`, the
///    absolute difference of the z member of each member of `src_offsets` must equal
///    `dst_subresource.layer_count`
///  - If the `maintenance8` feature is not enabled and either `src_image` or `dst_image` is of
///    type [`VkImageType::_3d`], then for each element of `regions`,
///    `src_subresource.base_array_layer` and `dst_subresource.base_array_layer` must each be 0,
///    and `src_subresource.layer_count` and `dst_subresource.layer_count` must each be 1
///  - For each element of `regions`, `src_subresource`.aspectMask must specify aspects present in
///    `src_image`
///  - For each element of `regions`, `dst_subresource`.aspectMask must specify aspects present in
///    `dst_image`
///  - For each element of `regions`, `src_offsets[0].x` and `src_offsets[1].x` must both be
///    greater than or equal to 0 and less than or equal to the width of the specified
///    `src_subresource` of `src_image`
///  - For each element of `regions`, `src_offsets[0].y` and `src_offsets[1].y` must both be
///    greater than or equal to 0 and less than or equal to the height of the specified
///    `src_subresource` of `src_image`
///  - If `src_image` is of type [`VkImageType::_1d`], then for each element of `regions`,
///    `src_offsets[0].y` must be 0 and `src_offsets[1].y` must be 1
///  - For each element of `regions`, `src_offsets[0].z` and `src_offsets[1].z` must both be
///    greater than or equal to 0 and less than or equal to the depth of the specified
///    `src_subresource` of `src_image`
///  - If `src_image` is of type [`VkImageType::_1d`] or [`VkImageType::_2d`], then for each
///    element of `regions`, `src_offsets[0].z` must be 0 and `src_offsets[1].z` must be 1
///  - For each element of `regions`, `dst_offsets[0].x` and `dst_offsets[1].x` must both be
///    greater than or equal to 0 and less than or equal to the width of the specified
///    `dst_subresource` of `dst_image`
///  - For each element of `regions`, `dst_offsets[0].y` and `dst_offsets[1].y` must both be
///    greater than or equal to 0 and less than or equal to the height of the specified
///    `dst_subresource` of `dst_image`
///  - If `dst_image` is of type [`VkImageType::_1d`], then for each element of `regions`,
///    `dst_offsets[0].y` must be 0 and `dst_offsets[1].y` must be 1
///  - For each element of `regions`, `dst_offsets[0].z` and `dst_offsets[1].z` must both be
///    greater than or equal to 0 and less than or equal to the depth of the specified
///    `dst_subresource` of `dst_image`
///  - If `dst_image` is of type [`VkImageType::_1d`] or [`VkImageType::_2d`], then for each
///    element of `regions`, `dst_offsets[0].z` must be 0 and `dst_offsets[1].z` must be 1
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `src_image` must be a valid [`VkImage`] handle
///  - `src_image_layout` must be a valid [`VkImageLayout`] value
///  - `dst_image` must be a valid [`VkImage`] handle
///  - `dst_image_layout` must be a valid [`VkImageLayout`] value
///  - `regions` must be a valid pointer to an array of `region_count` valid [`VkImageBlit`]
///    structures
///  - filter must be a valid [`VkFilter`] value
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Graphics`] operations
///  - This command must only be called outside of a render pass instance
///  - This command must not be called between suspended render pass instances
///  - This command must only be called outside of a video coding scope
///  - `region_count` must be greater than 0
///  - Each of `command_buffer`, `dst_image`, and `src_image` must have been created, allocated, or
///    retrieved from the same [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdBlitImage = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_image: VkImage,
    src_image_layout: VkImageLayout,
    dst_image: VkImage,
    dst_image_layout: VkImageLayout,
    region_count: u32,
    regions: *const VkImageBlit,
    filter: VkFilter,
);

/// The name of [`VkCmdBindVertexBuffers`]
pub const VK_CMD_BLIT_IMAGE: &CStr = c"vkCmdBlitImage";
