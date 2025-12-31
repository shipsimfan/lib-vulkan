use crate::vk_define_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_handle!(
    /// Opaque handle to a queue object
    ///
    /// Creating a logical device also creates the queues associated with that device. The queues
    /// to create are described by a set of [`VkDeviceQueueCreateInfo`] structures that are passed
    /// to [`VkCreateDevice`] in `queue_create_infos`.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkQueue
);
