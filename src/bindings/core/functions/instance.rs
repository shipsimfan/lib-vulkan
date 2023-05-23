use crate::{
    bindings::{VkInstance, VkPhysicalDevice},
    VkResult,
};
use std::{ffi::c_void, ptr::NonNull};

pub type VkDestroyInstance =
    extern "system" fn(instance: VkInstance, p_allocator: Option<NonNull<c_void>>);
pub type VkEnumeratePhysicalDevices = extern "system" fn(
    instance: VkInstance,
    p_physical_device_count: &mut u32,
    p_physical_devices: Option<NonNull<VkPhysicalDevice>>,
) -> VkResult;
