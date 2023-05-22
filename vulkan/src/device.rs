use crate::{
    bindings::{self, VkDestroyDevice},
    get_device_proc_addr, Result, VkInstance,
};
use loader::Loader;
use native::NativeLoader;
use std::sync::Arc;

pub struct VkDevice<L: Loader = NativeLoader> {
    inner: bindings::VkDevice,
    instance: Arc<VkInstance<L>>,

    destroy_device: VkDestroyDevice,
}

impl<L: Loader> VkDevice<L> {
    pub(crate) fn new(inner: bindings::VkDevice, instance: Arc<VkInstance<L>>) -> Result<Self> {
        let get_proc_addr = instance.get_device_proc_addr();

        let destroy_device = get_device_proc_addr!(get_proc_addr, inner, "vkDestroyDevice")?;

        Ok(VkDevice {
            inner,
            instance,
            destroy_device,
        })
    }
}

impl<L: Loader> Drop for VkDevice<L> {
    fn drop(&mut self) {
        (self.destroy_device)(self.inner, None)
    }
}
