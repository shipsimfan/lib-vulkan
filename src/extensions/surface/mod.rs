use crate::{Instance, Loader, NativeLoader, VkSurfaceKHR};
use std::{ptr::null, sync::Arc};

mod functions;

pub(crate) use functions::SurfaceFunctions;

pub struct Surface<L: Loader = NativeLoader> {
    handle: VkSurfaceKHR,
    instance: Arc<Instance<L>>,
}

impl<L: Loader> Surface<L> {
    pub(crate) fn new(handle: VkSurfaceKHR, instance: Arc<Instance<L>>) -> Self {
        Surface { handle, instance }
    }

    pub(crate) fn handle(&self) -> VkSurfaceKHR {
        self.handle
    }
}

impl<L: Loader> Drop for Surface<L> {
    fn drop(&mut self) {
        (self.instance.surface_functions().destroy_surface)(
            self.instance.handle(),
            self.handle,
            null(),
        )
    }
}
