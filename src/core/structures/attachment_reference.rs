use crate::VkImageLayout;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying an attachment reference
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkAttachmentReference {
    /// `attachment` is either an integer value identifying an attachment at the corresponding
    /// index in [`VkRenderPassCreateInfo::attachments`], or [`VK_ATTACHMENT_UNUSED`] to signify
    /// that this attachment is not used.
    pub attachment: u32,

    /// `layout` is a [`VkImageLayout`] value specifying the layout the attachment uses during the
    /// subpass.
    ///
    /// # Valid Usage
    ///  - If `attachment` is not [`VK_ATTACHMENT_UNUSED`], layout must not be
    ///    [`VkImageLayout::Undefined`], [`VkImageLayout::ZeroInitializedExt`],
    ///    [`VkImageLayout::Preinitialized`], or [`VkImageLayout::PresentSrcKhr`]
    ///  - If the `separate_depth_stencil_layouts` feature is not enabled, and attachment is not
    ///    [`VK_ATTACHMENT_UNUSED`], layout must not be [`VkImageLayout::DepthAttachmentOptimal`],
    ///    [`VkImageLayout::DepthReadOnlyOptimal`], [`VkImageLayout::StencilAttachmentOptimal`], or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`],
    ///  - If the `synchronization2` feature is not enabled, layout must not be
    ///    [`VkImageLayout::AttachmentOptimal`] or [`VkImageLayout::ReadOnlyOptimal`]
    ///  - If the `attachment_feedback_loop_layout` feature is not enabled, layout must not be
    ///    [`VkImageLayout::AttachmentFeedbackLoopOptimalExt`]
    ///  - If the `dynamic_rendering_local_read` feature is not enabled, layout must not be
    ///    [`VkImageLayout::RenderingLocalRead`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `layout` must be a valid [`VkImageLayout`] value
    pub layout: VkImageLayout,
}

impl const Default for VkAttachmentReference {
    fn default() -> Self {
        VkAttachmentReference {
            attachment: 0,
            layout: VkImageLayout::Undefined,
        }
    }
}
