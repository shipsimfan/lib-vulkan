use crate::bindings::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkCompositeAlphaFlagBitsKHR {
    Opaque = 0x00000001,
    PreMultiplied = 0x00000002,
    PostMultiplied = 0x00000004,
    Inherit = 0x00000008,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkCompositeAlphaFlagsKHR(VkFlags);

impl VkCompositeAlphaFlagsKHR {
    pub const fn new(bits: &[VkCompositeAlphaFlagBitsKHR]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkCompositeAlphaFlagsKHR(flags)
    }

    pub const fn contains(&self, bit: VkCompositeAlphaFlagBitsKHR) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}
