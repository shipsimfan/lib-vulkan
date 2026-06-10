use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkSampleCountFlag`]
    ///
    /// # Description
    /// [`VkSampleCountFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkSampleCountFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkSampleCountFlags;

    /// Bitmask specifying sample counts supported for an image used for storage operations
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkSampleCountFlag {
        /// [`VkSampleCountFlag::_1`] specifies an image with one sample per pixel.
        _1 = 0x00000001,

        /// [`VkSampleCountFlag::_2`] specifies an image with 2 samples per pixel.
        _2 = 0x00000002,

        /// [`VkSampleCountFlag::_4`] specifies an image with 4 samples per pixel.
        _4 = 0x00000004,

        /// [`VkSampleCountFlag::_8`] specifies an image with 8 samples per pixel.
        _8 = 0x00000008,

        /// [`VkSampleCountFlag::_16`] specifies an image with 16 samples per pixel.
        _16 = 0x00000010,

        /// [`VkSampleCountFlag::_32`] specifies an image with 32 samples per pixel.
        _32 = 0x00000020,

        /// [`VkSampleCountFlag::_64`] specifies an image with 64 samples per pixel.
        _64 = 0x00000040,
    }
}
