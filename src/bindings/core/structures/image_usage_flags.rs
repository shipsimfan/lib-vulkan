use crate::bindings::VkFlags;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkImageUsageFlagBits {
    TransferSrc = 0x00000001,
    TransferDst = 0x00000002,
    Sampled = 0x00000004,
    Storage = 0x00000008,
    ColorAttachment = 0x00000010,
    DepthStencilAttachment = 0x00000020,
    TransientAttachment = 0x00000040,
    InputAttachment = 0x00000080,
    FragmentShadingRateAttachment = 0x00000100,
    FragmentDensityMap = 0x00000200,
    VideoDecodeDst = 0x00000400,
    VideoDecodeSrc = 0x00000800,
    VideoDecodeDPB = 0x00001000,
    VideoEncodeDst = 0x00002000,
    VideoEncodeSrc = 0x00004000,
    VideoEncodeDPB = 0x00008000,
    InvocationMask = 0x00040000,
    AttachmentFeedbackLoop = 0x00080000,
    SampleWeight = 0x00100000,
    SampleBlockMatch = 0x00200000,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkImageUsageFlags(VkFlags);

impl VkImageUsageFlags {
    pub const fn new(bits: &[VkImageUsageFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkImageUsageFlags(flags)
    }

    pub const fn contains(&self, bit: VkImageUsageFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}
