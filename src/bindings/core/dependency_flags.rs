use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkDependencyFlagBits {
    ByRegion = 0x00000001,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkDependencyFlags(VkFlags);

impl VkDependencyFlags {
    pub const fn new(bits: &[VkDependencyFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkDependencyFlags(flags)
    }

    pub const fn contains(&self, bit: VkDependencyFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkDependencyFlags {
    fn default() -> Self {
        VkDependencyFlags(0)
    }
}
