use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkInstanceCreateFlag`]
    ///
    /// # Description
    /// [`VkInstanceCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkInstanceCreateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkInstanceCreateFlags;

    /// Bitmask specifying behavior of the instance
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkInstanceCreateFlag {
        /// [`VkInstanceCreateFlag::EnumeratePortabilityBitKhr`] specifies that the instance
        /// will enumerate available Vulkan Portability-compliant physical devices and groups in
        /// addition to the Vulkan physical devices and groups that are enumerated by default.
        ///
        /// Provided by [`khr_portability_enumeration`]
        EnumeratePortabilityBitKhr = 0x00000001,
    }
}
