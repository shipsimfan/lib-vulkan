use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkSampleCountFlagBits {
    _1Bit = 0x00000001,
    _2Bit = 0x00000002,
    _4Bit = 0x00000004,
    _8Bit = 0x00000008,
    _16Bit = 0x00000010,
    _32Bit = 0x00000020,
    _64Bit = 0x00000040,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkSampleCountFlags(VkFlags);

impl VkSampleCountFlags {
    pub const fn new(bits: &[VkSampleCountFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkSampleCountFlags(flags)
    }

    pub const fn contains(&self, bit: VkSampleCountFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkSampleCountFlags {
    fn default() -> Self {
        VkSampleCountFlags(0)
    }
}
