use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkFenceCreateFlag`]
    ///
    /// # Description
    /// [`VkFenceCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkFenceCreateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkFenceCreateFlags;

    /// Bitmask specifying initial state and behavior of a fence
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkFenceCreateFlag {
        /// [`VkFenceCreateFlag::Signalled`] specifies that the fence object is created in the
        /// signaled state. Otherwise, it is created in the unsignaled state.
        Signalled = 0x00000001,
    }
}
