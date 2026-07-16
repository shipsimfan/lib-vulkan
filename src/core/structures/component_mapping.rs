use crate::VkComponentSwizzle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying a color component mapping
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkComponentMapping {
    /// `r` is a [`VkComponentSwizzle`] specifying the component value placed in the R component of
    /// the output vector.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r` must be a valid [`VkComponentSwizzle`] value
    pub r: VkComponentSwizzle,

    /// `g` is a [`VkComponentSwizzle`] specifying the component value placed in the G component of
    /// the output vector.
    ///
    /// # Valid Usage (Implicit)
    ///  - `g` must be a valid [`VkComponentSwizzle`] value
    pub g: VkComponentSwizzle,

    /// `b` is a [`VkComponentSwizzle`] specifying the component value placed in the B component of
    /// the output vector.
    ///
    /// # Valid Usage (Implicit)
    ///  - `b` must be a valid [`VkComponentSwizzle`] value
    pub b: VkComponentSwizzle,

    /// `a` is a [`VkComponentSwizzle`] specifying the component value placed in the A component of
    /// the output vector.
    ///
    /// # Valid Usage (Implicit)
    ///  - `a` must be a valid [`VkComponentSwizzle`] value
    pub a: VkComponentSwizzle,
}

const impl Default for VkComponentMapping {
    fn default() -> Self {
        VkComponentMapping {
            r: VkComponentSwizzle::Identity,
            g: VkComponentSwizzle::Identity,
            b: VkComponentSwizzle::Identity,
            a: VkComponentSwizzle::Identity,
        }
    }
}
