use crate::{VkStructureType, VkWin32SurfaceCreateFlagsKHR, HINSTANCE, HWND};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_win32_surface;

/// Structure specifying parameters of a newly created Win32 surface object
///
/// Provided by [`khr_win32_surface`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkWin32SurfaceCreateInfoKHR {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    pub flags: VkWin32SurfaceCreateFlagsKHR,

    /// `hinstance` is the Win32 [`HINSTANCE`] for the window to associate the surface with.
    pub hinstance: HINSTANCE,

    /// `hwnd` is the Win32 [`HWND`] for the window to associate the surface with.
    pub hwnd: HWND,
}

impl Default for VkWin32SurfaceCreateInfoKHR {
    fn default() -> Self {
        VkWin32SurfaceCreateInfoKHR {
            r#type: VkStructureType::Win32SurfaceCreateInfoKHR,
            next: null(),
            flags: 0,
            hinstance: null_mut(),
            hwnd: null_mut(),
        }
    }
}
