use crate::bindings;
use std::ffi::CStr;

mod native;

pub use native::NativeLoader;

pub trait Loader {
    fn get_instance_proc_addr(
        &self,
        instance: Option<bindings::VkInstance>,
        name: &CStr,
    ) -> Option<extern "system" fn()>;
}
