// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying a two-dimensional extent
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExtent2D {
    /// `width` is the width of the extent.
    pub width: u32,

    /// `height` is the height of the extent.
    pub height: u32,
}

impl Default for VkExtent2D {
    fn default() -> Self {
        VkExtent2D {
            width: 0,
            height: 0,
        }
    }
}
