use crate::{VkAllocationCallbacks, VkDevice, VkInstance, VkPhysicalDevice, VkResult};
use std::ffi::c_char;

pub(crate) type VkDestroyInstance =
    extern "system" fn(instance: VkInstance, p_allocator: *const VkAllocationCallbacks);

pub(crate) type VkEnumeratePhysicalDevice = extern "system" fn(
    instance: VkInstance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut VkPhysicalDevice,
) -> VkResult;

pub(crate) type VkGetDeviceProcAddr =
    extern "system" fn(device: VkDevice, p_name: *const c_char) -> Option<extern "system" fn()>;
