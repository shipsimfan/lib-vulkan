use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkMapMemory};

flags! {
    /// Bitmask of [`VkMemoryMapFlag`]
    ///
    /// # Description
    /// [`VkMemoryMapFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkMemoryMapFlag`].
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkMemoryMapFlags;


    /// Bitmask specifying additional parameters of a memory map
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkMemoryMapFlag {
        /// [`VkMemoryMapFlag::PlacedExt`] requests that the implementation place the memory map at
        /// the virtual address specified by the application via
        /// [`VkMemoryMapPlacedInfoExt::placed_address`], replacing any existing mapping at that
        /// address. This flag must not be used with [`VkMapMemory`] as there is no way to specify
        /// the placement address.
        ///
        /// Provided by [`ext_map_memory_placed`]
        PlacedExt = 0x00000001,
    }
}
