use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a render pass object
    ///
    /// A render pass object represents a collection of attachments, subpasses, and dependencies
    /// between the subpasses, and describes how the attachments are used over the course of the
    /// subpasses.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkRenderPass
);
