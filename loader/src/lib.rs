use std::{ffi::c_void, ptr::NonNull};

pub trait Loader {
    fn get_instance_proc_addr(
        &self,
        instance: Option<NonNull<c_void>>,
        name: &str,
    ) -> Option<extern "system" fn()>;
}
