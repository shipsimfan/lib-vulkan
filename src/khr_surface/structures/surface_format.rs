use crate::{VkFormat, khr_surface::VkColorSpaceKhr};

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

/// Structure describing a supported swapchain format-color space pair
///
/// Provided by [`khr_surface`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceFormatKhr {
    /// `format` is a [`VkFormat`] that is compatible with the specified surface.
    pub format: VkFormat,

    /// `color_space` is a presentation [`VkColorSpaceKhr`] that is compatible with the surface.
    pub color_space: VkColorSpaceKhr,
}

impl Default for VkSurfaceFormatKhr {
    fn default() -> Self {
        VkSurfaceFormatKhr {
            format: VkFormat::Undefined,
            color_space: VkColorSpaceKhr::SRGBNonlinearKhr,
        }
    }
}
