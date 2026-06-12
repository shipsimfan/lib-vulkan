use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkCullModeFlag`]s
    ///
    /// # Description
    /// [`VkCullModeFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkCullModeFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkCullModeFlags;


    /// Bitmask controlling triangle culling
    ///
    /// # Description
    /// Once the orientation of triangles is determined, they are culled according to the
    /// [`VkPipelineRasterizationStateCreateInfo::cull_mode`] property of the currently active
    /// pipeline.
    ///
    /// Following culling, fragments are produced for any triangles which have not been discarded.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkCullModeFlag {
        /// [`VkCullModeFlag::None`] specifies that no triangles are discarded
        None = 0,

        /// [`VkCullModeFlag::Front`] specifies that front-facing triangles are discarded
        Front = 0x00000001,

        /// [`VkCullModeFlag::Back`] specifies that back-facing triangles are discarded
        Back = 0x00000002,

        /// [`VkCullModeFlag::FrontAndBack`] specifies that all triangles are discarded.
        FrontAndBack = 0x00000003,
    }
}
