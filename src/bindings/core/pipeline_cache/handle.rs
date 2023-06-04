use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkPipelineCacheT;
}

pub(crate) type VkPipelineCache = NonNull<VkPipelineCacheT>;
