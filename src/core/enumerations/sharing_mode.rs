// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkImageUsageFlag};

/// Buffer and image sharing modes
///
/// # Description
/// Ranges of buffers and image subresources of image objects created using
/// [`VkSharingMode::Exclusive`] must only be accessed by queues in the queue family that has
/// ownership of the resource. Upon creation, such resources are not owned by any queue family;
/// ownership is implicitly acquired upon first use within a queue. Once a resource using
/// [`VkSharingMode::Exclusive`] is owned by some queue family, the application must perform a
/// queue family ownership transfer to make the memory contents of a range or image subresource
/// accessible to a different queue family.
///
/// If the `maintenance9` feature is enabled, the contents of buffer resources, and of linear image
/// resources (i.e., those created with tiling set to [`VK_IMAGE_TILING_LINEAR`]) are always
/// preserved when they are implicitly acquired by a different queue family on the same logical
/// device (i.e., neither queue family is [`VK_QUEUE_FAMILY_FOREIGN_EXT`] or
/// [`VK_QUEUE_FAMILY_EXTERNAL`]). This means that whenever the `maintenance9` feature is enabled,
/// explicit queue family ownership transfers of buffer and linear image resources between
/// different queue families on the same logical device are optional.
///
/// Additionally, if the maintenance9 feature is enabled, the contents of some optimal image
/// resources (i.e., those created with [`VK_IMAGE_TILING_OPTIMAL`]) are always preserved when they
/// are implicitly acquired by a different queue family on the same logical device (i.e., neither
/// queue family is [`VK_QUEUE_FAMILY_FOREIGN_EXT`] or [`VK_QUEUE_FAMILY_EXTERNAL`]). This applies
/// only to optimal images that are being implicitly acquired by a queue family whose index bit is
/// set in the current queue family’s
/// [`VkQueueFamilyOwnershipTransferPropertiesKhr::optimal_image_transfer_to_queue_families`], and
/// that were created without any of the following bits set in usage:
///  - [`VkImageUsageFlag::ColorAttachmentBit`]
///  - [`VkImageUsageFlag::DepthStencilAttachmentBit`]
///  - [`VkImageUsageFlag::TransientAttachmentBit`]
///  - [`VkImageUsageFlag::InputAttachmentBit`]
///  - [`VkImageUsageFlag::AttachmentFeedbackLoopBitExt`]
///  - [`VkImageUsageFlag::FragmentShadingRateAttachmentBitKhr`]
///
/// This means that whenever the maintenance9 feature is enabled, explicit queue family ownership
/// transfers of such image resources between such combinations of queue families are optional. For
/// all other optimal images and/or combinations of queue families, the application must still
/// perform an explicit queue family ownership transfer if it wishes to make the memory contents of
/// an optimal image subresource already owned by a queue family accessible to a different queue
/// family.
///
/// A queue family can take ownership of an image subresource or buffer range of a resource created
/// with [`VkSharingMode::Exclusive`], without an ownership transfer, in the same way as for a
/// resource that was just created; however, taking ownership in this way has the effect that the
/// contents of the image subresource or buffer range are undefined.
///
/// Ranges of buffers and image subresources of image objects created using
/// [`VkSharingMode::Concurrent`] must only be accessed by queues from the queue families specified
/// through the `queue_family_index_count` and `queue_family_indices` members of the corresponding
/// create info structures.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkSharingMode {
    /// [`VkSharingMode::Exclusive`] specifies that access to any range or image subresource of the
    /// object will be exclusive to a single queue family at a time.
    Exclusive = 0,

    /// [`VkSharingMode::Concurrent`] specifies that concurrent access to any range or image
    /// subresource of the object from multiple queue families is supported.
    Concurrent = 1,
}
