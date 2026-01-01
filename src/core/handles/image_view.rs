use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to an image view object
    ///
    /// Image objects are not directly accessed by pipeline shaders for reading or writing image
    /// data. Instead, image views representing contiguous ranges of the image subresources and
    /// containing additional metadata are used for that purpose. Views must be created on images
    /// of compatible types, and must represent a valid subset of image subresources.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkImageView
);
