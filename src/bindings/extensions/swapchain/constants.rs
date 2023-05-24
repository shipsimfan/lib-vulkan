use std::ffi::CStr;

pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &CStr =
    unsafe { std::mem::transmute("VK_KHR_swapchain\0") };
pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
