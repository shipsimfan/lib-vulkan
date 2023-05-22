use crate::bindings::{VkDevice, VkQueue};
use std::{ffi::c_void, ptr::NonNull};

pub type VkGetDeviceProcAddr =
    extern "system" fn(device: VkDevice, p_namce: NonNull<u8>) -> Option<extern "system" fn()>;

pub type VkDestroyDevice =
    extern "system" fn(device: VkDevice, p_allocator: Option<NonNull<c_void>>);

pub type VkGetDeviceQueue = extern "system" fn(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: NonNull<Option<VkQueue>>,
);
