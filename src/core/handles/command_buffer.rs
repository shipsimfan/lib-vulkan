use crate::vk_define_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_handle!(
    /// Opaque handle to a command buffer object
    ///
    /// Command buffers are objects used to record commands which can be subsequently submitted to
    /// a device queue for execution. There are two levels of command buffers - primary command
    /// buffers, which can execute secondary command buffers, and which are submitted to queues,
    /// and secondary command buffers, which can be executed by primary command buffers, and which
    /// are not directly submitted to queues.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkCommandBuffer
);
