use std::{ffi::c_void, ptr::NonNull};

use crate::bindings::VkStructureType;

#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    flags: u32,
    h_instance: win32::HInstance,
    h_wnd: win32::HWnd,
}

impl VkWin32SurfaceCreateInfoKHR {
    pub fn new(h_instance: win32::HInstance, h_wnd: win32::HWnd) -> Self {
        VkWin32SurfaceCreateInfoKHR {
            s_type: VkStructureType::Win32SurfaceCreateInfo,
            p_next: None,
            flags: 0,
            h_instance,
            h_wnd,
        }
    }
}
