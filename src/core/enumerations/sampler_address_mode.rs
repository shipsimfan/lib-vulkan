// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_2};

/// Specify behavior of sampling with texture coordinates outside an image
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkSamplerAddressMode {
    /// [`VkSamplerAddressMode::Repeat`] specifies that the repeat wrap mode will be used.
    Repeat = 0,

    /// [`VkSamplerAddressMode::MirroredRepeat`] specifies that the mirrored repeat wrap mode will
    /// be used.
    MirroredRepeat = 1,

    /// [`VkSamplerAddressMode::ClampToEdge`] specifies that the clamp to edge wrap mode will be
    /// used.
    ClampToEdge = 2,

    /// [`VkSamplerAddressMode::ClampToBorder`] specifies that the clamp to border wrap mode will
    /// be used.
    ClampToBorder = 3,

    /// [`VkSamplerAddressMode::MirrorClampToEdge`] specifies that the mirror clamp to edge wrap
    /// mode will be used. This is only valid if the `sampler_mirror_clamp_to_edge` feature is
    /// enabled, or if the [`khr_sampler_mirror_clamp_to_edge`] extension is enabled.
    ///
    /// Provided by [`VK_VERSION_1_2`], [`khr_sampler_mirror_clamp_to_edge`]
    MirrorClampToEdge = 4,
}
