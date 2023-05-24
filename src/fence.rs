use crate::bindings;

pub struct VkFence {
    inner: bindings::VkFence,
}

impl VkFence {
    pub(crate) fn inner(&self) -> bindings::VkFence {
        self.inner
    }
}
