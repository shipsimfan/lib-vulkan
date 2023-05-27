use crate::{bindings::VkImage, Loader, NativeLoader, Swapchain};
use std::sync::Arc;

pub struct Image<L: Loader = NativeLoader> {
    handle: VkImage,
    swapchain: Arc<Swapchain<L>>,
}

impl<L: Loader> Image<L> {
    pub(crate) fn new(handle: VkImage, swapchain: Arc<Swapchain<L>>) -> Self {
        Image { handle, swapchain }
    }
}
