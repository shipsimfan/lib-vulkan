use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkImageT;
}

pub(crate) type VkImage = NonNull<VkImageT>;
