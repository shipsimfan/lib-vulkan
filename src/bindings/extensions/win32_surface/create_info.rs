use crate::{VkStructureType, VkWin32SurfaceCreateFlagsKHR};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkWin32SurfaceCreateInfoKHR {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkWin32SurfaceCreateFlagsKHR,
    pub(crate) h_instance: win32::HInstance,
    pub(crate) h_wnd: win32::HWnd,
}
