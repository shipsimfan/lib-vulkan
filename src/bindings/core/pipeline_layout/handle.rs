use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkPipelineLayoutT;
}

pub(crate) type VkPipelineLayout = NonNull<VkPipelineLayoutT>;
