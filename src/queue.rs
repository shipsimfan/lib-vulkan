use crate::{
    bindings::{self, VkPresentInfoKHR},
    Loader, NativeLoader, Result, VkDevice, VkResult,
};
use std::sync::Arc;

pub struct VkQueue<L: Loader = NativeLoader> {
    inner: bindings::VkQueue,
    device: Arc<VkDevice<L>>,
}

impl<L: Loader> VkQueue<L> {
    pub(crate) fn new(inner: bindings::VkQueue, device: Arc<VkDevice<L>>) -> Self {
        VkQueue { inner, device }
    }

    pub fn queue_present(&mut self, present_info: &VkPresentInfoKHR) -> Result<()> {
        match (self.device.swapchain_functions().unwrap().queue_present)(self.inner, present_info) {
            VkResult::Success => Ok(()),
            result => Err(result),
        }
    }
}
