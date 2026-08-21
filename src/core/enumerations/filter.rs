// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Specify filters used for texture lookups
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkFilter {
    /// [`VkFilter::Nearest`] specifies nearest filtering.
    Nearest = 0,

    /// [`VkFilter::Linear`] specifies linear filtering.
    Linear = 1,

    /// [`VkFilter::CubicExt`] specifies cubic filtering.
    ///
    /// Provided by [`ext_filter_cubic`]
    CubicExt = 1000015000,
}
