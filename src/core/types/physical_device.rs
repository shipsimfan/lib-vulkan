use crate::vk_define_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_handle!(
    /// Opaque handle to a physical device object
    ///
    /// Vulkan separates the concept of physical and logical devices. A physical device usually
    /// represents a single complete implementation of Vulkan (excluding instance-level
    /// functionality) available to the host, of which there are a finite number. A logical device
    /// represents an instance of that implementation with its own state and resources independent
    /// of other logical devices.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkPhysicalDevice
);
