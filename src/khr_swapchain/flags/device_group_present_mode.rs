use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_1, khr_surface, khr_swapchain};

flags! {
    /// Bitmask of [`VkDeviceGroupPresentModeFlagKhr`]
    ///
    /// # Decsription
    /// [`VkDeviceGroupPresentModeFlagsKhr`] is a bitmask type for setting a mask of zero or more
    /// [`VkDeviceGroupPresentModeFlagKhr`].
    ///
    /// Provided by [`khr_swapchain`]
    pub struct VkDeviceGroupPresentModeFlagsKhr;

    /// Bitmask specifying supported device group present modes
    ///
    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
    /// [`khr_surface`]
    pub enum VkDeviceGroupPresentModeFlagKhr {
        /// [`VkDeviceGroupPresentModeFlagKhr::LocalKhr`] specifies that any physical device
        /// with a presentation engine can present its own swapchain images.
        LocalKhr = 0x00000001,

        /// [`VkDeviceGroupPresentModeFlagKhr::RemoteKhr`] specifies that any physical device
        /// with a presentation engine can present swapchain images from any physical device in its
        /// `present_mask`.
        RemoteKhr = 0x00000002,

        /// [`VkDeviceGroupPresentModeFlagKhr::SumKhr`] specifies that any physical device with
        /// a presentation engine can present the sum of swapchain images from any physical devices in
        /// its `present_mask`.
        SumKhr = 0x00000004,

        /// [`VkDeviceGroupPresentModeFlagKhr::LocalMultiDeviceKhr`] specifies that multiple
        /// physical devices with a presentation engine can each present their own swapchain images.
        LocalMultiDeviceKhr = 0x00000008,
    }
}
