use std::ffi::CString;
use win32::HModule;

pub struct Library(HModule);

impl Library {
    pub fn open(name: &[u16]) -> Option<Self> {
        win32::load_library_ex(name, &[])
            .ok()
            .map(|module| Library(module))
    }

    pub fn get_proc_addr(&self, proc: &str) -> Option<*const ()> {
        let proc = CString::new(proc).unwrap();
        win32::get_proc_address(self.0, &proc)
            .ok()
            .map(|function| function as *const ())
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        win32::free_library(self.0).ok();
    }
}
