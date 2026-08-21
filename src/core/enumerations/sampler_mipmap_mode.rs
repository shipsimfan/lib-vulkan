// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Specify mipmap mode used for texture lookups
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkSamplerMipmapMode {
    /// [`VkSamplerMipmapMode::Nearest`] specifies nearest filtering.
    Nearest = 0,

    /// [`VkSamplerMipmapMode::Linear`] specifies linear filtering.
    Linear = 1,
}
