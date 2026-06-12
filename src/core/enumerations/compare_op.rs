// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Comparison operator for depth, stencil, and sampler operations
///
/// # Description
/// Comparison operators compare a reference and a test value, and return a true (“passed”) or
/// false (“failed”) value depending on the comparison operator chosen.
///
/// Comparison operators are used for:
///  - The Depth Compare Operation operator for a sampler, specified by
///    [`VkSamplerCreateInfo::compare_op`].
///  - The stencil comparison operator for the stencil test, specified by
///    `VkCmdSetStencilOp::compare_op` or [`VkStencilOpState::compare_op`].
///  - The Depth Comparison operator for the depth test, specified by
///    `VkCmdSetDepthCompareOp::depthCompareOp` or
///    [`VkPipelineDepthStencilStateCreateInfo::depth_compare_op`].
///
/// Each such use describes how the reference and test values for that comparison are determined.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkCompareOp {
    /// [`VkCompareOp::Never`] specifies that the comparison always evaluates false.
    Never = 0,

    /// [`VkCompareOp::Less`] specifies that the comparison evaluates reference < test.
    Less = 1,

    /// [`VkCompareOp::Equal`] specifies that the comparison evaluates reference = test.
    Equal = 2,

    /// [`VkCompareOp::LessOrEqual`] specifies that the comparison evaluates reference ≤ test.
    LessOrEqual = 3,

    /// [`VkCompareOp::Greater`] specifies that the comparison evaluates reference > test.
    Greater = 4,

    /// [`VkCompareOp::NotEqual`] specifies that the comparison evaluates reference ≠ test.
    NotEqual = 5,

    /// [`VkCompareOp::GreaterOrEqual`] specifies that the comparison evaluates reference ≥ test.
    GreaterOrEqual = 6,

    /// [`VkCompareOp::Always`] specifies that the comparison always evaluates true.
    Always = 7,
}
