use crate::VkInstance;

mod native;

pub use native::NativeLoader;

pub trait Loader {
    fn get_instance_proc_addr(&self, instance: Option<VkInstance>, name: &str)
        -> Option<*const ()>;
}
