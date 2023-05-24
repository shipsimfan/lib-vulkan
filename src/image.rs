use crate::{bindings, Loader, NativeLoader, VkDevice};
use std::sync::Arc;

pub struct VkImage<L: Loader = NativeLoader> {
    inner: bindings::VkImage,
    device: Arc<VkDevice<L>>,
}

impl<L: Loader> VkImage<L> {
    pub fn new(inner: bindings::VkImage, device: Arc<VkDevice<L>>) -> Self {
        VkImage { inner, device }
    }

    pub(crate) fn inner(&self) -> bindings::VkImage {
        self.inner
    }

    pub(crate) fn device(&self) -> &Arc<VkDevice<L>> {
        &self.device
    }
}
