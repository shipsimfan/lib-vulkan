// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Image view types
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum VkImageViewType {
    _1d = 0,
    _2d = 1,
    _3d = 2,
    Cube = 3,
    _1dArray = 4,
    _2dArray = 5,
    CubeArray = 6,
}
