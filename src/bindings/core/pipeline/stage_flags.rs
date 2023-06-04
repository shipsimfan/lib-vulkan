use crate::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkPipelineStageFlagBits {
    TopOfPipe = 0x00000001,
    DrawIndirect = 0x00000002,
    VertexInput = 0x00000004,
    VertexShader = 0x00000008,
    TessellationControlShader = 0x00000010,
    TessellationEvaluationShader = 0x00000020,
    GeometryShader = 0x00000040,
    FragmentShader = 0x00000080,
    EarlyFragmentTests = 0x00000100,
    LateFragmentTests = 0x00000200,
    ColorAttachmentOutput = 0x00000400,
    ComputeShader = 0x00000800,
    Transfer = 0x00001000,
    BottomOfPipe = 0x00002000,
    Host = 0x00004000,
    AllGraphics = 0x00008000,
    AllCommands = 0x00010000,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineStageFlags(VkFlags);

impl VkPipelineStageFlags {
    pub const fn new(bits: &[VkPipelineStageFlagBits]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkPipelineStageFlags(flags)
    }

    pub const fn contains(&self, bit: VkPipelineStageFlagBits) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}

impl Default for VkPipelineStageFlags {
    fn default() -> Self {
        VkPipelineStageFlags(0)
    }
}
