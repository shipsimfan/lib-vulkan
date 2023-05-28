use crate::VkFlags;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct VkShaderModuleCreateFlags(VkFlags);

impl Default for VkShaderModuleCreateFlags {
    fn default() -> Self {
        VkShaderModuleCreateFlags(0)
    }
}
