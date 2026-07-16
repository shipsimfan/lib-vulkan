use crate::VkImageAspectFlags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkImageAspectFlag, VkImageView};

/// Structure specifying an image subresource range
///
/// # Description
/// The number of mipmap levels and array layers must be a subset of the image subresources in the
/// image. If an application wants to use all mip levels or layers in an image after the
/// `base_mip_level` or `base_array_layer`, it can set `level_count` and `layer_count` to the
/// special values [`VK_REMAINING_MIP_LEVELS`] and [`VK_REMAINING_ARRAY_LAYERS`] without knowing
/// the exact number of mip levels or layers.
///
/// For cube and cube array image views, the layers of the image view starting at
/// `base_array_layer` correspond to faces in the order +X, -X, +Y, -Y, +Z, -Z. For cube arrays,
/// each set of six sequential layers is a single cube, so the number of cube maps in a cube map
/// array view is `layer_count / 6`, and image array layer (`base_array_layer + i`) is face index
/// `(i mod 6)` of cube `i / 6`. If the number of layers in the view, whether set explicitly in
/// `layer_count` or implied by [`VK_REMAINING_ARRAY_LAYERS`], is not a multiple of 6, the last
/// cube map in the array must not be accessed.
///
/// `aspect_mask` must be only [`VkImageAspectFlag::Color`], [`VkImageAspectFlag::Depth`] or
/// [`VkImageAspectFlag::Stencil`] if format is a color, depth-only or stencil-only format,
/// respectively, except if format is a multi-planar format. If using a depth/stencil format with
/// both depth and stencil components, `aspect_mask` must include at least one of
/// [`VkImageAspectFlag::Depth`] and [`VkImageAspectFlag::Stencil`], and can include both.
///
/// When the [`VkImageSubresourceRange`] structure is used to select a subset of the slices of a 3D
/// image’s mip level in order to create a 2D or 2D array image view of a 3D image created with
/// [`VkImageCreateFlag::2dArrayCompatible`], `base_array_layer` and `layer_count` specify the
/// first slice index and the number of slices to include in the created image view. Such an image
/// view can be used as a framebuffer attachment that refers only to the specified range of slices
/// of the selected mip level. If the maintenance9 feature is not enabled, any layout transitions
/// performed on such an attachment view during a render pass instance still apply to the entire
/// subresource referenced which includes all the slices of the selected mip level.
///
/// When using an image view of a depth/stencil image to populate a descriptor set (e.g. for
/// sampling in the shader, or for use as an input attachment), the `aspect_mask` must only include
/// one bit, which selects whether the image view is used for depth reads (i.e. using a
/// floating-point sampler or input attachment in the shader) or stencil reads (i.e. using an
/// unsigned integer sampler or input attachment in the shader). When an image view of a
/// depth/stencil image is used as a depth/stencil framebuffer attachment, the `aspect_mask` is
/// ignored and both depth and stencil image subresources are used.
///
/// When creating a [`VkImageView`], if sampler Y′CBCR conversion is enabled in the sampler, the
/// `aspect_mask` of a subresourceRange used by the [`VkImageView`] must be
/// [`VkImageAspectFlag::Color`].
///
/// When creating a [`VkImageView`], if sampler Y′CBCR conversion is not enabled in the sampler and
/// the image format is multi-planar format, the image must have been created with
/// [`VkImageCreateFlag::MutableFormat`], and the `aspect_mask` of the [`VkImageView`]’s
/// `subresource_range` must be [`VkImageAspectFlag::Plane0`], [`VkImageAspectFlag::Plane1`]
/// or [`VkImageAspectFlag::Plane2`].
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageSubresourceRange {
    /// `aspect_mask` is a bitmask of [`VkImageAspectFlags`] specifying which aspect(s) of the
    /// image are included in the view.
    ///
    /// # Valid Usage
    ///  - If `aspect_mask` includes [`VkImageAspectFlag::Color`], then it must not include any
    ///    of [`VkImageAspectFlag::Plane0`], [`VkImageAspectFlag::Plane1`], or
    ///    [`VkImageAspectFlag::Plane2`]
    ///  - `aspect_mask` must not include `VkImageAspectFlag::MemoryPlaneIExt` for any index `I`
    ///
    /// # Valid Usage (Implicit)
    ///  - `aspect_mask` must be a valid combination of [`VkImageAspectFlag`] values
    ///  - `aspect_mask` must not be 0
    pub aspect_mask: VkImageAspectFlags,

    /// `base_mip_level` is the first mipmap level accessible to the view.
    pub base_mip_level: u32,

    /// `level_count` is the number of mipmap levels (starting from `base_mip_level`) accessible to
    /// the view.
    ///
    /// # Valid Usage
    ///  - If `level_count` is not [`VK_REMAINING_MIP_LEVELS`], it must be greater than 0
    pub level_count: u32,

    /// `base_array_layer` is the first array layer accessible to the view.
    pub base_array_layer: u32,

    /// `layer_count` is the number of array layers (starting from `base_array_layer`) accessible
    /// to the view.
    ///
    /// # Valid Usage
    ///  - If `layer_count` is not [`VK_REMAINING_ARRAY_LAYERS`], it must be greater than 0
    pub layer_count: u32,
}

const impl Default for VkImageSubresourceRange {
    fn default() -> Self {
        VkImageSubresourceRange {
            aspect_mask: VkImageAspectFlags::empty(),
            base_mip_level: 0,
            level_count: 0,
            base_array_layer: 0,
            layer_count: 0,
        }
    }
}
