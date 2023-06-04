use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkAttachmentDescriptionFlagBits {
    MayAlias = 0x00000001,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkAttachmentDescriptionFlags(VkFlags);

impl VkAttachmentDescriptionFlags {
    pub const fn new(bits: &[VkAttachmentDescriptionFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkAttachmentDescriptionFlags(flags)
    }

    pub const fn contains(&self, bit: VkAttachmentDescriptionFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkAttachmentDescriptionFlags {
    fn default() -> Self {
        VkAttachmentDescriptionFlags(0)
    }
}
