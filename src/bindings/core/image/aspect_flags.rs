use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkImageAspectFlagBits {
    Color = 0x00000001,
    Depth = 0x00000002,
    Stencil = 0x00000004,
    Metadata = 0x00000010,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkImageAspectFlags(VkFlags);

impl VkImageAspectFlags {
    pub const fn new(bits: &[VkImageAspectFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkImageAspectFlags(flags)
    }

    pub const fn contains(&self, bit: VkImageAspectFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkImageAspectFlags {
    fn default() -> Self {
        VkImageAspectFlags(0)
    }
}
