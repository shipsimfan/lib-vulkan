// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Specifies the type of an image object
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum VkImageType {
    /// [`VkImageType::_1d`] specifies a one-dimensional image.
    _1d = 0,

    /// [`VkImageType::_2d`] specifies a two-dimensional image.
    _2d = 1,

    /// [`VkImageType::_3d`] specifies a three-dimensional image.
    _3d = 2,
}
