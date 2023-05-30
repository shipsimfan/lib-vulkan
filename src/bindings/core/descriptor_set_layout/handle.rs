use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkDescriptorSetLayoutT;
}

pub(crate) type VkDescriptorSetLayout = NonNull<VkDescriptorSetLayoutT>;
