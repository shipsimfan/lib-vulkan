// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VkDynamicState};

/// Framebuffer logical operations
///
/// # Description
/// Logical operations are controlled by the `logic_op_enable` and `logic_op` members of
/// [`VkPipelineColorBlendStateCreateInfo`]. The `logic_op_enable` state can also be controlled by
/// [`VkCmdSetLogicOpEnableExt`] if graphics pipeline is created with
/// [`VkDynamicState::LogicOpEnableExt`] set in
/// [`VkPipelineDynamicStateCreateInfo::dynamic_states`]. The `logic_op` state can also be
/// controlled by [`VkCmdSetLogicOpExt`] if graphics pipeline is created with
/// [`VkDynamicState::LogicOpExt`] set in [`VkPipelineDynamicStateCreateInfo::dynamic_states`]. If
/// `logic_op_enable` is [`VK_TRUE`], then a logical operation selected by `logic_op` is applied
/// between each color attachment and the fragment’s corresponding output value, and blending of
/// all attachments is treated as if it were disabled. Any attachments using color formats for
/// which logical operations are not supported simply pass through the color values unmodified. The
/// logical operation is applied independently for each of the red, green, blue, and alpha
/// components.
///
/// The logical operations supported by Vulkan follow the following definitions:
///  - `¬` is bitwise invert,
///  - `∧` is bitwise and,
///  - `∨` is bitwise or,
///  - `⊕` is bitwise exclusive or,
///  - `s` is the fragment’s Rs0, Gs0, Bs0 or As0 component value for the fragment output
///    corresponding to the color attachment being updated, and
///  - `d` is the color attachment’s R, G, B or A component value.
///
/// The result of the logical operation is then written to the color attachment as controlled by
/// the component write mask.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkLogicOp {
    /// 0
    Clear = 0,

    /// `s ∧ d`
    And = 1,

    /// `s ∧ ¬ d`
    AndReverse = 2,

    /// `s`
    Copy = 3,

    /// `¬ s ∧ d`
    AndInverted = 4,

    /// `d`
    NoOp = 5,

    /// `s ⊕ d`
    Xor = 6,

    /// `s ∨ d`
    Or = 7,

    /// `¬ (s ∨ d)`
    Nor = 8,

    /// `¬ (s ⊕ d)`
    Equivalent = 9,

    /// `¬ d`
    Invert = 10,

    /// `s ∨ ¬ d`
    OrReverse = 11,

    /// `¬ s`
    CopyInverted = 12,

    /// `¬ s ∨ d`
    OrInverted = 13,

    /// `¬ (s ∧ d)`
    Nand = 14,

    /// all 1s
    Set = 15,
}
