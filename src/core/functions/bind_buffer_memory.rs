use crate::{VkBuffer, VkDevice, VkDeviceMemory, VkDeviceSize, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkBindBufferMemory2, VkBindBufferMemoryInfo,
    VkBufferCreateFlag, VkBufferCreateInfo, VkBufferUsageFlag, VkGetBufferMemoryRequirements,
    VkGetBufferMemoryRequirements2, VkMemoryAllocateInfo, VkMemoryHeapFlag, VkMemoryPropertyFlag,
    VkMemoryRequirements, VkPhysicalDeviceProperties,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Bind device memory to a buffer object
///
/// # Parameters
///  - `device` is the logical device that owns the buffer and memory.
///  - `buffer` is the buffer to be attached to memory.
///  - `memory` is a [`VkDeviceMemory`] object describing the device memory to attach.
///  - `memory_offset` is the start offset of the region of memory which is to be bound to the
///    buffer. The number of bytes returned in the [`VkMemoryRequirements::size`] member in memory,
///    starting from `memory_offset` bytes, will be bound to the specified buffer.
///
/// # Description
/// [`VkBindBufferMemory`] is equivalent to passing the same parameters through
/// [`VkBindBufferMemoryInfo`] to [`VkBindBufferMemory2`].
///
/// # Valid Usage (Implicit)
///  - `buffer` must not have been bound to a memory object
///  - `buffer` must not have been created with any sparse memory binding flags
///  - `memory_offset` must be less than the size of memory
///  - `memory` must have been allocated using one of the memory types allowed in the
///    `memory_type_bits` member of the [`VkMemoryRequirements`] structure returned from a call to
///    [`VkGetBufferMemoryRequirements`] with `buffer`
///  - If `memory` was not allocated from a memory heap with the
///    [`VkMemoryHeapFlag::TileMemoryQcom`] property set, `memory_offset` must be an integer
///    multiple of the `alignment` member of the [`VkMemoryRequirements`] structure returned from a
///    call to [`VkGetBufferMemoryRequirements`] with `buffer`
///  - If `memory` was allocated from a memory heap with the [`VkMemoryHeapFlag::TileMemoryQcom`]
///    property set, `memory_offset` must be an integer multiple of the alignment member of the
///    [`VkTileMemoryRequirementsQcom`] structure returned from a call to
///    [`VkGetBufferMemoryRequirements`] with `buffer`
///  - If `memory` was not allocated from a memory heap with the
///    [`VkMemoryHeapFlag::TileMemoryQcom`] property set, `size` member of the
///    [`VkMemoryRequirements`] structure returned from a call to [`VkGetBufferMemoryRequirements`]
///    with `buffer` must be less than or equal to the size of memory minus `memory_offset`
///  - If `memory` was allocated from a memory heap with the [`VkMemoryHeapFlag::TileMemoryQcom`]
///    property set, `size` member of the [`VkTileMemoryRequirementsQcom`] structure returned from
///    a call to [`VkGetBufferMemoryRequirements`] with `buffer` must be less than or equal to the
///    size of memory minus `memory_offset`
///  - If `buffer` requires a dedicated allocation (as reported by
///    [`VkGetBufferMemoryRequirements2`] in
///    [`VkMemoryDedicatedRequirements::requires_dedicated_allocation`] for `buffer`), `memory`
///    must have been allocated with [`VkMemoryDedicatedAllocateInfo::buffer`] equal to `buffer`
///  - If the [`VkMemoryAllocateInfo`] provided when `memory` was allocated included a
///    [`VkMemoryDedicatedAllocateInfo`] structure in its `next` chain, and
///    [`VkMemoryDedicatedAllocateInfo::buffer`] was not [`VK_NULL_HANDLE`], then `buffer` must
///    equal [`VkMemoryDedicatedAllocateInfo::buffer`], and `memory_offset` must be zero
///  - If the [`VkMemoryAllocateInfo`] provided when memory was allocated included a
///    [`VkMemoryDedicatedAllocateInfo`] structure in its `next` chain,
///    [`VkMemoryDedicatedAllocateInfo::image`] must have been [`VK_NULL_HANDLE`]
///  - If `buffer` was created with the [`VkBufferCreateFlag::Protected`] bit set, the `buffer`
///    must be bound to a memory object allocated with a memory type that reports
///    [`VkMemoryPropertyFlag::Protected`]
///  - If `buffer` was created with the [`VkBufferCreateFlag::Protected`] bit not set, the `buffer`
///    must not be bound to a memory object allocated with a memory type that reports
///    [`VkMemoryPropertyFlag::Protected`]
///  - If `buffer` was created with
///    [`VkDedicatedAllocationBufferCreateInfoNv::dedicated_allocation`] equal to [`VK_TRUE`],
///    `memory` must have been allocated with [`VkDedicatedAllocationMemoryAllocateInfoNv::buffer`]
///    equal to a buffer handle created with identical creation parameters to `buffer` and
///    `memory_offset` must be zero
///  - If the [`khr_dedicated_allocation`] extension is not enabled,
///    [`VkPhysicalDeviceProperties::api_version`] is less than Vulkan 1.1, and `buffer` was not
///    created with [`VkDedicatedAllocationBufferCreateInfoNv::dedicated_allocation`] equal to
///    [`VK_TRUE`], `memory` must not have been allocated dedicated for a specific buffer or image
///  - If the value of [`VkExportMemoryAllocateInfo::handle_types`] used to allocate memory is not
///    0, it must include at least one of the handles set in
///    [`VkExternalMemoryBufferCreateInfo::handle_types`] when `buffer` was created
///  - If `memory` was allocated by a memory import operation, that is not
///    [`VkImportAndroidHardwareBufferInfoAndroid`] with a non-[`null_mut`] buffer value, the
///    external handle type of the imported memory must also have been set in
///    [`VkExternalMemoryBufferCreateInfo::handle_types`] when `buffer` was created
///  - If `memory` was allocated with the [`VkImportAndroidHardwareBufferInfoAndroid`] memory
///    import operation with a non-[`null_mut`] buffer value,
///    [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`] must also have been set in
///    [`VkExternalMemoryBufferCreateInfo::handle_types`] when `buffer` was created
///  - If the [`VkPhysicalDeviceBufferDeviceAddressFeatures::buffer_device_address`] feature is
///    enabled and `buffer` was created with the [`VkBufferUsageFlag::ShaderDeviceAddress`] usage
///    flag set, `memory` must have been allocated with the [`VkMemoryAllocateFlag::DeviceAddress`]
///    bit set
///  - If the [`VkPhysicalDeviceBufferDeviceAddressFeatures::buffer_device_address_capture_replay`]
///    feature is enabled and `buffer` was created with the
///    [`VkBufferCreateFlag::DeviceAddressCaptureReplay`] bit set, memory must have been allocated
///    with the [`VkMemoryAllocateFlag::DeviceAddressCaptureReplay`] bit set
///  - If `buffer` was created with [`VkBufferCollectionBufferCreateInfoFuchsia`] chained to
///    [`VkBufferCreateInfo::next`], `memory` must be allocated with a
///    [`VkImportMemoryBufferCollectionFuchsia`] chained to [`VkMemoryAllocateInfo::next`]
///  - If the buffer was created with the [`VkBufferCreateFlag::DescriptorBufferCaptureReplayExt`]
///    bit set, `memory` must have been allocated with the [`VkMemoryAllocateFlag::DeviceAddress`]
///    bit set
///  - If the buffer was created with the [`VkBufferCreateFlag::DescriptorBufferCaptureReplayExt`]
///    bit set, `memory` must have been allocated with the
///    [`VkMemoryAllocateFlag::DeviceAddressCaptureReplay`] bit set
///  - If the buffer was created with the [`VkBufferUsageFlag::DescriptorHeapExt`] or
///    [`VkBufferUsageFlag2::DescriptorHeapExt`] bit set, `memory` must have been allocated with
///    the [`VkMemoryAllocateFlag::DeviceAddress`] bit set
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `buffer` must be a valid [`VkBuffer`] handle
///  - `memory` must be a valid [`VkDeviceMemory`] handle
///  - `buffer` must have been created, allocated, or retrieved from `device`
///  - `memory` must have been created, allocated, or retrieved from `device`
///
/// # Host Synchronization
///  - Host access to `buffer` must be externally synchronized
///
/// # Return Codes
///
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorInvalidOpaqueCaptureAddress`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
///  Provided by [`VK_VERSION_1_0`]
pub type VkBindBufferMemory = unsafe extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memory_offset: VkDeviceSize,
) -> VkResult;

/// The name of [`VkBindBufferMemory`]
pub const VK_BIND_BUFFER_MEMORY: &CStr = c"vkBindBufferMemory";
