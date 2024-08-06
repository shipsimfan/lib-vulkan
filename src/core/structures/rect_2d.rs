use crate::{VkExtent2D, VkOffset2D};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying a two-dimensional subregion
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkRect2D {
    /// `offset` is a [`VkOffset2D`] specifying the rectangle offset.
    pub offset: VkOffset2D,

    /// `extent` is a [`VkExtent2D`] specifying the rectangle extent.
    pub extent: VkExtent2D,
}

impl Default for VkRect2D {
    fn default() -> Self {
        VkRect2D {
            offset: VkOffset2D::default(),
            extent: VkExtent2D::default(),
        }
    }
}
