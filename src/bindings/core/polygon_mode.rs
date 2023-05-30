#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkPolygonMode {
    Fill = 0,
    Line = 1,
    Point = 2,
}

impl Default for VkPolygonMode {
    fn default() -> Self {
        VkPolygonMode::Fill
    }
}
