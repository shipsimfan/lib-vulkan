use crate::bindings::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkDeviceQueueCreateFlagBits {
    Protected = 0x00000001,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkDeviceQueueCreateFlags(VkFlags);

impl VkDeviceQueueCreateFlags {
    pub const fn new(bits: &[VkDeviceQueueCreateFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkDeviceQueueCreateFlags(flags)
    }

    pub const fn contains(&self, bit: VkDeviceQueueCreateFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}
