use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkAccessFlagBits {
    IndirectCommandRead = 0x00000001,
    IndexRead = 0x00000002,
    VertexAttributeRead = 0x00000004,
    UniformRead = 0x00000008,
    InputAttachmentRead = 0x00000010,
    ShaderRead = 0x00000020,
    ShaderWrite = 0x00000040,
    ColorAttachmentRead = 0x00000080,
    ColorAttachmentWrite = 0x00000100,
    DepthStencilAttachmentRead = 0x00000200,
    DepthStencilAttachmentWrite = 0x00000400,
    TransferRead = 0x00000800,
    TransferWrite = 0x00001000,
    HostRead = 0x00002000,
    HostWrite = 0x00004000,
    MemoryRead = 0x00008000,
    MemoryWrite = 0x00010000,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkAccessFlags(VkFlags);

impl VkAccessFlags {
    pub const fn new(bits: &[VkAccessFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkAccessFlags(flags)
    }

    pub const fn contains(&self, bit: VkAccessFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkAccessFlags {
    fn default() -> Self {
        VkAccessFlags(0)
    }
}
