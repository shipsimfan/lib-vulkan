use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkPipelineInputAssemblyStateCreateFlags(VkFlags);

impl Default for VkPipelineInputAssemblyStateCreateFlags {
    fn default() -> Self {
        VkPipelineInputAssemblyStateCreateFlags(0)
    }
}
