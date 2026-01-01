use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a sampler object
    ///
    /// [`VkSampler`] objects represent the state of an image sampler which is used by the
    /// implementation to read image data and apply filtering and other transformations for the
    /// shader.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkSampler
);
