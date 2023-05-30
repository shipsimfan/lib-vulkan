use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkColorComponentFlagBits {
    R = 0x00000001,
    G = 0x00000002,
    B = 0x00000004,
    A = 0x00000008,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkColorComponentFlags(VkFlags);

impl VkColorComponentFlags {
    pub const fn new(bits: &[VkColorComponentFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkColorComponentFlags(flags)
    }

    pub const fn contains(&self, bit: VkColorComponentFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkColorComponentFlags {
    fn default() -> Self {
        VkColorComponentFlags(
            VkColorComponentFlagBits::R as VkFlags
                | VkColorComponentFlagBits::G as VkFlags
                | VkColorComponentFlagBits::B as VkFlags
                | VkColorComponentFlagBits::A as VkFlags,
        )
    }
}
