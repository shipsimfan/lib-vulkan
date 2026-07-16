use crate::{VkCompareOp, VkStencilOp};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying stencil operation state
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkStencilOpState {
    /// `fail_op` is a [`VkStencilOp`] value specifying the action performed on samples that fail
    /// the stencil test.
    ///
    /// # Valid Usage (Implicit)
    ///  - `fail_op` must be a valid [`VkStencilOp`] value
    pub fail_op: VkStencilOp,

    /// `pass_op` is a [`VkStencilOp`] value specifying the action performed on samples that pass
    /// both the depth and stencil tests.
    ///
    /// # Valid Usage (Implicit)
    ///  - `pass_op` must be a valid [`VkStencilOp`] value
    pub pass_op: VkStencilOp,

    /// `depth_fail_op` is a [`VkStencilOp`] value specifying the action performed on samples that
    /// pass the stencil test and fail the depth test.
    ///
    /// # Valid Usage (Implicit)
    ///  - `depth_fail_op` must be a valid [`VkStencilOp`] value
    pub depth_fail_op: VkStencilOp,

    /// `compare_op` is a [`VkCompareOp`] value specifying the comparison operator used in the
    /// stencil test.
    ///
    /// # Valid Usage (Implicit)
    ///  - `compare_op` must be a valid [`VkCompareOp`] value
    pub compare_op: VkCompareOp,

    /// `compare_mask` selects the bits of the unsigned integer stencil values participating in the
    /// stencil test.
    pub compare_mask: u32,

    /// `write_mask` selects the bits of the unsigned integer stencil values updated by the stencil
    /// test in the stencil framebuffer attachment.
    pub write_mask: u32,

    /// `reference` is an integer stencil reference value that is used in the unsigned stencil
    /// comparison.
    pub reference: u32,
}

const impl Default for VkStencilOpState {
    fn default() -> Self {
        VkStencilOpState {
            fail_op: VkStencilOp::Keep,
            pass_op: VkStencilOp::Keep,
            depth_fail_op: VkStencilOp::Keep,
            compare_op: VkCompareOp::Never,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        }
    }
}
