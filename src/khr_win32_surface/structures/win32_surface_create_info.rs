use crate::{VkStructureType, khr_win32_surface::VkWin32SurfaceCreateFlagsKhr};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};
use win32::{HINSTANCE, HWND};

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_win32_surface;

/// Structure specifying parameters of a newly created Win32 surface object
///
/// Provided by [`khr_win32_surface`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkWin32SurfaceCreateInfoKhr {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::Win32SurfaceCreateInfoKhr`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkWin32SurfaceCreateFlagsKhr,

    /// `hinstance` is the Win32 [`HINSTANCE`] for the window to associate the surface with.
    ///
    /// # Valid Usage
    ///  - `hinstance` must be a valid Win32 [`HINSTANCE`]
    pub hinstance: HINSTANCE,

    /// `hwnd` is the Win32 [`HWND`] for the window to associate the surface with.
    ///
    /// # Valid Usage
    ///  - `hwnd` must be a valid Win32 [`HWND`]
    pub hwnd: HWND,
}

impl const Default for VkWin32SurfaceCreateInfoKhr {
    fn default() -> Self {
        VkWin32SurfaceCreateInfoKhr {
            r#type: VkStructureType::Win32SurfaceCreateInfoKhr,
            next: null(),
            flags: VkWin32SurfaceCreateFlagsKhr::empty(),
            hinstance: null_mut(),
            hwnd: null_mut(),
        }
    }
}
