use crate::{VkBlendFactor, VkBlendOp, VkBool32, VkColorComponentFlags};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_VERSION_1_0, VkColorComponentFlag};

/// Structure specifying a pipeline color blend attachment state
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineColorBlendAttachmentState {
    /// `blend_enable` controls whether blending is enabled for the corresponding color attachment.
    /// If blending is not enabled, the source fragment’s color for that attachment is passed
    /// through unmodified.
    pub blend_enable: VkBool32,

    /// `src_color_blend_factor` selects which blend factor is used to determine the source factors
    /// `(Sr, Sg, Sb)`.
    ///
    /// # Valid Usage
    ///  - If the `dual_src_blend` feature is not enabled, `src_color_blend_factor` must not be
    ///    [`VkBlendFactor::Src1Color`], [`VkBlendFactor::OneMinusSrc1Color`],
    ///    [`VkBlendFactor::Src1Alpha`], or [`VkBlendFactor::OneMinusSrc1Alpha`]
    ///  - If the [`khr_portability_subset`] extension is enabled, and
    ///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::constant_alpha_color_blend_factors`] is
    ///    [`VK_FALSE`], `src_color_blend_factor` must not be [`VkBlendFactor::ConstantAlpha`] or
    ///    [`VkBlendFactor::OneMinusConstantAlpha`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `src_color_blend_factor` must be a valid [`VkBlendFactor`] value
    pub src_color_blend_factor: VkBlendFactor,

    /// `dst_color_blend_factor` selects which blend factor is used to determine the destination
    /// factors `(Dr, Dg, Db)`.
    ///
    /// # Valid Usage
    ///  - If the `dual_src_blend` feature is not enabled, `dst_color_blend_factor` must not be
    ///    [`VkBlendFactor::Src1Color`], [`VkBlendFactor::OneMinusSrc1Color`],
    ///    [`VkBlendFactor::Src1Alpha`], or [`VkBlendFactor::OneMinusSrc1Alpha`]
    ///  - If the [`khr_portability_subset`] extension is enabled, and
    ///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::constant_alpha_color_blend_factors`] is
    ///    [`VK_FALSE`], `dst_color_blend_factor` must not be [`VkBlendFactor::ConstantAlpha`] or
    ///    [`VkBlendFactor::OneMinusConstantAlpha`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `dst_color_blend_factor` must be a valid [`VkBlendFactor`] value
    pub dst_color_blend_factor: VkBlendFactor,

    /// `color_blend_op` selects which blend operation is used to calculate the RGB values to write
    /// to the color attachment.
    ///
    /// # Valid Usage
    ///  - If either of `color_blend_op` or `alpha_blend_op` is an advanced blend operation, then
    ///    `color_blend_op` must equal `alpha_blend_op`
    ///  - If
    ///    [`VkPhysicalDeviceBlendOperationAdvancedPropertiesExt::advanced_blend_independent_blend`]
    ///    is [`VK_FALSE`] and `color_blend_op` is an advanced blend operation, then
    ///    `color_blend_op` must be the same for all attachments
    ///  - If
    ///    [`VkPhysicalDeviceBlendOperationAdvancedPropertiesExt::advanced_blend_all_operations`]
    ///    is [`VK_FALSE`], then `color_blend_op` must not be [`VkBlendOp::ZeroExt`],
    ///    [`VkBlendOp::SrcExt`], [`VkBlendOp::DstExt`], [`VkBlendOp::SrcOverExt`],
    ///    [`VkBlendOp::DstOverExt`], [`VkBlendOp::SrcInExt`], [`VkBlendOp::DstInExt`],
    ///    [`VkBlendOp::SrcOutExt`], [`VkBlendOp::DstOutExt`], [`VkBlendOp::SrcAtopExt`],
    ///    [`VkBlendOp::DstAtopExt`], [`VkBlendOp::XorExt`], [`VkBlendOp::InvertExt`],
    ///    [`VkBlendOp::InvertRgbExt`], [`VkBlendOp::LinearDodgeExt`],
    ///    [`VkBlendOp::LinearBurnExt`], [`VkBlendOp::VividLightExt`],
    ///    [`VkBlendOp::LinearLightExt`], [`VkBlendOp::PinLightExt`], [`VkBlendOp::HardMixExt`],
    ///    [`VkBlendOp::PlusExt`], [`VkBlendOp::PlusClampedExt`],
    ///    [`VkBlendOp::PlusClampedAlphaExt`], [`VkBlendOp::PlusDarkerExt`],
    ///    [`VkBlendOp::MinusExt`], [`VkBlendOp::MinusClampedExt`], [`VkBlendOp::ContrastExt`],
    ///    [`VkBlendOp::InvertOvgExt`], [`VkBlendOp::RedExt`], [`VkBlendOp::GreenExt`], or
    ///    [`VkBlendOp::BlueExt`]
    ///  - If `color_blend_op` or `alpha_blend_op` is an advanced blend operation, then
    ///    `color_attachment_count` of the subpass this pipeline is compiled against must be less
    ///    than or equal to
    ///    [`VkPhysicalDeviceBlendOperationAdvancedPropertiesExt::advanced_blend_max_color_attachments`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `color_blend_op` must be a valid [`VkBlendOp`] value
    pub color_blend_op: VkBlendOp,

    /// `src_alpha_blend_factor` selects which blend factor is used to determine the source factor
    /// `Sa`.
    ///
    /// # Valid Usage
    ///  - If the `dual_src_blend` feature is not enabled, `src_alpha_blend_factor` must not be
    ///    [`VkBlendFactor::Src1Color`], [`VkBlendFactor::OneMinusSrc1Color`],
    ///    [`VkBlendFactor::Src1Alpha`], or [`VkBlendFactor::OneMinusSrc1Alpha`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `src_alpha_blend_factor` must be a valid [`VkBlendFactor`] value
    pub src_alpha_blend_factor: VkBlendFactor,

    /// `dst_alpha_blend_factor` selects which blend factor is used to determine the destination
    /// factor `Da`.
    ///
    /// # Valid Usage
    ///  - If the `dual_src_blend` feature is not enabled, `dst_alpha_blend_factor` must not be
    ///    [`VkBlendFactor::Src1Color`], [`VkBlendFactor::OneMinusSrc1Color`],
    ///    [`VkBlendFactor::Src1Alpha`], or [`VkBlendFactor::OneMinusSrc1Alpha`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `dst_alpha_blend_factor` must be a valid [`VkBlendFactor`] value
    pub dst_alpha_blend_factor: VkBlendFactor,

    /// `alpha_blend_op` selects which blend operation is used to calculate the alpha values to
    /// write to the color attachment.
    ///
    /// # Valid Usage
    ///  - If either of `color_blend_op` or `alpha_blend_op` is an advanced blend operation, then
    ///    `color_blend_op` must equal `alpha_blend_op`
    ///  - If
    ///    [`VkPhysicalDeviceBlendOperationAdvancedPropertiesExt::advanced_blend_independent_blend`]
    ///    is [`VK_FALSE`] and `alpha_blend_op` is an advanced blend operation, then
    ///    `alpha_blend_op` must be the same for all attachments
    ///  - If `color_blend_op` or `alpha_blend_op` is an advanced blend operation, then
    ///    `color_attachment_count` of the subpass this pipeline is compiled against must be less
    ///    than or equal to
    ///    [`VkPhysicalDeviceBlendOperationAdvancedPropertiesExt::advanced_blend_max_color_attachments`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `alpha_blend_op` must be a valid [`VkBlendOp`] value
    pub alpha_blend_op: VkBlendOp,

    /// `color_write_mask` is a bitmask of [`VkColorComponentFlag`]s specifying which of the R, G,
    /// B, and/or A components are enabled for writing, as described for the Color Write Mask.
    ///
    /// # Valid Usage (Implicit)
    ///  - `color_write_mask` must be a valid combination of [`VkColorComponentFlag`] values
    pub color_write_mask: VkColorComponentFlags,
}

const impl Default for VkPipelineColorBlendAttachmentState {
    fn default() -> Self {
        VkPipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: VkBlendFactor::Zero,
            dst_color_blend_factor: VkBlendFactor::Zero,
            color_blend_op: VkBlendOp::Add,
            src_alpha_blend_factor: VkBlendFactor::Zero,
            dst_alpha_blend_factor: VkBlendFactor::Zero,
            alpha_blend_op: VkBlendOp::Add,
            color_write_mask: VkColorComponentFlags::empty(),
        }
    }
}
