use crate::{Instance, Loader, NativeLoader, VkPhysicalDevice};
use std::sync::Arc;

pub struct PhysicalDevice<L: Loader = NativeLoader> {
    handle: VkPhysicalDevice,
    instance: Arc<Instance<L>>,
}

impl<L: Loader> PhysicalDevice<L> {
    pub(crate) fn new(handle: VkPhysicalDevice, instance: Arc<Instance<L>>) -> Self {
        PhysicalDevice { handle, instance }
    }
}
