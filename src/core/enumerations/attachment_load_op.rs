// rustdoc
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_4};

/// Specify how contents of an attachment are treated at the beginning of the subpass where it is
/// first used
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkAttachmentLoadOp {
    /// [`VkAttachmentLoadOp::Load`] specifies that the previous contents of the image within the
    /// render area will be preserved as the initial values. For attachments with a depth/stencil
    /// format, this uses the access type [`VkAccessFlag::DepthStencilAttachmentRead`]. For
    /// attachments with a color format, this uses the access type
    /// [`VkAccessFlag::ColorAttachmentRead`].
    Load = 0,

    /// [`VkAttachmentLoadOp::Clear`] specifies that the contents within the render area will be
    /// cleared to a uniform value, which is specified when a render pass instance is begun. For
    /// attachments with a depth/stencil format, this uses the access type
    /// [`VkAccessFlag::DepthStencilAttachmentWrite`]. For attachments with a color format, this
    /// uses the access type [`VkAccessFlag::ColorAttachmentWrite`].
    Clear = 1,

    /// [`VkAttachmentLoadOp::DontCare`] specifies that the previous contents within the area need
    /// not be preserved; the contents of the attachment will be undefined inside the render area.
    /// For attachments with a depth/stencil format, this uses the access type
    /// [`VkAccessFlag::DepthStencilAttachmentWrite`]. For attachments with a color format, this
    /// uses the access type [`VkAccessFlag::ColorAttachmentWrite`].
    DontCare = 2,

    /// [`VkAttachmentLoadOp::None`] specifies that the previous contents of the image will be
    /// undefined inside the render pass. No access type is used as the image is not accessed.
    ///
    /// Provided by [`VK_VERSION_1_4`]
    None = 1000400000,
}
