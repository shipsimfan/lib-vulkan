use crate::{loader::windows::str_to_utf16, VkInstance, VkVoidFunction};
use std::{
    ffi::{c_char, CStr},
    path::Path,
    ptr::null_mut,
};
use win32::{try_get_last_error, FreeLibrary, GetProcAddress, LoadLibrary, HMODULE};

/// A loaded Vulkan driver
pub(in crate::loader) struct Driver {
    /// The driver itself
    #[allow(unused)]
    library: Win32Library,

    /// The function to get other function pointers from
    icd_get_instance_proc_addr:
        extern "system" fn(instance: VkInstance, name: *const c_char) -> Option<VkVoidFunction>,
}

/// A open Windows library
struct Win32Library(HMODULE);

const VK_ICD_GET_INSTANCE_PROC_ADDR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vk_icdGetInstanceProcAddr\0") };

impl Driver {
    /// Attempts to open the [`Driver`] located at `path`
    pub(super) fn open(path: &Path) -> Option<Self> {
        let library = Win32Library::open(path).unwrap();

        let icd_get_instance_proc_addr = unsafe {
            std::mem::transmute(library.get_proc_address(VK_ICD_GET_INSTANCE_PROC_ADDR)?)
        };

        Some(Driver {
            library,
            icd_get_instance_proc_addr,
        })
    }

    /// Gets the address of the `name` vulkan global procedure
    pub(in crate::loader) fn icd_get_global_proc_addr(
        &self,
        name: &CStr,
    ) -> Option<VkVoidFunction> {
        (self.icd_get_instance_proc_addr)(null_mut(), name.as_ptr())
    }
}

impl Win32Library {
    /// Opens the Windows library at `path`
    pub(self) fn open(path: &Path) -> win32::Result<Self> {
        let path = str_to_utf16(path.to_string_lossy().as_ref());

        try_get_last_error!(LoadLibrary(path.as_ptr())).map(|handle| Win32Library(handle))
    }

    /// Gets the address of a procedure in this library
    pub(self) fn get_proc_address(&self, name: &CStr) -> Option<extern "system" fn()> {
        unsafe { GetProcAddress(self.0, name.as_ptr()) }
    }
}

impl Drop for Win32Library {
    fn drop(&mut self) {
        unsafe { FreeLibrary(self.0) };
    }
}
