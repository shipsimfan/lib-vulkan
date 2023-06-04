#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkPipelineBindPoint {
    Graphics = 0,
    Compute = 1,
}
