#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkBlendOp {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}
