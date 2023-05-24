use super::Loader;
use crate::VkInstance;
use std::ffi::{c_char, CStr, CString};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use windows as os;

pub struct NativeLoader {
    _library: os::Library,
    get_instance_proc_addr: extern "system" fn(
        instance: Option<VkInstance>,
        p_name: *const c_char,
    ) -> Option<*const ()>,
}

impl NativeLoader {
    pub fn new() -> Option<Self> {
        let library = os::Library::open(&win32::string_to_utf16!("vulkan-1"))?;
        let get_instance_proc_addr =
            unsafe {
                std::mem::transmute(library.get_proc_addr(
                    CStr::from_bytes_with_nul(b"vkGetInstanceProcAddr\0").unwrap(),
                )?)
            };

        Some(NativeLoader {
            _library: library,
            get_instance_proc_addr,
        })
    }
}

impl Loader for NativeLoader {
    fn get_instance_proc_addr(
        &self,
        instance: Option<VkInstance>,
        name: &str,
    ) -> Option<*const ()> {
        let name = CString::new(name).unwrap();

        (self.get_instance_proc_addr)(instance, name.as_ptr())
    }
}
