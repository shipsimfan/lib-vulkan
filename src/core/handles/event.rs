use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to an event object
    ///
    /// Events are a synchronization primitive that can be used to insert a fine-grained dependency
    /// between commands submitted to the same queue, or between the host and a queue. Events must
    /// not be used to insert a dependency between commands submitted to different queues. Events
    /// have two states - signaled and unsignaled. An application can signal or unsignal an event
    /// either on the host or on the device. A device can be made to wait for an event to become
    /// signaled before executing further operations. No command exists to wait for an event to
    /// become signaled on the host, but the current state of an event can be queried.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkEvent
);
