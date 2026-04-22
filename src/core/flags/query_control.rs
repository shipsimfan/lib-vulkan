use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkQueryControlFlag`]
    ///
    /// # Description
    /// [`VkQueryControlFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkQueryControlFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkQueryControlFlags;


    /// Bitmask specifying constraints on a query
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkQueryControlFlag {
        /// [`VkQueryControlFlag::PreciseBit`] specifies the precision of occlusion queries.
        PreciseBit = 0x00000001,
    }
}
