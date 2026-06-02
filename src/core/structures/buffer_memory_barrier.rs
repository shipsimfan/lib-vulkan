use crate::{VkAccessFlags, VkBuffer, VkDeviceSize, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkAccessFlag, VkApplicationInfo, VkDependencyFlag, VkDeviceMemory, VkInstance,
    VkSharingMode,
};

/// Structure specifying a buffer memory barrier
///
/// # Description
/// The first access scope is limited to access to memory through the specified buffer range, via
/// access types in the source access mask specified by `src_access_mask` and, if a
/// [`VkMemoryBarrierAccessFlags3Khr`] is passed in `next`, `src_access_mask3`. If the source
/// access mask includes [`VkAccessFlag::HostWrite`], a memory domain operation is performed
/// where available memory in the host domain is also made available to the device domain.
///
/// The second access scope is limited to access to memory through the specified buffer range, via
/// access types in the destination access mask specified by `dst_access_mask` and, if a
/// [`VkMemoryBarrierAccessFlags3Khr`] is passed in `next`, `dst_access_mask3`. If the destination
/// access mask includes [`VkAccessFlag::HostWrite`] or [`VkAccessFlag::HostRead`], a memory
/// domain operation is performed where available memory in the device domain is also made
/// available to the host domain.
///
/// If `src_queue_family_index` is not equal to `dst_queue_family_index`, and
/// `src_queue_family_index` is equal to the current queue family, then the memory barrier defines
/// a queue family release operation for the specified buffer range, and if `dependency_flags` did
/// not include [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the second
/// synchronization scope of the calling command does not apply to this operation.
///
/// If `dst_queue_family_index` is not equal to `src_queue_family_index`, and
/// `dst_queue_family_index` is equal to the current queue family, then the memory barrier defines
/// a queue family acquire operation for the specified buffer range, and if `dependency_flags` did
/// not include [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the first
/// synchronization scope of the calling command does not apply to this operation.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferMemoryBarrier {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::MemoryBarrier`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`] or a pointer to a valid instance of
    ///    [`VkExternalMemoryAcquireUnmodifiedExt`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `src_access_mask` is a bitmask of [`VkAccessFlag`]s specifying a source access mask.
    pub src_access_mask: VkAccessFlags,

    /// `dst_access_mask` is a bitmask of [`VkAccessFlag`]s specifying a destination access mask.
    pub dst_access_mask: VkAccessFlags,

    /// `src_queue_family_index` is the source queue family for a queue family ownership transfer.
    ///
    /// # Valid Usage
    ///  - If `buffer` was created with a sharing mode of [`VkSharingMode::Exclusive`], and
    ///    `src_queue_family_index` and `dst_queue_family_index` are not equal,
    ///    `src_queue_family_index` must be [`VK_QUEUE_FAMILY_EXTERNAL`],
    ///    [`VK_QUEUE_FAMILY_FOREIGN_EXT`], or a valid queue family
    ///  - If the `khr_external_memory` extension is not enabled, and the value of
    ///    [`VkApplicationInfo::api_version`] used to create the [`VkInstance`] is not greater than
    ///    or equal to Version 1.1, `src_queue_family_index` must not be
    ///    [`VK_QUEUE_FAMILY_EXTERNAL`]
    ///  - If the `ext_queue_family_foreign` extension is not enabled `src_queue_family_index` must
    ///    not be [`VK_QUEUE_FAMILY_FOREIGN_EXT`]
    ///  - If the `synchronization2` feature is not enabled, and `buffer` was created with a
    ///    sharing mode of [`VkSharingMode::Concurrent`], `src_queue_family_index` must be
    ///    [`VK_QUEUE_FAMILY_IGNORED`] or [`VK_QUEUE_FAMILY_EXTERNAL`]
    pub src_queue_family_index: u32,

    /// `dst_queue_family_index` is the destination queue family for a queue family ownership
    /// transfer.
    ///
    /// # Valid Usage
    ///  - If `buffer` was created with a sharing mode of [`VkSharingMode::Exclusive`], and
    ///    `src_queue_family_index` and `dst_queue_family_index` are not equal,
    ///    `dst_queue_family_index` must be [`VK_QUEUE_FAMILY_EXTERNAL`],
    ///    [`VK_QUEUE_FAMILY_FOREIGN_EXT`], or a valid queue family
    ///  - If the `khr_external_memory` extension is not enabled, and the value of
    ///    [`VkApplicationInfo::api_version`] used to create the [`VkInstance`] is not greater than
    ///    or equal to Version 1.1, `dst_queue_family_index` must not be
    ///    [`VK_QUEUE_FAMILY_EXTERNAL`]
    ///  - If the `ext_queue_family_foreign` extension is not enabled `dst_queue_family_index` must
    ///    not be [`VK_QUEUE_FAMILY_FOREIGN_EXT`]
    ///  - If the `synchronization2` feature is not enabled, and `buffer` was created with a
    ///    sharing mode of [`VkSharingMode::Concurrent`], `dst_queue_family_index` must be
    ///    [`VK_QUEUE_FAMILY_IGNORED`] or [`VK_QUEUE_FAMILY_EXTERNAL`]
    pub dst_queue_family_index: u32,

    /// `buffer` is a handle to the buffer whose backing memory is affected by the barrier.
    ///
    /// # Valid Usage
    ///  - If `buffer` is non-sparse then it must be bound completely and contiguously to a single
    ///    [`VkDeviceMemory`] object
    ///
    /// # Valid Usage (Implicit)
    ///  - `buffer` must be a valid [`VkBuffer`] handle
    pub buffer: VkBuffer,

    /// `offset` is an offset in bytes into the backing memory for buffer; this is relative to the
    /// base offset as bound to the buffer
    ///
    /// # Valid Usage
    ///  - `offset` must be less than the size of buffer
    pub offset: VkDeviceSize,

    /// `size` is a size in bytes of the affected area of backing memory for buffer, or
    /// [`VK_WHOLE_SIZE`] to use the range from offset to the end of the buffer.
    ///
    /// # Valid Usage
    ///  - If `size` is not equal to [`VK_WHOLE_SIZE`], `size` must be greater than 0
    ///  - If `size` is not equal to [`VK_WHOLE_SIZE`], `size` must be less than or equal to than
    ///    the size of `buffer` minus `offset`
    pub size: VkDeviceSize,
}

impl const Default for VkBufferMemoryBarrier {
    fn default() -> Self {
        VkBufferMemoryBarrier {
            r#type: VkStructureType::BufferMemoryBarrier,
            next: null(),
            src_access_mask: VkAccessFlags::default(),
            dst_access_mask: VkAccessFlags::default(),
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            buffer: VkBuffer::null(),
            offset: 0,
            size: 0,
        }
    }
}

impl NextChain for VkBufferMemoryBarrier {
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
