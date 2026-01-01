use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a pipeline object
    ///
    /// Compute, ray tracing, and graphics pipelines are each represented by [`VkPipeline`]
    /// handles.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkPipeline
);
