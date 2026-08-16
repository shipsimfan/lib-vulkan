use crate::{VkMemoryAllocateFlags, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_1, VkMemoryAllocateFlag, VkMemoryHeapFlag};

/// Structure controlling how many instances of memory will be allocated
///
/// # Description
/// If [`VkMemoryAllocateFlag::DeviceMask`] is not set, the number of instances allocated depends
/// on whether [`VkMemoryHeapFlag::MultiInstance`] is set in the memory heap. If
/// [`VkMemoryHeapFlag::MultiInstance`] is set, then memory is allocated for every physical device
/// in the logical device (as if `device_mask` has bits set for all device indices). If
/// [`VkMemoryHeapFlag::MultiInstance`] is not set, then a single instance of memory is allocated
/// (as if `device_mask` is set to one).
///
/// On some implementations, allocations from a multi-instance heap may consume memory on all
/// physical devices even if the `device_mask` excludes some devices. If
/// [`VkPhysicalDeviceGroupProperties::subset_allocation`] is [`VK_TRUE`], then memory is only
/// consumed for the devices in the device mask.
///
/// Provided by [`VK_VERSION_1_1`]

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryAllocateFlagsInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::MemoryAllocateFlagsInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkMemoryAllocateFlag`]s controlling the allocation.
    ///
    /// # Valid Usage
    ///  - If the allocation is performing a memory import operation, then `flags` must not contain
    ///    [`VkMemoryAllocateFlag::ZeroInitializeExt`]
    ///  - If the allocation uses protected memory, then `flags` must not contain
    ///    [`VkMemoryAllocateFlag::ZeroInitializeExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkMemoryAllocateFlag`] values
    pub flags: VkMemoryAllocateFlags,

    /// `device_mask` is a mask of physical devices in the logical device, indicating that memory
    /// must be allocated on each device in the mask, if [`VkMemoryAllocateFlag::DeviceMask`] is
    /// set in `flags`.
    ///
    /// # Valid Usage
    ///  - If [`VkMemoryAllocateFlag::DeviceMask`] is set, `device_mask` must be a valid device
    ///    mask
    ///  - If [`VkMemoryAllocateFlag::DeviceMask`] is set, `device_mask` must not be zero
    pub device_mask: u32,
}

const impl Default for VkMemoryAllocateFlagsInfo {
    fn default() -> Self {
        VkMemoryAllocateFlagsInfo {
            r#type: VkStructureType::MemoryAllocateFlagsInfo,
            next: null(),
            flags: VkMemoryAllocateFlags::default(),
            device_mask: 0,
        }
    }
}

impl NextChain for VkMemoryAllocateFlagsInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
