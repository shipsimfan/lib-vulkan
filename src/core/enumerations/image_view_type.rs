// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Image view types
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkImageViewType {
    #[allow(missing_docs)]
    _1d = 0,

    #[allow(missing_docs)]
    _2d = 1,

    #[allow(missing_docs)]
    _3d = 2,

    #[allow(missing_docs)]
    Cube = 3,

    #[allow(missing_docs)]
    _1dArray = 4,

    #[allow(missing_docs)]
    _2dArray = 5,

    #[allow(missing_docs)]
    CubeArray = 6,
}
