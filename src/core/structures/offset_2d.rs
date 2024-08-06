/// Structure specifying a two-dimensional offset
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkOffset2D {
    /// `x` is the x offset.
    pub x: i32,

    /// `y` is the y offset.
    pub y: i32,
}

impl Default for VkOffset2D {
    fn default() -> Self {
        VkOffset2D { x: 0, y: 0 }
    }
}
