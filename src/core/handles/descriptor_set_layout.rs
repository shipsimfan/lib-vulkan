use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a descriptor set layout object
    ///
    /// A descriptor set layout object is defined by an array of zero or more descriptor bindings.
    /// Each individual descriptor binding is specified by a descriptor type, a count (array size)
    /// of the number of descriptors in the binding, a set of shader stages that can access the
    /// binding, and (if using immutable samplers) an array of sampler descriptors.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkDescriptorSetLayout
);
