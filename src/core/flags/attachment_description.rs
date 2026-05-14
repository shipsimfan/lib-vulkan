use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkAttachmentDescriptionFlag`]
    ///
    /// # Description
    /// [`VkAttachmentDescriptionFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkAttachmentDescriptionFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkAttachmentDescriptionFlags;

    /// Bitmask specifying additional properties of an attachment
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkAttachmentDescriptionFlag {
        /// [`VkAttachmentDescriptionFlag::MayAlias`] specifies that the attachment aliases the
        /// same device memory as other attachments.
        MayAlias = 0x00000001,

        /// [`VkAttachmentDescriptionFlag::ResolveSkipTransferFunctionKhr`] specifies that
        /// resolve operations happening to an sRGB encoded attachment must not convert samples
        /// from nonlinear to linear before averaging.
        ///
        /// Provided by [`khr_maintenance10`]
        ResolveSkipTransferFunctionKhr = 0x00000002,

        /// [`VkAttachmentDescriptionFlag::ResolveEnableTransferFunctionKhr`] specifies that
        /// resolve operations happening to an sRGB encoded attachment must convert samples from
        /// nonlinear to linear before averaging.
        ///
        /// Provided by [`khr_maintenance10`]
        ResolveEnableTransferFunctionKhr = 0x00000004,
    }
}
