use crate::{Device, Loader, NativeLoader, VkQueue};
use std::sync::Arc;

pub struct Queue<L: Loader = NativeLoader> {
    handle: VkQueue,
    device: Arc<Device<L>>,
}

impl<L: Loader> Queue<L> {
    pub(crate) fn new(handle: VkQueue, device: Arc<Device<L>>) -> Self {
        Queue { handle, device }
    }
}
