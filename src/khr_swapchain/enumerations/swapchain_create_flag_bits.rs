// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VK_VERSION_1_1};

/// Bitmask controlling swapchain creation
///
/// Provided by [`khr_swapchain`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkSwapchainCreateFlagBitsKHR {
    /// [`VkSwapchainCreateFlagBitsKHR::SplitInstanceBindRegionsBitKhr`] specifies that images
    /// created from the swapchain (i.e. with the swapchain member of
    /// [`VkImageSwapchainCreateInfoKHR`] set to this swapchain’s handle) must use
    /// [`SplitInstanceBindRegionsBit`].
    ///
    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
    /// [`khr_swapchain`]
    SplitInstanceBindRegionsBitKhr = 0x00000001,

    /// [`VkSwapchainCreateFlagBitsKHR::ProtectedBitKhr`] specifies that images created from the
    /// swapchain are protected images.
    ///
    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`]
    ProtectedBitKhr = 0x00000002,

    /// [`VkSwapchainCreateFlagBitsKHR::MutableFormatBitKhr`] specifies that the images of the
    /// swapchain can be used to create a VkImageView with a different format than what the
    /// swapchain was created with. The list of allowed image view formats is specified by adding a
    /// [`VkImageFormatListCreateInfo`] structure to the `next` chain of
    /// [`VkSwapchainCreateInfoKHR`]. In addition, this flag also specifies that the swapchain can
    /// be created with usage flags that are not supported for the format the swapchain is created
    /// with but are supported for at least one of the allowed image view formats.
    ///
    /// Provided by [`khr_swapchain_mutable_format`]
    MutableFormatBitKhr = 0x00000004,

    /// [`VkSwapchainCreateFlagBitsKHR::DeferredMemoryAllocationBitExt`] specifies that the
    /// implementation may defer allocation of memory associated with each swapchain image until
    /// its index is to be returned from [`VkAcquireNextImageKHR`] or [`VkAcquireNextImage2KHR`]
    /// for the first time.
    ///
    /// Provided by [`ext_swapchain_maintenance1`]
    DeferredMemoryAllocationBitExt = 0x00000008,
}
