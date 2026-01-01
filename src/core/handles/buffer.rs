use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a buffer object
    ///
    /// Buffers represent linear arrays of data which are used for various purposes by binding them
    /// to a graphics or compute pipeline via descriptor sets or certain commands, or by directly
    /// specifying them as parameters to certain commands.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkBuffer
);
