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
        /// [`VkSurfaceTransformFlagKhr::IdentityBitKhr`] specifies that image content is presented
        /// without being transformed.
        IdentityBitKhr = 0x00000001,

        /// [`VkSurfaceTransformFlagKhr::Rotate90BitKhr`] specifies that image content is rotated
        /// 90 degrees clockwise.
        Rotate90BitKhr = 0x00000002,

        /// [`VkSurfaceTransformFlagKhr::Rotate180BitKhr`] specifies that image content is rotated
        /// 180 degrees clockwise.
        Rotate180BitKhr = 0x00000004,

        /// [`VkSurfaceTransformFlagKhr::Rotate270BitKhr`] specifies that image content is rotated
        /// 270 degrees clockwise.
        Rotate270BitKhr = 0x00000008,

        /// [`VkSurfaceTransformFlagKhr::HorizontalMirrorBitKhr`] specifies that image content is
        /// mirrored horizontally.
        HorizontalMirrorBitKhr = 0x00000010,

        /// [`VkSurfaceTransformFlagKhr::HorizontalMirrorRotate90BitKhr`] specifies that image
        /// content is mirrored horizontally, then rotated 90 degrees clockwise.
        HorizontalMirrorRotate90BitKhr = 0x00000020,

        /// [`VkSurfaceTransformFlagKhr::HorizontalMirrorRotate180BitKhr`] specifies that image
        /// content is mirrored horizontally, then rotated 180 degrees clockwise.
        HorizontalMirrorRotate180BitKhr = 0x00000040,

        /// [`VkSurfaceTransformFlagKhr::HorizontalMirrorRotate270BitKhr`] specifies that image
        /// content is mirrored horizontally, then rotated 270 degrees clockwise.
        ///
        HorizontalMirrorRotate270BitKhr = 0x00000080,

        /// [`VkSurfaceTransformFlagKhr::InheritBitKhr`] specifies that the presentation transform
        /// is not specified, and is instead determined by platform-specific considerations and
        /// mechanisms outside Vulkan.
        InheritBitKhr = 0x00000100,
    }
}
