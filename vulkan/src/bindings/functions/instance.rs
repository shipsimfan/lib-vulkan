use crate::bindings::VkInstance;
use std::{ffi::c_void, ptr::NonNull};

pub type VkDestroyInstance =
    extern "system" fn(instance: VkInstance, p_allocator: Option<NonNull<c_void>>);
