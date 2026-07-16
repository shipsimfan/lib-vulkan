use crate::{VkSemaphoreCreateFlags, VkStructureType, util::NextChainMut};
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying parameters of a newly created semaphore
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSemaphoreCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::SemaphoreCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null_mut`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkExportMetalObjectCreateInfoExt`] structure, its
    ///    `export_object_type` member must be [`VkExportMetalObjectType::MetalSharedEventExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null_mut`] or a pointer to a valid instance of
    ///    [`VkExportMetalObjectCreateInfoExt`], [`VkExportSemaphoreCreateInfo`],
    ///    [`VkExportSemaphoreWin32HandleInfoKhr`], [`VkImportMetalSharedEventInfoExt`],
    ///    [`VkQueryLowLatencySupportNv`], or [`VkSemaphoreTypeCreateInfo`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique, with the
    ///    exception of structures of type [`VkExportMetalObjectCreateInfoExt`]
    pub next: *mut c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkSemaphoreCreateFlags,
}

const impl Default for VkSemaphoreCreateInfo {
    fn default() -> Self {
        Self {
            r#type: VkStructureType::SemaphoreCreateInfo,
            next: null_mut(),
            flags: VkSemaphoreCreateFlags::default(),
        }
    }
}

impl NextChainMut for VkSemaphoreCreateInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&mut self) -> *mut c_void {
        self.next
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        (self as *mut Self).cast()
    }

    fn set_next(&mut self, next: Option<&mut dyn NextChainMut>) {
        self.next = next.map_or(null_mut(), |n| n.as_mut_ptr());
    }
}
