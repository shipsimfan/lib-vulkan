use crate::{
    VkStructureType, khr_wayland_surface::VkWaylandSurfaceCreateFlagsKhr, util::NextChain,
};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};
use wayland::{wl_display, wl_surface};

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_wayland_surface;

/// Structure specifying parameters of a newly created Wayland surface object
///
/// Provided by [`khr_wayland_surface`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkWaylandSurfaceCreateInfoKhr {
    ///  `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::WaylandSurfaceCreateInfoKhr`]
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
    pub flags: VkWaylandSurfaceCreateFlagsKhr,

    /// `display` is a pointer to a Wayland [`wl_display`] to associate the surface with
    ///
    /// # Valid Usage
    ///  - `display` must point to a valid Wayland [`wl_display`]
    pub display: *mut wl_display,

    /// `surface` is a pointer to a Wayland [`wl_surface`] to associate the surface with
    ///
    /// # Valid Usage
    ///  - `surface` must point to a valid Wayland [`wl_surface`]
    pub surface: *mut wl_surface,
}

impl const Default for VkWaylandSurfaceCreateInfoKhr {
    fn default() -> Self {
        VkWaylandSurfaceCreateInfoKhr {
            r#type: VkStructureType::WaylandSurfaceCreateInfoKhr,
            next: null(),
            flags: VkWaylandSurfaceCreateFlagsKhr::empty(),
            display: null_mut(),
            surface: null_mut(),
        }
    }
}

impl NextChain for VkWaylandSurfaceCreateInfoKhr {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }
    
    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
