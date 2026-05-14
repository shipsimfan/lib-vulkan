use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

flags! {
    /// Bitmask of [`VkSurfaceTransformFlagKhr`]
    ///
    /// # Description
    /// [`VkSurfaceTransformFlagsKhr`] is a bitmask type for setting a mask of zero or more
    /// [`VkSurfaceTransformFlagKhr`].
    ///
    /// Provided by [`khr_surface`]
    pub struct VkSurfaceTransformFlagsKhr;

    /// Presentation transforms supported on a device
    ///
    /// Provided by [`khr_surface`]
    pub enum VkSurfaceTransformFlagKhr {
        /// [`VkSurfaceTransformFlagKhr::IdentityKhr`] specifies that image content is presented
        /// without being transformed.
        IdentityKhr = 0x00000001,

        /// [`VkSurfaceTransformFlagKhr::Rotate90Khr`] specifies that image content is rotated
        /// 90 degrees clockwise.
        Rotate90Khr = 0x00000002,

        /// [`VkSurfaceTransformFlagKhr::Rotate180Khr`] specifies that image content is rotated
        /// 180 degrees clockwise.
        Rotate180Khr = 0x00000004,

        /// [`VkSurfaceTransformFlagKhr::Rotate270Khr`] specifies that image content is rotated
        /// 270 degrees clockwise.
        Rotate270Khr = 0x00000008,

        /// [`VkSurfaceTransformFlagKhr::HorizontalMirrorKhr`] specifies that image content is
        /// mirrored horizontally.
        HorizontalMirrorKhr = 0x00000010,

        /// [`VkSurfaceTransformFlagKhr::HorizontalMirrorRotate90Khr`] specifies that image
        /// content is mirrored horizontally, then rotated 90 degrees clockwise.
        HorizontalMirrorRotate90Khr = 0x00000020,

        /// [`VkSurfaceTransformFlagKhr::HorizontalMirrorRotate180Khr`] specifies that image
        /// content is mirrored horizontally, then rotated 180 degrees clockwise.
        HorizontalMirrorRotate180Khr = 0x00000040,

        /// [`VkSurfaceTransformFlagKhr::HorizontalMirrorRotate270Khr`] specifies that image
        /// content is mirrored horizontally, then rotated 270 degrees clockwise.
        ///
        HorizontalMirrorRotate270Khr = 0x00000080,

        /// [`VkSurfaceTransformFlagKhr::InheritKhr`] specifies that the presentation transform
        /// is not specified, and is instead determined by platform-specific considerations and
        /// mechanisms outside Vulkan.
        InheritKhr = 0x00000100,
    }
}
