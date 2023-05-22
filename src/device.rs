use crate::{
    bindings::{self, VkDestroyDevice, VkGetDeviceQueue},
    get_device_proc_addr, Loader, NativeLoader, Result, VkInstance, VkQueue,
};
use std::{ptr::NonNull, sync::Arc};

pub struct VkDevice<L: Loader = NativeLoader> {
    inner: bindings::VkDevice,
    instance: Arc<VkInstance<L>>,

    destroy_device: VkDestroyDevice,
    get_device_queue: VkGetDeviceQueue,
}

impl<L: Loader> VkDevice<L> {
    pub(crate) fn new(
        inner: bindings::VkDevice,
        instance: Arc<VkInstance<L>>,
    ) -> Result<Arc<Self>> {
        let get_proc_addr = instance.get_device_proc_addr();

        let destroy_device = get_device_proc_addr!(get_proc_addr, inner, "vkDestroyDevice")?;
        let get_device_queue = get_device_proc_addr!(get_proc_addr, inner, "vkGetDeviceQueue")?;

        Ok(Arc::new(VkDevice {
            inner,
            instance,

            destroy_device,
            get_device_queue,
        }))
    }

    pub fn get_device_queue(
        self: &Arc<Self>,
        queue_family_index: u32,
        queue_index: u32,
    ) -> VkQueue<L> {
        let mut queue = None;
        (self.get_device_queue)(self.inner, queue_family_index, queue_index, unsafe {
            NonNull::new_unchecked(&mut queue)
        });
        VkQueue::new(queue.unwrap(), self.clone())
    }
}

impl<L: Loader> Drop for VkDevice<L> {
    fn drop(&mut self) {
        (self.destroy_device)(self.inner, None)
    }
}
