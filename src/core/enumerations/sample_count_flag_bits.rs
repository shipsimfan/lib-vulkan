// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Bitmask specifying sample counts supported for an image used for storage operations
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkSampleCountFlagBits {
    /// [`VkSampleCountFlagBits::_1Bit`] specifies an image with one sample per pixel.
    _1Bit = 0x00000001,

    /// [`VkSampleCountFlagBits::_2Bit`] specifies an image with 2 samples per pixel.
    _2Bit = 0x00000002,

    /// [`VkSampleCountFlagBits::_4Bit`] specifies an image with 4 samples per pixel.
    _4Bit = 0x00000004,

    /// [`VkSampleCountFlagBits::_8Bit`] specifies an image with 8 samples per pixel.
    _8Bit = 0x00000008,

    /// [`VkSampleCountFlagBits::_16Bit`] specifies an image with 16 samples per pixel.
    _16Bit = 0x00000010,

    /// [`VkSampleCountFlagBits::_32Bit`] specifies an image with 32 samples per pixel.
    _32Bit = 0x00000020,

    /// [`VkSampleCountFlagBits::_64Bit`] specifies an image with 64 samples per pixel.
    _64Bit = 0x00000040,
}
