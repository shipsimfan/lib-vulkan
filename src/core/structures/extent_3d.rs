// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying a three-dimensional extent
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExtent3D {
    /// `width` is the width of the extent.
    pub width: u32,

    /// `height` is the height of the extent.
    pub height: u32,

    /// `depth` is the depth of the extent.
    pub depth: u32,
}

impl Default for VkExtent3D {
    fn default() -> Self {
        VkExtent3D {
            width: 0,
            height: 0,
            depth: 0,
        }
    }
}
