#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkImageViewType {
    _1D = 0,
    _2D = 1,
    _3D = 2,
    Cube = 3,
    _1DArray = 4,
    _2DArray = 5,
    CubeArray = 6,
}
