// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Specifies the tiling arrangement of data in an image
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkImageTiling {
    /// [`VkImageTiling::Optimal`] specifies optimal tiling (texels are laid out in an
    /// implementation-dependent arrangement, for more efficient memory access).
    Optimal = 0,

    /// [`VkImageTiling::Linear`] specifies linear tiling (texels are laid out in memory in
    /// row-major order, possibly with some padding on each row).
    Linear = 1,

    /// [`VkImageTiling::DrmFormatModifierExt`] specifies that the image’s tiling is defined by a
    /// Linux DRM format modifier. The modifier is specified at image creation with
    /// [`VkImageDrmFormatModifierListCreateInfoExt`] or
    /// [`VkImageDrmFormatModifierExplicitCreateInfoExt`], and can be queried with
    /// [`VkGetImageDrmFormatModifierPropertiesExt`].
    ///
    /// Provided by [`ext_image_drm_format_modifier`]
    DrmFormatModifierExt = 1000158000,
}
