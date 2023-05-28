use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkShaderModuleT;
}

pub(crate) type VkShaderModule = NonNull<VkShaderModuleT>;
