use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a framebuffer object
    ///
    /// Render passes operate in conjunction with framebuffers. Framebuffers represent a collection
    /// of specific memory attachments that a render pass instance uses.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkFramebuffer
);
