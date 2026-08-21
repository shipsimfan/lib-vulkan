// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Specify border color used for texture lookups
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkBorderColor {
    /// [`VkBorderColor::FloatTransparentBlack`] specifies a transparent, floating-point format,
    /// black color.
    FloatTransparentBlack = 0,

    /// [`VkBorderColor::IntTransparentBlack`] specifies a transparent, integer format, black
    /// color.
    IntTransparentBlack = 1,

    /// [`VkBorderColor::FloatOpaqueBlack`] specifies an opaque, floating-point format, black
    /// color.
    FloatOpaqueBlack = 2,

    /// [`VkBorderColor::IntOpaqueBlack`] specifies an opaque, integer format, black color.
    IntOpaqueBlack = 3,

    /// [`VkBorderColor::FloatOpaqueWhite`] specifies an opaque, floating-point format, white
    /// color.
    FloatOpaqueWhite = 4,

    /// [`VkBorderColor::IntOpaqueWhite`] specifies an opaque, integer format, white color.
    IntOpaqueWhite = 5,

    /// [`VkBorderColor::FloatCustomExt`] specifies that a
    /// [`VkSamplerCustomBorderColorCreateInfoExt`] structure is included in the
    /// [`VkSamplerCreateInfo::next`] chain containing the color data in floating-point format.
    ///
    /// Provided by [`ext_custom_border_color`]
    FloatCustomExt = 1000287003,

    /// [`VkBorderColor::IntCustomExt`] specifies that a
    /// [`VkSamplerCustomBorderColorCreateInfoExt`] structure is included in the
    /// [`VkSamplerCreateInfo::next`] chain containing the color data in integer format.
    ///
    /// Provided by [`ext_custom_border_color`]
    IntCustomExt = 1000287004,
}
