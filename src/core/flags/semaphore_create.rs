use crate::flags_no_bits;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags_no_bits! {
    /// Reserved for future use
    ///
    /// # Description
    /// [`VkSemaphoreCreateFlags`] is a bitmask type for setting a mask, but is currently reserved
    /// for future use.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkSemaphoreCreateFlags;
}
