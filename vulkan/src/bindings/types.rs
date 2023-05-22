use std::{ffi::c_void, ptr::NonNull};

pub type VkBool32 = u32;
pub type VkDevice = NonNull<c_void>;
pub type VkDeviceSize = u64;
pub type VkFlags = u32;
pub type VkInstance = NonNull<c_void>;
pub type VkPhysicalDevice = NonNull<c_void>;
