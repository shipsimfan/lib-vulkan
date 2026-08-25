use crate::{VkImageSubresourceLayers, VkOffset3D};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_REMAINING_ARRAY_LAYERS, VK_VERSION_1_0, VkImageCreateInfo};

/// Structure specifying an image blit operation
///
/// # Description
/// For each element of the `regions` array, a blit operation is performed for the specified source
/// and destination regions.
///
/// # Valid Usage
///  - The `aspect_mask` member of `src_subresource` and `dst_subresource` must match
///  - If neither of the `layer_count` members of `src_subresource` or `dst_subresource` are
///    [`VK_REMAINING_ARRAY_LAYERS`], the `layer_count` members of `src_subresource` or
///    `dst_subresource` must match
///  - If one of the `layer_count` members of `src_subresource` or `dst_subresource` is
///    [`VK_REMAINING_ARRAY_LAYERS`], the other member must be either [`VK_REMAINING_ARRAY_LAYERS`]
///    or equal to the `array_layers` member of the [`VkImageCreateInfo`] used to create the image
///    minus `base_array_layer`
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageBlit {
    /// `src_subresource` is the subresource to blit from.
    ///
    /// # Valid Usage (Implicit)
    ///  - `src_subresource` must be a valid [`VkImageSubresourceLayers`] structure
    pub src_subresource: VkImageSubresourceLayers,

    /// `src_offsets` is a pointer to an array of two [`VkOffset3D`] structures specifying the
    /// bounds of the source region within `src_subresource`.
    pub src_offsets: [VkOffset3D; 2],

    /// `dst_subresource` is the subresource to blit into.
    ///
    /// # Valid Usage (Implicit)
    ///  - `dst_subresource` must be a valid [`VkImageSubresourceLayers`] structure
    pub dst_subresource: VkImageSubresourceLayers,

    /// `dst_offsets` is a pointer to an array of two [`VkOffset3D`] structures specifying the
    /// bounds of the destination region within `dst_subresource`.
    pub dst_offsets: [VkOffset3D; 2],
}
