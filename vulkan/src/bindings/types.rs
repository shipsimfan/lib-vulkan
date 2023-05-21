use std::{ffi::c_void, ptr::NonNull};

pub type VkFlags = u32;
pub type VkInstance = NonNull<c_void>;
