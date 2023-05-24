use crate::bindings::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkImageViewCreateFlagBits {
    FragmentDensityMapDynamic = 0x00000001,
    FragmentDensityMapDeferred = 0x00000002,
    DescriptorBufferCaptureReplay = 0x00000004,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkImageViewCreateFlags(VkFlags);

impl VkImageViewCreateFlags {
    pub const fn new(bits: &[VkImageViewCreateFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkImageViewCreateFlags(flags)
    }

    pub const fn contains(&self, bit: VkImageViewCreateFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}
