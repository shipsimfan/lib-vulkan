use crate::bindings::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkQueueFlagBits {
    Graphics = 0x00000001,
    Compute = 0x00000002,
    Transfer = 0x00000004,
    SparseBinding = 0x00000008,
    Protected = 0x00000010,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkQueueFlags(VkFlags);

impl VkQueueFlags {
    pub const fn new(bits: &[VkQueueFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkQueueFlags(flags)
    }

    pub const fn contains(&self, bit: VkQueueFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}
