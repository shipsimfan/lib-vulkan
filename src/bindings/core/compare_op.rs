#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkCompareOp {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessOrEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterOrEqual = 6,
    Always = 7,
}
