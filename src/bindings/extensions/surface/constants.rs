use std::ffi::CStr;

pub const VK_KHR_SURFACE_EXTENSION_NAME: &CStr = unsafe { std::mem::transmute("VK_KHR_surface\0") };
pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
