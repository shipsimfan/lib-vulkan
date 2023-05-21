use crate::{
    bindings::{self, VkDestroyInstance},
    get_instance_proc_addr, Result,
};
use loader::Loader;

pub struct VkInstance {
    inner: bindings::VkInstance,

    destroy_instance: VkDestroyInstance,
}

impl VkInstance {
    pub(crate) fn new<L: Loader>(inner: bindings::VkInstance, loader: &L) -> Result<Self> {
        let destroy_instance = get_instance_proc_addr!(loader, Some(inner), "vkDestroyInstance")?;

        Ok(VkInstance {
            inner,
            destroy_instance,
        })
    }
}

impl Drop for VkInstance {
    fn drop(&mut self) {
        (self.destroy_instance)(self.inner, None);
    }
}
