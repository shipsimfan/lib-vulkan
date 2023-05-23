use std::ffi::CStr;

pub const VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME: &CStr =
    unsafe { std::mem::transmute("VK_KHR_portability_enumeration\0") };
pub const VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION: u32 = 1;
