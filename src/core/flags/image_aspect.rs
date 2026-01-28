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
        /// [`VkImageViewCreateFlag::ColorBit`] specifies the color aspect.
        ColorBit = 0x00000001,

        /// [`VkImageViewCreateFlag::DepthBit`] specifies the depth aspect.
        DepthBit = 0x00000002,

        /// [`VkImageViewCreateFlag::StencilBit`] specifies the stencil aspect.
        StencilBit = 0x00000004,

        /// [`VkImageViewCreateFlag::MetadataBit`] specifies the metadata aspect used for sparse
        /// resource operations.
        MetadataBit = 0x00000008,

        /// [`VkImageViewCreateFlag::Plane0Bit`] specifies plane 0 of a multi-planar image format.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Plane0Bit = 0x00000010,

        /// [`VkImageViewCreateFlag::Plane1Bit`] specifies plane 1 of a multi-planar image format.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Plane1Bit = 0x00000020,

        /// [`VkImageViewCreateFlag::Plane2Bit`] specifies plane 2 of a multi-planar image format.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Plane2Bit = 0x00000040,

        /// [`VkImageViewCreateFlag::None`] specifies no image aspect, or the image aspect is not
        /// applicable.
        ///
        /// Provided by [`VK_VERSION_1_3`]
        None = 0,

        /// [`VkImageViewCreateFlag::MemoryPlane0BitExt`] specifies memory plane 0.
        ///
        /// Provided by [`ext_image_drm_format_modifier`]
        MemoryPlane0BitExt = 0x00000080,

        /// [`VkImageViewCreateFlag::MemoryPlane1BitExt`] specifies memory plane 1.
        ///
        /// Provided by [`ext_image_drm_format_modifier`]
        MemoryPlane1BitExt = 0x00000100,

        /// [`VkImageViewCreateFlag::MemoryPlane2BitExt`] specifies memory plane 2.
        ///
        /// Provided by [`ext_image_drm_format_modifier`]
        MemoryPlane2BitExt = 0x00000200,

        /// [`VkImageViewCreateFlag::MemoryPlane3BitExt`] specifies memory plane 3.
        ///
        /// Provided by [`ext_image_drm_format_modifier`]
        MemoryPlane3BitExt = 0x00000400,
    }
}
