use std::ffi::c_float;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkPhysicalDeviceProperties};

/// Structure specifying a viewport
///
/// # Description
/// The application can specify a negative term for `height`, which has the effect of negating the
/// y coordinate in clip space before performing the transform. When using a negative height, the
/// application should also adjust the `y` value to point to the lower left corner of the viewport
/// instead of the upper left corner. Using the negative `height` allows the application to avoid
/// having to negate the y component of the `Position` output from the last pre-rasterization
/// shader stage.
///
/// The `width` and `height` of the implementation-dependent maximum viewport dimensions must be
/// greater than or equal to the width and height of the largest image which can be created and
/// attached to a framebuffer.
///
/// The floating-point viewport bounds are represented with an implementation-dependent precision.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkViewport {
    /// `x` is the viewport’s upper left corner `(x, y)`.
    ///
    /// # Valid Usage
    ///  - `x` must be greater than or equal to `viewport_bounds_range[0]`
    ///  - `(x + width)` must be less than or equal to `viewport_bounds_range[1]`
    pub x: c_float,

    /// `y` is the viewport’s upper left corner `(x, y)`.
    ///
    /// # Valid Usage
    ///  - `y` must be greater than or equal to `viewport_bounds_range[0]`
    ///  - `y` must be less than or equal to `viewport_bounds_range[1]`
    ///  - `(y + height)` must be greater than or equal to `viewport_bounds_range[0]`
    ///  - `(y + height)` must be less than or equal to `viewport_bounds_range[1]`
    pub y: c_float,

    /// `width` is the viewport’s width.
    ///
    /// # Valid Usage
    ///  - `width` must be greater than 0.0
    ///  - `width` must be less than or equal to
    ///    `VkPhysicalDeviceLimits::max_viewport_dimensions[0]`
    pub width: c_float,

    /// `height` is the viewport’s height.
    ///
    /// # Valid Usage
    ///  - If the [`khr_maintenance1`] extension is not enabled, the
    ///    [`amd_negative_viewport_height`] extension is not enabled, and
    ///    [`VkPhysicalDeviceProperties::api_version`] is less than Vulkan 1.1, `height` must be
    ///    greater than 0.0
    ///  - The absolute value of `height` must be less than or equal to
    ///    `VkPhysicalDeviceLimits::max_viewport_dimensions[1]`
    pub height: c_float,

    /// `min_depth` is the minimum of the depth range for the viewport.
    ///
    /// # Valid Usage
    ///  - If the [`ext_depth_range_unrestricted`] extension is not enabled, `min_depth` must be
    ///    between 0.0 and 1.0, inclusive
    pub min_depth: c_float,

    /// `max_depth` is the maximum of the depth range for the viewport.
    ///
    /// # Valid Usage
    ///  - If the [`ext_depth_range_unrestricted`] extension is not enabled, `max_depth` must be
    ///    between 0.0 and 1.0, inclusive
    pub max_depth: c_float,
}

impl const Default for VkViewport {
    fn default() -> Self {
        VkViewport {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            min_depth: 0.0,
            max_depth: 0.0,
        }
    }
}
