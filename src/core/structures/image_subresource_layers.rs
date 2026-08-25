use crate::VkImageAspectFlags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_REMAINING_ARRAY_LAYERS, VK_VERSION_1_0, VkImageAspectFlag};

/// Structure specifying an image subresource layers
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageSubresourceLayers {
    /// `aspect_mask` is a combination of [`VkImageAspectFlag`], selecting the color, depth and/or
    /// stencil aspects to be copied.
    ///
    /// # Valid Usage
    ///  - If `aspect_mask` contains [`VkImageAspectFlag::Color`], it must not contain either of
    ///    [`VkImageAspectFlag::Depth`] or [`VkImageAspectFlag::Stencil`]
    ///  - `aspect_mask` must not contain [`VkImageAspectFlag::Metadata`]
    ///  - `aspect_mask` must not include `VkImageAspectFlag::MemoryPlaneIExt`] for any index `I`
    ///
    /// # Valid Usage (Implicit)
    ///  - `aspect_mask` must be a valid combination of [`VkImageAspectFlag`] values
    ///  - `aspect_mask` must not be 0
    pub aspect_mask: VkImageAspectFlags,

    /// `mip_level` is the mipmap level to copy
    pub mip_level: u32,

    /// `base_array_layer` is the starting layer to copy.
    pub base_array_layer: u32,

    /// `layer_count` is the number of layers to copy.
    ///
    /// # Valid Usage
    ///  - If the `maintenance5` feature is not enabled, `layer_count` must not be
    ///    [`VK_REMAINING_ARRAY_LAYERS`]
    ///  - If `layer_count` is not [`VK_REMAINING_ARRAY_LAYERS`], it must be greater than 0
    pub layer_count: u32,
}

const impl Default for VkImageSubresourceLayers {
    fn default() -> Self {
        VkImageSubresourceLayers {
            aspect_mask: VkImageAspectFlags::default(),
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 0,
        }
    }
}
