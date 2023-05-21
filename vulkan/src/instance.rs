use crate::{
    bindings::{self, VkDestroyInstance},
    get_instance_proc_addr, Result,
};
use loader::Loader;
use native::NativeLoader;
use std::sync::Arc;

pub struct VkInstance<L: Loader = NativeLoader> {
    inner: bindings::VkInstance,
    _loader: Arc<L>,

    destroy_instance: VkDestroyInstance,
}

impl<L: Loader> VkInstance<L> {
    pub(crate) fn new(inner: bindings::VkInstance, loader: Arc<L>) -> Result<Self> {
        let destroy_instance = get_instance_proc_addr!(loader, Some(inner), "vkDestroyInstance")?;

        Ok(VkInstance {
            inner,
            _loader: loader,

            destroy_instance,
        })
    }
}

impl<L: Loader> Drop for VkInstance<L> {
    fn drop(&mut self) {
        (self.destroy_instance)(self.inner, None);
    }
}
