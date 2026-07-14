use crate::{VkAllocationCallbacks, VkDevice, VkDeviceMemory, VkMemoryAllocateInfo, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkGetPhysicalDeviceMemoryProperties, VkMemoryHeap, VkMemoryHeapFlag,
    VkMemoryPropertyFlag, VkPhysicalDevice, VkPhysicalDeviceMemoryProperties,
};
#[allow(unused_imports)]
use std::ptr::null;

/// Allocate device memory
///
/// # Parameters
///  - `device` is the logical device that owns the memory.
///  - `allocate_info` is a pointer to a [`VkMemoryAllocateInfo`] structure describing parameters
///    of the allocation. A successfully returned allocation must use the requested parameters — no
///    substitution is permitted by the implementation.
///  - `allocator` controls host memory allocation.
///  - `memory` is a pointer to a [`VkDeviceMemory`] handle in which information about the
///    allocated memory is returned.
///
/// # Description
/// Allocations returned by [`VkAllocateMemory`] are guaranteed to meet any alignment requirement
/// of the implementation. For example, if an implementation requires 128 byte alignment for images
/// and 64 byte alignment for buffers, the device memory returned through this mechanism would be
/// 128-byte aligned. This ensures that applications can correctly suballocate objects of different
/// types (with potentially different alignment requirements) in the same memory object.
///
/// When memory is allocated, its contents are undefined with the following constraint:
///  - The contents of unprotected memory must not be a function of the contents of data protected
///    memory objects, even if those memory objects were previously freed.
///
/// The maximum number of valid memory allocations that can exist simultaneously within a
/// [`VkDevice`] may be restricted by implementation- or platform-dependent limits. The
/// `max_memory_allocation_count` feature describes the number of allocations that can exist
/// simultaneously before encountering these internal limits.
///
/// Some platforms may have a limit on the maximum size of a single allocation. For example,
/// certain systems may fail to create allocations with a size greater than or equal to 4GB. Such
/// a limit is implementation-dependent, and if such a failure occurs then the error V
/// [`VkResult::VkErrorOutOfDeviceMemory`] must be returned. This limit is advertised in
/// [`VkPhysicalDeviceMaintenance3Properties::max_memory_allocation_size`].
///
/// The cumulative memory size allocated to a heap can be limited by the size of the specified
/// heap. In such cases, allocated memory is tracked on a per-device and per-heap basis. Some
/// platforms allow overallocation into other heaps. The overallocation behavior can be specified
/// through the [`amd_memory_overallocation_behavior`] extension.
///
/// If the `memory_type_index` belongs to a heap with the [`VkMemoryHeapFlag::TileMemoryQcom`] bit
/// included in its properties, this allocation is backed by tile memory, which is an on device
/// cache. Unlike other heaps, allocations out of the tile memory will always have a starting
/// address at the start of the heap and its contents are aliased with all other [`VkDeviceMemory`]
/// objects bound to the same range while executing within the same tile memory scope.
///
/// If the [`VkPhysicalDevicePageableDeviceLocalMemoryFeaturesExt::pageable_device_local_memory`]
/// feature is enabled, memory allocations made from a heap that includes
/// [`VkMemoryHeapFlag::DeviceLocal`] in [`VkMemoryHeap::flags`] may be transparently moved to
/// host-local memory allowing multiple applications to share device-local memory. If there is no
/// space left in device-local memory when this new allocation is made, other allocations may be
/// moved out transparently to make room. The operating system will determine which allocations to
/// move to device-local memory or host-local memory based on platform-specific criteria. To help
/// the operating system make good choices, the application should set the appropriate memory
/// priority with [`VkMemoryPriorityAllocateInfoExt`] and adjust it as necessary with
/// [`VkSetDeviceMemoryPriorityExt`]. Higher priority allocations will moved to device-local memory
/// first.
///
/// Memory allocations made on heaps without the [`VkMemoryHeapFlag::DeviceLocal`] property will
/// not be transparently promoted to device-local memory by the operating system.
///
/// # Valid Usage
///  - `allocate_info.allocation_size` must be less than or equal to
///    `VkPhysicalDeviceMemoryProperties::memory_heaps[memindex].size` where
///    `memindex = VkPhysicalDeviceMemoryProperties::memory_types[allocate_info.memory_type_index].heap_index`
///    as returned by [`VkGetPhysicalDeviceMemoryProperties`] for the [`VkPhysicalDevice`] that
///    `device` was created from
///  - `allocate_info.memory_type_index` must be less than
///    [`VkPhysicalDeviceMemoryProperties::memory_type_count`] as returned by
///    [`VkGetPhysicalDeviceMemoryProperties`] for the [`VkPhysicalDevice`] that device was created
///    from
///  - If the `device_coherent_memory` feature is not enabled, `allocate_info.memory_type_index`
///    must not identify a memory type supporting [`VkMemoryPropertyFlag::DeviceCoherentAmd`]
///  - There must be less than [`VkPhysicalDeviceLimits::max_memory_allocation_count`] device
///    memory allocations currently allocated on the device
///  - If the `tile_memory_heap` feature is not enabled, `allocate_info.memory_type_index` must not
///    identify a memory type that corresponds to a [`VkMemoryHeap`] with the
///    [`VkMemoryHeapFlag::TileMemoryQcom`] property
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `allocate_info` must be a valid pointer to a valid [`VkMemoryAllocateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `memory` must be a valid pointer to a [`VkDeviceMemory`] handle
///  - The device must have been created with at least 1 queue
///
/// # Return Codes
///
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorInvalidExternalHandle`]
///  - [`VkResult::VkErrorInvalidOpaqueCaptureAddress`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkAllocateMemory = unsafe extern "system" fn(
    device: VkDevice,
    allocate_info: *const VkMemoryAllocateInfo,
    allocator: *const VkAllocationCallbacks,
    memory: *mut VkDeviceMemory,
) -> VkResult;

/// The name of [`VkAllocateMemory`]
pub const VK_ALLOCATE_MEMORY: &CStr = c"vkAllocateMemory";
