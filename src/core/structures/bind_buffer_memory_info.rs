use crate::{VkBuffer, VkDeviceMemory, VkDeviceSize, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_1, VkBufferCreateFlag, VkBufferCreateInfo,
    VkBufferUsageFlag, VkDevice, VkGetBufferMemoryRequirements2, VkMemoryAllocateInfo,
    VkMemoryHeapFlag, VkMemoryPropertyFlag, VkMemoryRequirements, VkPhysicalDeviceProperties,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Structure specifying how to bind a buffer to memory
///
/// # Valid Usage
///  - If `memory` was not allocated from a memory heap with the
///    [`VkMemoryHeapFlag::TileMemoryQcom`] property set, `memory_offset` must be an integer
///    multiple of the alignment member of the [`VkMemoryRequirements`] structure returned from a
///    call to [`VkGetBufferMemoryRequirements`] with `buffer`
///  - If `memory` was not allocated from a memory heap with the
///    [`VkMemoryHeapFlag::TileMemoryQcom`] property set, `size` member of the
///    [`VkMemoryRequirements`] structure returned from a call to [`VkGetBufferMemoryRequirements`]
///    with `buffer` must be less than or equal to the size of memory minus `memory_offset`
///  - If `buffer` was created with the [`VkBufferCreateFlag::Protected`] bit set, the buffer must
///    be bound to a memory object allocated with a memory type that reports
///    [`VkMemoryPropertyFlag::Protected`]
///  - If `buffer` was created with the [`VkBufferCreateFlag::Protected`] bit not set, the buffer
///    must not be bound to a memory object allocated with a memory type that reports
///    [`VkMemoryPropertyFlag::Protected`]
///
/// # Valid Usage (Implicit)
///  - Both of `buffer`, and `memory` must have been created, allocated, or retrieved from the same
///    [`VkDevice`]
///
/// Provided by [`VK_VERSION_1_1`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBindBufferMemoryInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::BindBufferMemoryInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkBindBufferMemoryDeviceGroupInfo`] structure, all
    ///    instances of memory specified by [`VkBindBufferMemoryDeviceGroupInfo::device_indices`]
    ///    must have been allocated
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkBindBufferMemoryDeviceGroupInfo`] or [`VkBindMemoryStatus`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `buffer` is the buffer to be attached to memory.
    ///
    /// # Valid Usage
    ///  - `buffer` must not have been bound to a memory object
    ///  - `buffer` must not have been created with any sparse memory binding flags
    ///  - If `buffer` requires a dedicated allocation (as reported by
    ///    [`VkGetBufferMemoryRequirements2`] in
    ///    [`VkMemoryDedicatedRequirements::requires_dedicated_allocation`] for `buffer`), memory
    ///    must have been allocated with [`VkMemoryDedicatedAllocateInfo::buffer`] equal to
    ///    `buffer`
    ///  - If `buffer` was created with
    ///    [`VkDedicatedAllocationBufferCreateInfoNv::dedicated_allocation`] equal to [`VK_TRUE`],
    ///    `memory` must have been allocated with
    ///    [`VkDedicatedAllocationMemoryAllocateInfoNv::buffer`] equal to a buffer handle created
    ///    with identical creation parameters to `buffer` and `memory_offset` must be zero
    ///  - If `buffer` was created with [`VkBufferCollectionBufferCreateInfoFuchsia`] chained to
    ///    [`VkBufferCreateInfo::next`], memory must be allocated with a
    ///    [`VkImportMemoryBufferCollectionFuchsia`] chained to [`VkMemoryAllocateInfo::next`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `buffer` must be a valid [`VkBuffer`] handle
    ///
    /// # Host Synchronization
    ///  - Host access to `buffer` must be externally synchronized
    pub buffer: VkBuffer,

    /// `memory` is a [`VkDeviceMemory`] object describing the device memory to attach.
    ///
    /// # Valid Usage
    ///  - `memory` must have been allocated using one of the memory types allowed in the
    ///    `memory_type_bits` member of the [`VkMemoryRequirements`] structure returned from a call
    ///    to [`VkGetBufferMemoryRequirements`] with `buffer`
    ///  - If `memory` was allocated from a memory heap with the
    ///    [`VkMemoryHeapFlag::TileMemoryQcom`] property set, `memory_offset` must be an integer
    ///    multiple of the alignment member of the [`VkTileMemoryRequirementsQcom`] structure
    ///    returned from a call to [`VkGetBufferMemoryRequirements`] with `buffer`
    ///  - If `memory` was allocated from a memory heap with the
    ///    [`VkMemoryHeapFlag::TileMemoryQcom`] property set, `size` member of the
    ///    [`VkTileMemoryRequirementsQcom`] structure returned from a call to
    ///    [`VkGetBufferMemoryRequirements`] with `buffer` must be less than or equal to the size
    ///    of memory minus `memory_offset`
    ///  - If the [`VkMemoryAllocateInfo`] provided when memory was allocated included a
    ///    [`VkMemoryDedicatedAllocateInfo`] structure in its `next` chain, and
    ///    [`VkMemoryDedicatedAllocateInfo::buffer`] was not [`VK_NULL_HANDLE`], then `buffer` must
    ///    equal [`VkMemoryDedicatedAllocateInfo::buffer`], and `memory_offset` must be zero
    ///  - If the [`VkMemoryAllocateInfo`] provided when memory was allocated included a
    ///    [`VkMemoryDedicatedAllocateInfo`] structure in its `next` chain,
    ///    [`VkMemoryDedicatedAllocateInfo::image`] must have been [`VK_NULL_HANDLE`]
    ///  - If the [`khr_dedicated_allocation`] extension is not enabled,
    ///    [`VkPhysicalDeviceProperties::api_version`] is less than Vulkan 1.1, and `buffer` was
    ///    not created with [`VkDedicatedAllocationBufferCreateInfoNv::dedicated_allocation`] equal
    ///    to [`VK_TRUE`], `memory` must not have been allocated dedicated for a specific buffer or
    ///    image
    ///  - If the value of [`VkExportMemoryAllocateInfo::handle_types`] used to allocate memory is
    ///    not 0, it must include at least one of the handles set in
    ///    [`VkExternalMemoryBufferCreateInfo::handle_types`] when `buffer` was created
    ///  - If `memory` was allocated by a memory import operation, that is not
    ///    [`VkImportAndroidHardwareBufferInfoAndroid`] with a non-[`null_mut`] `buffer` value, the
    ///    external handle type of the imported memory must also have been set in
    ///    [`VkExternalMemoryBufferCreateInfo::handle_types`] when `buffer` was created
    ///  - If `memory` was allocated with the [`VkImportAndroidHardwareBufferInfoAndroid`] memory
    ///    import operation with a non-[`null_mut`] `buffer` value,
    ///    [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`] must also have been set
    ///    in [`VkExternalMemoryBufferCreateInfo::handle_types`] when buffer was created
    ///  - If the [`VkPhysicalDeviceBufferDeviceAddressFeatures::buffer_device_address`] feature is
    ///    enabled and `buffer` was created with the [`VkBufferUsageFlag::ShaderDeviceAddress`]
    ///    usage flag set, `memory` must have been allocated with the
    ///    [`VkMemoryAllocateFlag::DeviceAddress`] bit set
    ///  - If the
    ///    [`VkPhysicalDeviceBufferDeviceAddressFeatures::buffer_device_address_capture_replay`]
    ///    feature is enabled and `buffer` was created with the
    ///    [`VkBufferCreateFlag::DeviceAddressCaptureReplay`] bit set, `memory` must have been
    ///    allocated with the [`VkMemoryAllocateFlag::DeviceAddressCaptureReplay`] bit set
    ///  - If the `buffer` was created with the
    ///    [`VkBufferCreateFlag::DescriptorBufferCaptureReplayExt`] bit set, `memory` must have
    ///    been allocated with the [`VkMemoryAllocateFlag::DeviceAddress`] bit set
    ///  - If the `buffer` was created with the
    ///    [`VkBufferCreateFlag::DescriptorBufferCaptureReplayExt`] bit set, `memory` must have
    ///    been allocated with the [`VkMemoryAllocateFlag::DeviceAddressCaptureReplay`] bit set
    ///  - If the `buffer` was created with the [`VkBufferUsageFlag::DescriptorHeapExt`] or
    ///    [`VkBufferUsageFlag2::DescriptorHeapExt`] bit set, `memory` must have been allocated
    ///    with the [`VkMemoryAllocateFlag::DeviceAddress`] bit set
    ///
    /// # Valid Usage (Implicit)
    ///  - `memory` must be a valid [`VkDeviceMemory`] handle
    pub memory: VkDeviceMemory,

    /// `memory_offset` is the start offset of the region of memory which is to be bound to the
    /// buffer. The number of bytes returned in the [`VkMemoryRequirements::size`] member in
    /// `memory`, starting from `memory_offset` bytes, will be bound to the specified buffer.
    ///
    /// # Valid Usage
    ///  - `memory_offset` must be less than the size of memory
    pub memory_offset: VkDeviceSize,
}

impl const Default for VkBindBufferMemoryInfo {
    fn default() -> Self {
        VkBindBufferMemoryInfo {
            r#type: VkStructureType::BindBufferMemoryInfo,
            next: null(),
            buffer: VkBuffer::null(),
            memory: VkDeviceMemory::null(),
            memory_offset: 0,
        }
    }
}

impl NextChain for VkBindBufferMemoryInfo {
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
