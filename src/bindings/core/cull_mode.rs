use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkCullModeFlagBits {
    None = 0,
    Front = 0x00000001,
    Back = 0x00000002,
    FrontAndBack = 0x00000003,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkCullModeFlags(VkFlags);

impl VkCullModeFlags {
    pub const fn new(bits: &[VkCullModeFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkCullModeFlags(flags)
    }

    pub const fn contains(&self, bit: VkCullModeFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkCullModeFlags {
    fn default() -> Self {
        VkCullModeFlags(VkCullModeFlagBits::Back as VkFlags)
    }
}
