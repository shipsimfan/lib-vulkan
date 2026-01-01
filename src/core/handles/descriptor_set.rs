use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a descriptor set object
    ///
    /// Descriptor sets are allocated from descriptor pool objects.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkDescriptorSet
);
