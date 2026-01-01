use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a pipeline layout object
    ///
    /// Access to descriptor sets from a pipeline is accomplished through a pipeline layout. Zero
    /// or more descriptor set layouts and zero or more push constant ranges are combined to form a
    /// pipeline layout object describing the complete set of resources that can be accessed by a
    /// pipeline. The pipeline layout represents a sequence of descriptor sets with each having a
    /// specific layout. This sequence of layouts is used to determine the interface between shader
    /// stages and shader resources. Each pipeline is created using a pipeline layout.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkPipelineLayout
);
