use crate::bindings;

pub struct VkSemaphore {
    inner: bindings::VkSemaphore,
}

impl VkSemaphore {
    pub(crate) fn inner(&self) -> bindings::VkSemaphore {
        self.inner
    }
}
