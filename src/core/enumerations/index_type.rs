// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_4};

/// Type of index buffer indices
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkIndexType {
    /// [`VkIndexType::Uint16`] specifies that indices are 16-bit unsigned integer values.
    Uint16 = 0,

    /// [`VkIndexType::Uint32`] specifies that indices are 32-bit unsigned integer values.
    Uint32 = 1,

    /// [`VkIndexType::Uint8`] specifies that indices are 8-bit unsigned integer values.
    ///
    /// Provided by [`VK_VERSION_1_4`]
    Uint8 = 1000265000,

    /// [`VkIndexType::NoneKhr`] specifies that no indices are provided.
    ///
    /// Provided by [`khr_acceleration_structure`]
    NoneKhr = 1000165000,
}
