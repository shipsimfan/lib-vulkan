use crate::{VkInstance, VkVoidFunction};
use std::{
    ffi::{c_char, CStr},
    path::Path,
    ptr::null_mut,
};

/// A loaded Vulkan driver
pub(in crate::loader) struct Driver {
    /// The driver itself
    library: Win32Library,

    /// The function to get other function pointers from
    icd_get_instance_proc_addr:
        extern "system" fn(instance: VkInstance, name: *const c_char) -> Option<VkVoidFunction>,
}

/// A open Windows library
struct Win32Library();

impl Driver {
    /// Attempts to open the [`Driver`] located at `path`
    pub(super) fn open(path: &Path) -> win32::Result<Self> {
        todo!()
    }

    pub(in crate::loader) fn icd_get_instance_proc_addr(&self, name: &CStr) -> Option<VkVoidFunction> {
        (self.icd_get_instance_proc_addr)(null_mut(), name.as_ptr())
    }
}
