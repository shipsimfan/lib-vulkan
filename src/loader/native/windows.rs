use std::ffi::CStr;
use win32::{FarProc, HModule};

pub struct Library(HModule);

impl Library {
    pub fn open(name: &[u16]) -> Option<Self> {
        win32::load_library_ex(name, &[])
            .ok()
            .map(|module| Library(module))
    }

    pub fn get_proc_addr(&self, proc: &CStr) -> Option<FarProc> {
        win32::get_proc_address(self.0, proc).ok()
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        win32::free_library(self.0).ok();
    }
}
