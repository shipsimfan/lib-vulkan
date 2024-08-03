// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Buffer and image sharing modes
///
/// Ranges of buffers and image subresources of image objects created using
/// [`VkSharingMode::Exclusive`] must only be accessed by queues in the queue family that has
/// ownership of the resource. Upon creation, such resources are not owned by any queue family;
/// ownership is implicitly acquired upon first use within a queue. Once a resource using
/// [`VkSharingMode::Exclusive`] is owned by some queue family, the application must perform a
/// queue family ownership transfer to make the memory contents of a range or image subresource
/// accessible to a different queue family.
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
#[allow(missing_docs)]
pub enum VkSharingMode {
    /// [`VkSharingMode::Exclusive`] specifies that access to any range or image subresource of the
    /// object will be exclusive to a single queue family at a time.
    Exclusive = 0,

    /// [`VkSharingMode::Concurrent`] specifies that concurrent access to any range or image
    /// subresource of the object from multiple queue families is supported.
    Concurrent = 1,
}
