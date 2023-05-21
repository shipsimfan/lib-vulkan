use crate::bindings;

pub struct VkInstance(bindings::VkInstance);

impl VkInstance {
    pub(crate) fn new(inner: bindings::VkInstance) -> Self {
        VkInstance(inner)
    }
}
