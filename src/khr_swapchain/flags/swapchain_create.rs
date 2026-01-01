use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_1, khr_swapchain};

flags! {
    /// Bitmask of [`VkSwapchainCreateFlagKhr`]
    ///
    /// # Description
    /// [`VkSwapchainCreateFlagsKhr`] is a bitmask type for setting a mask of zero or more
    /// [`VkSwapchainCreateFlagKhr`].
    ///
    /// Provided by [`khr_swapchain`]
    pub struct VkSwapchainCreateFlagsKhr;

    /// Bitmask controlling swapchain creation
    ///
    /// Provided by [`khr_swapchain`]
    pub enum VkSwapchainCreateFlagKhr {
        /// [`VkSwapchainCreateFlagKhr::SplitInstanceBindRegionsBitKhr`] specifies that images
        /// created from the swapchain (i.e. with the swapchain member of
        /// [`VkImageSwapchainCreateInfoKhr`] set to this swapchain’s handle) must use
        /// [`SplitInstanceBindRegionsBit`].
        ///
        /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
        /// [`khr_swapchain`]
        SplitInstanceBindRegionsBitKhr = 0x00000001,

        /// [`VkSwapchainCreateFlagKhr::ProtectedBitKhr`] specifies that images created from the
        /// swapchain are protected images.
        ///
        /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`]
        ProtectedBitKhr = 0x00000002,

        /// [`VkSwapchainCreateFlagKhr::MutableFormatBitKhr`] specifies that the images of the
        /// swapchain can be used to create a VkImageView with a different format than what the
        /// swapchain was created with. The list of allowed image view formats is specified by adding a
        /// [`VkImageFormatListCreateInfo`] structure to the `next` chain of
        /// [`VkSwapchainCreateInfoKhr`]. In addition, this flag also specifies that the swapchain can
        /// be created with usage flags that are not supported for the format the swapchain is created
        /// with but are supported for at least one of the allowed image view formats.
        ///
        /// Provided by [`khr_swapchain_mutable_format`]
        MutableFormatBitKhr = 0x00000004,

        /// [`VkSwapchainCreateFlagKhr::DeferredMemoryAllocationBitExt`] specifies that the
        /// implementation may defer allocation of memory associated with each swapchain image until
        /// its index is to be returned from [`VkAcquireNextImageKhr`] or [`VkAcquireNextImage2Khr`]
        /// for the first time.
        ///
        /// Provided by [`ext_swapchain_maintenance1`]
        DeferredMemoryAllocationBitExt = 0x00000008,
    }
}
