use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_3;

flags! {
    /// Bitmask of [`VkSubmitFlag`]
    ///
    /// # Description
    /// [`VkSubmitFlags`] is a bitmask type for setting a mask of zero or more [`VkSubmitFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    pub struct VkSubmitFlags;


    /// Bitmask specifying behavior of a submission
    ///
    /// Provided by [`VK_VERSION_1_3`]
    pub enum VkSubmitFlag {
        /// [`VkSubmitFlag::Protected`] specifies that this batch is a protected submission.
        Protected = 0x00000001,
    }
}
