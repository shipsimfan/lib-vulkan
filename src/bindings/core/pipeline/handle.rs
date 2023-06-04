use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkPipelineT;
}

pub(crate) type VkPipeline = NonNull<VkPipelineT>;
