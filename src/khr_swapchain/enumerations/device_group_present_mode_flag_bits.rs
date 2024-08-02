// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_surface, khr_swapchain, VK_VERSION_1_1};

/// Bitmask specifying supported device group present modes
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_surface`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkDeviceGroupPresentModeFlagBitsKHR {
    /// [`VkDeviceGroupPresentModeFlagBitsKHR::LocalBitKhr`] specifies that any physical device
    /// with a presentation engine can present its own swapchain images.
    LocalBitKhr = 0x00000001,

    /// [`VkDeviceGroupPresentModeFlagBitsKHR::RemoteBitKhr`] specifies that any physical device
    /// with a presentation engine can present swapchain images from any physical device in its
    /// `present_mask`.
    RemoteBitKhr = 0x00000002,

    /// [`VkDeviceGroupPresentModeFlagBitsKHR::SumBitKhr`] specifies that any physical device with
    /// a presentation engine can present the sum of swapchain images from any physical devices in
    /// its `present_mask`.
    SumBitKhr = 0x00000004,

    /// [`VkDeviceGroupPresentModeFlagBitsKHR::LocalMultiDeviceBitKhr`] specifies that multiple
    /// physical devices with a presentation engine can each present their own swapchain images.
    LocalMultiDeviceBitKhr = 0x00000008,
}
