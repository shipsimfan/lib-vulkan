use std::ptr::NonNull;

extern "system" {
    pub(crate) type VkImageViewT;
}

pub(crate) type VkImageView = NonNull<VkImageViewT>;
