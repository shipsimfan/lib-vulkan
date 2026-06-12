use std::ffi::c_size_t;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkBool32};

/// Structure specifying a specialization map entry
///
/// # Description
/// If a `constant_id` value is not a specialization constant ID used in the shader, that map entry
/// does not affect the behavior of the pipeline.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSpecializationMapEntry {
    /// `constant_id` is the ID of the specialization constant in SPIR-V.
    ///
    /// # Valid Usage
    ///  - For a `constant_id` specialization constant declared in a shader, size must match the
    ///    byte size of the `constant_id`. If the specialization constant is of type boolean, size
    ///    must be the byte size of [`VkBool32`]
    pub constant_id: u32,

    /// `offset` is the byte offset of the specialization constant value within the supplied data
    /// buffer.
    pub offset: u32,

    /// `size` is the byte size of the specialization constant value within the supplied data
    /// buffer.
    pub size: c_size_t,
}

impl const Default for VkSpecializationMapEntry {
    fn default() -> Self {
        VkSpecializationMapEntry {
            constant_id: 0,
            offset: 0,
            size: 0,
        }
    }
}
