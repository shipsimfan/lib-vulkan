use loader::Loader;
use std::{ffi::c_void, ptr::NonNull};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use windows as os;

pub struct NativeLoader {
    _library: os::Library,
    get_instance_proc_addr: extern "system" fn(
        instance: Option<NonNull<c_void>>,
        p_name: *const u8,
    ) -> Option<extern "system" fn()>,
}

impl NativeLoader {
    pub fn new() -> Option<Self> {
        let library = os::Library::open(&win32::string_to_utf16!("vulkan-1"))?;
        let get_instance_proc_addr =
            unsafe { std::mem::transmute(library.get_proc_addr("vkGetInstanceProcAddr\0")?) };

        Some(NativeLoader {
            _library: library,
            get_instance_proc_addr,
        })
    }
}

impl Loader for NativeLoader {
    fn get_instance_proc_addr(
        &self,
        instance: Option<NonNull<c_void>>,
        name: &str,
    ) -> Option<extern "system" fn()> {
        debug_assert_eq!(name.as_bytes()[name.len() - 1], 0);
        (self.get_instance_proc_addr)(instance, name.as_ptr())
    }
}
