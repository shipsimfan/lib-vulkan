use crate::VkStructureType;
use std::ptr::null;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Base structure for a read-only pointer chain
///
/// # Description
/// [`VkBaseInStructure`] can be used to facilitate iterating through a read-only structure pointer
/// chain.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkBaseInStructure {
    /// `r#type` is the structure type of the structure being iterated through.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to the next structure in a structure chain.
    pub next: *const VkBaseInStructure,
}

impl const Default for VkBaseInStructure {
    fn default() -> Self {
        VkBaseInStructure {
            r#type: VkStructureType::ApplicationInfo,
            next: null(),
        }
    }
}
