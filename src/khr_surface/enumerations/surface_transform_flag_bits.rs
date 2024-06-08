// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

/// Presentation transforms supported on a device
///
/// Provided by [`khr_surface`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkSurfaceTransformFlagBitsKHR {
    /// [`VkSurfaceTransformFlagBitsKHR::IdentityBitKHR`] specifies that image content is presented
    /// without being transformed.
    IdentityBitKHR = 0x00000001,

    /// [`VkSurfaceTransformFlagBitsKHR::Rotate90BitKHR`] specifies that image content is rotated
    /// 90 degrees clockwise.
    Rotate90BitKHR = 0x00000002,

    /// [`VkSurfaceTransformFlagBitsKHR::Rotate180BitKHR`] specifies that image content is rotated
    /// 180 degrees clockwise.
    Rotate180BitKHR = 0x00000004,

    /// [`VkSurfaceTransformFlagBitsKHR::Rotate270BitKHR`] specifies that image content is rotated
    /// 270 degrees clockwise.
    Rotate270BitKHR = 0x00000008,

    /// [`VkSurfaceTransformFlagBitsKHR::HorizontalMirrorBitKHR`] specifies that image content is
    /// mirrored horizontally.
    HorizontalMirrorBitKHR = 0x00000010,

    /// [`VkSurfaceTransformFlagBitsKHR::HorizontalMirrorRotate90BitKHR`] specifies that image
    /// content is mirrored horizontally, then rotated 90 degrees clockwise.
    HorizontalMirrorRotate90BitKHR = 0x00000020,

    /// [`VkSurfaceTransformFlagBitsKHR::HorizontalMirrorRotate180BitKHR`] specifies that image
    /// content is mirrored horizontally, then rotated 180 degrees clockwise.
    HorizontalMirrorRotate180BitKHR = 0x00000040,

    /// [`VkSurfaceTransformFlagBitsKHR::HorizontalMirrorRotate270BitKHR`] specifies that image
    /// content is mirrored horizontally, then rotated 270 degrees clockwise.
    ///
    HorizontalMirrorRotate270BitKHR = 0x00000080,

    /// [`VkSurfaceTransformFlagBitsKHR::InheritBitKHR`] specifies that the presentation transform
    /// is not specified, and is instead determined by platform-specific considerations and
    /// mechanisms outside Vulkan.
    InheritBitKHR = 0x00000100,
}
