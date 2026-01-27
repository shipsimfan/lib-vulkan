// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkFormat};

/// Specify how a component is swizzled
///
/// # Description
/// Setting the identity swizzle on a component is equivalent to setting the identity mapping on
/// that component.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkComponentSwizzle {
    /// [`VkComponentSwizzle::Identity`] specifies that the component is set to the identity
    /// swizzle.
    Identity = 0,

    /// [`VkComponentSwizzle::Zero`] specifies that the component is set to zero.
    Zero = 1,

    /// [`VkComponentSwizzle::One`] specifies that the component is set to either 1 or 1.0,
    /// depending on whether the type of the image view format is integer or floating-point
    /// respectively, as determined by the Format Definition section for each [`VkFormat`].
    One = 2,

    /// [`VkComponentSwizzle::R`] specifies that the component is set to the value of the R
    /// component of the image.
    R = 3,

    /// [`VkComponentSwizzle::G`] specifies that the component is set to the value of the G
    /// component of the image.
    G = 4,

    /// [`VkComponentSwizzle::B`] specifies that the component is set to the value of the B
    /// component of the image.
    B = 5,

    /// [`VkComponentSwizzle::A`] specifies that the component is set to the value of the A
    /// component of the image.
    A = 6,
}
