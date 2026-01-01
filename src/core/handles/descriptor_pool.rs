use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a descriptor pool object
    ///
    /// A descriptor pool maintains a pool of descriptors, from which descriptor sets are
    /// allocated. Descriptor pools are externally synchronized, meaning that the application must
    /// not allocate and/or free descriptor sets from the same pool in multiple threads
    /// simultaneously.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkDescriptorPool
);
