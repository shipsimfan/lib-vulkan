use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a device memory object
    ///
    /// A Vulkan device operates on data in device memory via memory objects.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkDeviceMemory
);
