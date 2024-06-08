// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

/// Alpha compositing modes supported on a device
///
/// Provided by [`khr_surface`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkCompositeAlphaFlagBitsKHR {
    /// The alpha component, if it exists, of the images is ignored in the compositing process.
    /// Instead, the image is treated as if it has a constant alpha of 1.0.
    OpaqueBitKHR = 0x00000001,

    /// The alpha component, if it exists, of the images is respected in the compositing process.
    /// The non-alpha components of the image are expected to already be multiplied by the alpha
    /// component by the application.
    PreMultipliedBitKHR = 0x00000002,

    /// The alpha component, if it exists, of the images is respected in the compositing process.
    /// The non-alpha components of the image are not expected to already be multiplied by the
    /// alpha component by the application; instead, the compositor will multiply the non-alpha
    /// components of the image by the alpha component during compositing.
    PostMultipliedBitKHR = 0x00000004,

    /// The way in which the presentation engine treats the alpha component in the images is
    /// unknown to the Vulkan API. Instead, the application is responsible for setting the
    /// composite alpha blending mode using native window system commands. If the application does
    /// not set the blending mode using native window system commands, then a platform-specific
    /// default will be used.
    InheritBitKHR = 0x00000008,
}
