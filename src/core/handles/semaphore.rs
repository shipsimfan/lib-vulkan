use crate::vk_define_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_handle!(
    /// Opaque handle to a semaphore object
    ///
    /// Semaphores are a synchronization primitive that can be used to insert a dependency between
    /// queue operations or between a queue operation and the host. Binary semaphores have two
    /// states - signaled and unsignaled. Timeline semaphores have a strictly increasing 64-bit
    /// unsigned integer payload and are signaled with respect to a particular reference value. A
    /// semaphore can be signaled after execution of a queue operation is completed, and a queue
    /// operation can wait for a semaphore to become signaled before it begins execution. A
    /// timeline semaphore can additionally be signaled from the host with the
    /// [`VkSignalSemaphore`] command and waited on from the host with the [`VkWaitSemaphores`]
    /// command.
    ///
    /// The internal data of a semaphore may include a reference to any resources and pending work
    /// associated with signal or unsignal operations performed on that semaphore object,
    /// collectively referred to as the semaphore’s payload. Mechanisms to import and export that
    /// internal data to and from semaphores are provided below. These mechanisms indirectly enable
    /// applications to share semaphore state between two or more semaphores and other
    /// synchronization primitives across process and API boundaries.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkSemaphore
);
