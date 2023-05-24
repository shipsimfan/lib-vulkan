use crate::bindings::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkSwapchainCreateFlagBitsKHR {
    SplitInstanceBindRegions = 0x00000001,
    Protected = 0x00000002,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkSwapchainCreateFlagsKHR(VkFlags);

impl VkSwapchainCreateFlagsKHR {
    pub const fn new(bits: &[VkSwapchainCreateFlagBitsKHR]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkSwapchainCreateFlagsKHR(flags)
    }

    pub const fn contains(&self, bit: VkSwapchainCreateFlagBitsKHR) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}
