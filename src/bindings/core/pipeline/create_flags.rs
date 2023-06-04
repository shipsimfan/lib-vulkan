use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkPipelineCreateFlagBits {
    DisableOptimizations = 0x00000001,
    AllowDerivatives = 0x00000002,
    Derivative = 0x00000004,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineCreateFlags(VkFlags);

impl VkPipelineCreateFlags {
    pub const fn new(bits: &[VkPipelineCreateFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkPipelineCreateFlags(flags)
    }

    pub const fn contains(&self, bit: VkPipelineCreateFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkPipelineCreateFlags {
    fn default() -> Self {
        VkPipelineCreateFlags(0)
    }
}
