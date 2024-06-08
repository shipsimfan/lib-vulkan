use crate::{VkColorSpaceKHR, VkFormat};

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

/// Structure describing a supported swapchain format-color space pair
///
/// Provided by [`khr_surface`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFormatKHR {
    /// `format` is a [`VkFormat`] that is compatible with the specified surface.
    pub format: VkFormat,

    /// `color_space` is a presentation [`VkColorSpaceKHR`] that is compatible with the surface.
    pub color_space: VkColorSpaceKHR,
}

impl Default for VkSurfaceFormatKHR {
    fn default() -> Self {
        VkSurfaceFormatKHR {
            format: VkFormat::Undefined,
            color_space: VkColorSpaceKHR::SRGBNonlinearKHR,
        }
    }
}
