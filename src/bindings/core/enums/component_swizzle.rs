#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkComponentSwizzle {
    Identity = 0,
    Zero = 1,
    One = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6,
}
