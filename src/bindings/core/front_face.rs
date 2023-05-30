#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkFrontFace {
    CounterClockwise = 0,
    Clockwise = 1,
}

impl Default for VkFrontFace {
    fn default() -> Self {
        VkFrontFace::Clockwise
    }
}
