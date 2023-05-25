#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkPhysicalDeviceType {
    Other = 0,
    IntegratedGPU = 1,
    DiscreteGPU = 2,
    VirtualGPU = 3,
    CPU = 4,
}

impl Default for VkPhysicalDeviceType {
    fn default() -> Self {
        VkPhysicalDeviceType::Other
    }
}
