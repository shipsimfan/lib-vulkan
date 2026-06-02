use crate::{VkFenceCreateFlags, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkFenceCreateFlag};

/// Structure specifying parameters of a newly created fence
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkFenceCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::FenceCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkExportFenceCreateInfo`] or
    ///    [`VkExportFenceWin32HandleInfoKhr`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkFenceCreateFlag`]s specifying the initial state and behavior of
    /// the fence.
    ///
    /// # Valid Usage (Implicit)
    ///  - flags must be a valid combination of [`VkFenceCreateFlag`] values
    pub flags: VkFenceCreateFlags,
}

impl const Default for VkFenceCreateInfo {
    fn default() -> Self {
        Self {
            r#type: VkStructureType::FenceCreateInfo,
            next: null(),
            flags: VkFenceCreateFlags::default(),
        }
    }
}

impl NextChain for VkFenceCreateInfo {
    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: *const c_void) {
        self.next = next;
    }
}
