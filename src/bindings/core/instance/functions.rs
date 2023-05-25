use crate::{bindings::VkPhysicalDevice, VkAllocationCallbacks, VkInstance, VkResult};

pub(crate) type VkDestroyInstance =
    extern "system" fn(instance: VkInstance, p_allocator: *const VkAllocationCallbacks);

pub(crate) type VkEnumeratePhysicalDevice = extern "system" fn(
    instance: VkInstance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut VkPhysicalDevice,
) -> VkResult;
