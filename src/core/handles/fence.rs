use crate::vk_define_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_handle!(
    /// Opaque handle to a fence object
    ///
    /// Fences are a synchronization primitive that can be used to insert a dependency from a queue
    /// to the host. Fences have two states - signaled and unsignaled. A fence can be signaled as
    /// part of the execution of a queue submission command. Fences can be unsignaled on the host
    /// with [`VkResetFences`]. Fences can be waited on by the host with the [`VkWaitForFences`]
    /// command, and the current state can be queried with [`VkGetFenceStatus`].
    ///
    /// The internal data of a fence may include a reference to any resources and pending work
    /// associated with signal or unsignal operations performed on that fence object, collectively
    /// referred to as the fence’s payload. Mechanisms to import and export that internal data to
    /// and from fences are provided below. These mechanisms indirectly enable applications to
    /// share fence state between two or more fences and other synchronization primitives across
    /// process and API boundaries.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkFence
);
