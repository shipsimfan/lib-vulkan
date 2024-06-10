use crate::{VkExtent3D, VkQueueFlags};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure providing information about a queue family
///
/// The value returned in `min_image_transfer_granularity` has a unit of compressed texel blocks
/// for images having a block-compressed format, and a unit of texels otherwise.
///
/// Possible values of `min_image_transfer_granularity` are:
///  - `(0,0,0)` specifies that only whole mip levels must be transferred using the image transfer
///    operations on the corresponding queues. In this case, the following restrictions apply to
///    all offset and extent parameters of image transfer operations:
///    - The `x`, `y`, and `z` members of a [`VkOffset3D`] parameter must always be zero.
///    - The `width`, `height`, and `depth` members of a [`VkExtent3D`] parameter must always match
///      the `width`, `height`, and `depth` of the image subresource corresponding to the
///      parameter, respectively.
///  - `(ax, ay, az)` where `ax`, `ay`, and `az` are all integer powers of two. In this case the
///    following restrictions apply to all image transfer operations:
///    - `x`, `y`, and `z` of a [`VkOffset3D`] parameter must be integer multiples of `ax`, `ay`,
///      and `az`, respectively.
///    - `width` of a [`VkExtent3D`] parameter must be an integer multiple of `ax`, or else
///      `x + width` must equal the width of the image subresource corresponding to the parameter.
///    - `height` of a [`VkExtent3D`] parameter must be an integer multiple of `ay`, or else
///      `y + height` must equal the height of the image subresource corresponding to the
///      parameter.
///    - `depth` of a [`VkExtent3D`] parameter must be an integer multiple of `az`, or else
///      `z + depth` must equal the depth of the image subresource corresponding to the parameter.
///    - If the format of the image corresponding to the parameters is one of the block-compressed
///      formats then for the purposes of the above calculations the granularity must be scaled up
///      by the compressed texel block dimensions.
///
/// Queues supporting graphics and/or compute operations must report `(1,1,1)` in
/// `min_image_transfer_granularity`, meaning that there are no additional restrictions on the
/// granularity of image transfer operations for these queues. Other queues supporting image
/// transfer operations are only required to support whole mip level transfers, thus
/// `min_image_transfer_granularity` for queues belonging to such queue families may be `(0,0,0)`.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkQueueFamilyProperties {
    /// `queue_flags` is a bitmask of VkQueueFlagBits indicating capabilities of the queues in this
    /// queue family.
    pub queue_flags: VkQueueFlags,

    /// `queue_count` is the unsigned integer count of queues in this queue family. Each queue
    /// family must support at least one queue.
    pub queue_count: u32,

    /// `timestamp_valid_bits` is the unsigned integer count of meaningful bits in the timestamps
    /// written via [`VkCmdWriteTimestamp2`] or [`VkCmdWriteTimestamp`]. The valid range for the
    /// count is 36 to 64 bits, or a value of 0, indicating no support for timestamps. Bits outside
    /// the valid range are guaranteed to be zeros.
    pub timestamp_valid_bits: u32,

    /// `min_image_transfer_granularity` is the minimum granularity supported for image transfer
    /// operations on the queues in this queue family.
    pub min_image_transfer_granularity: VkExtent3D,
}

impl Default for VkQueueFamilyProperties {
    fn default() -> Self {
        VkQueueFamilyProperties {
            queue_flags: 0,
            queue_count: 0,
            timestamp_valid_bits: 0,
            min_image_transfer_granularity: VkExtent3D::default(),
        }
    }
}
