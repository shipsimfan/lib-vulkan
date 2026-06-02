use crate::{VkAccessFlags, VkStructureType};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkAccessFlag};

/// Structure specifying a global memory barrier
///
/// # Description
/// The first access scope is limited to access types in the source access mask specified by
/// `src_access_mask` and, if a [`VkMemoryBarrierAccessFlags3Khr`] is passed in `next`,
/// `src_access_mask3`.
///
/// The second access scope is limited to access types in the destination access mask specified by
/// `dst_access_mask` and, if a [`VkMemoryBarrierAccessFlags3Khr`] is passed in `next`,
/// `dst_access_mask3`.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkMemoryBarrier {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::MemoryBarrier`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `src_access_mask` is a bitmask of [`VkAccessFlag`]s specifying a source access mask.
    ///
    /// # Valid Usage (Implicit)
    ///  - `src_access_mask` must be a valid combination of [`VkAccessFlag`] values
    pub src_access_mask: VkAccessFlags,

    /// `dst_access_mask` is a bitmask of [`VkAccessFlag`]s specifying a destination access mask.
    ///
    /// # Valid Usage (Implicit)
    ///  - `dst_access_mask` must be a valid combination of [`VkAccessFlag`] values
    pub dst_access_mask: VkAccessFlags,
}

impl const Default for VkMemoryBarrier {
    fn default() -> Self {
        VkMemoryBarrier {
            r#type: VkStructureType::MemoryBarrier,
            next: null(),
            src_access_mask: VkAccessFlags::default(),
            dst_access_mask: VkAccessFlags::default(),
        }
    }
}
