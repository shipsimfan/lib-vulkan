use crate::VkStructureType;
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Base structure for a read-only pointer chain
///
/// # Description
/// [`VkBaseOutStructure`] can be used to facilitate iterating through a structure pointer chain
/// that returns data back to the application.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkBaseOutStructure {
    /// `r#type` is the structure type of the structure being iterated through.
    pub r#type: VkStructureType,

    /// `next` is [`null_mut`] or a pointer to the next structure in a structure chain.
    pub next: *mut VkBaseOutStructure,
}

impl const Default for VkBaseOutStructure {
    fn default() -> Self {
        VkBaseOutStructure {
            r#type: VkStructureType::ApplicationInfo,
            next: null_mut(),
        }
    }
}
