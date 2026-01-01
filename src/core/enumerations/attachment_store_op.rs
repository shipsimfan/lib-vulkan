// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_3};

/// Specify how contents of an attachment are treated at the end of the subpass where it is last
/// used
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkAttachmentStoreOp {
    /// [`VkAttachmentStoreOp::Store`] specifies the contents generated during the render pass and
    /// within the render area are written to memory. For attachments with a depth/stencil format,
    /// this uses the access type [`VkAccessFlag::DepthStencilAttachmentWriteBit`]. For attachments
    /// with a color format, this uses the access type [`VkAccessFlag::ColorAttachmentWriteBit`].
    Store = 0,

    /// [`VkAttachmentStoreOp::DontCare`] specifies the contents within the render area are not
    /// needed after rendering, and may be discarded; the contents of the attachment will be
    /// undefined inside the render area. For attachments with a depth/stencil format, this uses
    /// the access type [`VkAccessFlag::DepthStencilAttachmentWriteBit`]. For attachments with a
    /// color format, this uses the access type [`VkAccessFlag::ColorAttachmentWriteBit`].
    DontCare = 1,

    /// [`VkAttachmentStoreOp::None`] specifies the contents within the render area are not
    /// accessed by the store operation as long as no values are written to the attachment during
    /// the render pass. If values are written during the render pass, this behaves identically to
    /// [`VkAttachmentStoreOp::DontCare`] and with matching access semantics.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    None = 1000301000,
}
