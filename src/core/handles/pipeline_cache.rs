use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a pipeline cache object
    ///
    /// Pipeline cache objects allow the result of pipeline construction to be reused between
    /// pipelines and between runs of an application. Reuse between pipelines is achieved by
    /// passing the same pipeline cache object when creating multiple related pipelines. Reuse
    /// across runs of an application is achieved by retrieving pipeline cache contents in one run
    /// of an application, saving the contents, and using them to preinitialize a pipeline cache on
    /// a subsequent run. The contents of the pipeline cache objects are managed by the
    /// implementation. Applications can manage the host memory consumed by a pipeline cache object
    /// and control the amount of data retrieved from a pipeline cache object.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkPipelineCache
);
