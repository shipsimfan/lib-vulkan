use loader::Loader;
use native::NativeLoader;

use crate::{bindings, VkDevice};
use std::sync::Arc;

pub struct VkQueue<L: Loader = NativeLoader> {
    inner: bindings::VkQueue,
    device: Arc<VkDevice<L>>,
}

impl<L: Loader> VkQueue<L> {
    pub(crate) fn new(inner: bindings::VkQueue, device: Arc<VkDevice<L>>) -> Self {
        VkQueue { inner, device }
    }
}
