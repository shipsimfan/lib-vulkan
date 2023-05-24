use std::ptr::NonNull;

extern "system" {
    pub type VkInstanceT;
}

pub type VkInstance = NonNull<VkInstanceT>;
