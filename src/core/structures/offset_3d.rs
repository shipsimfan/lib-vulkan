// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying a three-dimensional offset
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkOffset3D {
    /// `x` is the x offset.
    pub x: i32,

    /// `y` is the y offset.
    pub y: i32,

    /// `z` is the z offset.
    pub z: i32,
}

const impl Default for VkOffset3D {
    fn default() -> Self {
        VkOffset3D { x: 0, y: 0, z: 0 }
    }
}
