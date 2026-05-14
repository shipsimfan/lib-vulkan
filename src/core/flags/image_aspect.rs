use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_3};

flags! {
    /// Bitmask of [`VkImageAspectFlag`]
    ///
    /// # Description
    /// [`VkImageAspectFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkImageAspectFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkImageAspectFlags;

    /// Bitmask specifying which aspects of an image are included in a view
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkImageAspectFlag {
        /// [`VkImageViewCreateFlag::Color`] specifies the color aspect.
        Color = 0x00000001,

        /// [`VkImageViewCreateFlag::Depth`] specifies the depth aspect.
        Depth = 0x00000002,

        /// [`VkImageViewCreateFlag::Stencil`] specifies the stencil aspect.
        Stencil = 0x00000004,

        /// [`VkImageViewCreateFlag::Metadata`] specifies the metadata aspect used for sparse
        /// resource operations.
        Metadata = 0x00000008,

        /// [`VkImageViewCreateFlag::Plane0`] specifies plane 0 of a multi-planar image format.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Plane0 = 0x00000010,

        /// [`VkImageViewCreateFlag::Plane1`] specifies plane 1 of a multi-planar image format.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Plane1 = 0x00000020,

        /// [`VkImageViewCreateFlag::Plane2`] specifies plane 2 of a multi-planar image format.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Plane2 = 0x00000040,

        /// [`VkImageViewCreateFlag::None`] specifies no image aspect, or the image aspect is not
        /// applicable.
        ///
        /// Provided by [`VK_VERSION_1_3`]
        None = 0,

        /// [`VkImageViewCreateFlag::MemoryPlane0Ext`] specifies memory plane 0.
        ///
        /// Provided by [`ext_image_drm_format_modifier`]
        MemoryPlane0Ext = 0x00000080,

        /// [`VkImageViewCreateFlag::MemoryPlane1Ext`] specifies memory plane 1.
        ///
        /// Provided by [`ext_image_drm_format_modifier`]
        MemoryPlane1Ext = 0x00000100,

        /// [`VkImageViewCreateFlag::MemoryPlane2Ext`] specifies memory plane 2.
        ///
        /// Provided by [`ext_image_drm_format_modifier`]
        MemoryPlane2Ext = 0x00000200,

        /// [`VkImageViewCreateFlag::MemoryPlane3Ext`] specifies memory plane 3.
        ///
        /// Provided by [`ext_image_drm_format_modifier`]
        MemoryPlane3Ext = 0x00000400,
    }
}
