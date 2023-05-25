use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkInstanceCreateFlagBits {
    None = 0,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkInstanceCreateFlags(VkFlags);

impl VkInstanceCreateFlags {
    pub const fn new(bits: &[VkInstanceCreateFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkInstanceCreateFlags(flags)
    }

    pub const fn contains(&self, bit: VkInstanceCreateFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkInstanceCreateFlags {
    fn default() -> Self {
        VkInstanceCreateFlags(0)
    }
}
