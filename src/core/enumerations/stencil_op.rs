// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Stencil comparison function
///
/// # Description
/// For purposes of increment and decrement, the stencil bits are considered as an unsigned
/// integer.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkStencilOp {
    /// [`VkStencilOp::Keep`] keeps the current value.
    Keep = 0,

    /// [`VkStencilOp::Zero`] sets the value to 0.
    Zero = 1,

    /// [`VkStencilOp::Replace`] sets the value to reference.
    Replace = 2,

    /// [`VkStencilOp::IncrementAndClamp`] increments the current value and clamps to the maximum
    /// representable unsigned value.
    IncrementAndClamp = 3,

    /// [`VkStencilOp::DecrementAndClamp`] decrements the current value and clamps to 0.
    DecrementAndClamp = 4,

    /// [`VkStencilOp::Invert`] bitwise-inverts the current value.
    Invert = 5,

    /// [`VkStencilOp::IncrementAndWrap`] increments the current value and wraps to 0 when the
    /// maximum value would have been exceeded.
    IncrementAndWrap = 6,

    /// [`VkStencilOp::DecrementAndWrap`] decrements the current value and wraps to the maximum
    /// possible value when the value would go below 0.
    DecrementAndWrap = 7,
}
