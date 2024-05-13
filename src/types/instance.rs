use crate::vk_define_handle;

vk_define_handle!(
    /// Opaque handle to an instance object
    ///
    /// There is no global state in Vulkan and all per-application state is stored in a
    /// [`VkInstance`] object. Creating a [`VkInstance`] object initializes the Vulkan library and
    /// allows the application to pass information about itself to the implementation.
    ///
    /// Instances are represented by [`VkInstance`] handles.
    VkInstance
);
