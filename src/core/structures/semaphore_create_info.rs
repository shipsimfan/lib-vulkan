use crate::{VkSemaphoreCreateFlags, VkStructureType};
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

impl const Default for VkSemaphoreCreateInfo {
    fn default() -> Self {
        Self {
            r#type: VkStructureType::SemaphoreCreateInfo,
            next: null_mut(),
            flags: VkSemaphoreCreateFlags::default(),
        }
    }
}
