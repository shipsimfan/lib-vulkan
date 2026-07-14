use crate::{VkDevice, VkDeviceMemory, VkDeviceSize, VkMemoryMapFlags, VkResult};
use std::ffi::{CStr, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VK_WHOLE_SIZE, VkMemoryMapFlag, VkMemoryPropertyFlag, VkPhysicalDeviceLimits,
};

/// Map a memory object into application address space
///
/// # Parameters
///  - `device` is the logical device that owns the memory.
///  - `memory` is the [`VkDeviceMemory`] object to be mapped.
///  - `offset` is a zero-based byte offset from the beginning of the memory object.
///  - `size` is the size of the memory range to map, or [`VK_WHOLE_SIZE`] to map from offset to
///    the end of the allocation.
///  - `flags` is a bitmask of [`VkMemoryMapFlag`]s specifying additional parameters of the memory
///    map operation.
///  - `data` is a pointer to a `*mut c_void` variable in which a host-accessible pointer to the
///    beginning of the mapped range is returned. The value of the returned pointer minus `offset`
///    must be aligned to [`VkPhysicalDeviceLimits::min_memory_map_alignment`].
///
/// # Description
/// After a successful call to [`VkMapMemory`] the memory object memory is considered to be
/// currently host mapped.
///
/// [`VkMapMemory`] does not check whether the device memory is currently in use before returning
/// the host-accessible pointer. The application must guarantee that any previously submitted
/// command that writes to this range has completed before the host reads from or writes to that
/// range, and that any previously submitted command that reads from that range has completed
/// before the host writes to that region (see here for details on fulfilling such a guarantee). If
/// the device memory was allocated without the [`VkMemoryPropertyFlag::HostCoherent`] set, these
/// guarantees must be made for an extended range: the application must round down the start of the
/// range to the nearest multiple of [`VkPhysicalDeviceLimits::non_coherent_atom_size`], and round
/// the end of the range up to the nearest multiple of
/// [`VkPhysicalDeviceLimits::non_coherent_atom_size`].
///
/// While a range of device memory is host mapped, the application is responsible for synchronizing
/// both device and host access to that memory range.
///
/// Calling [`VkMapMemory`] is equivalent to calling [`VkMapMemory2`] with an empty `next` chain.
///
/// # Valid Usage
///  - `memory` must not be currently host mapped
///  - `offset` must be less than the size of `memory`
///  - If `size` is not equal to [`VK_WHOLE_SIZE`], `size` must be greater than 0
///  - If `size` is not equal to [`VK_WHOLE_SIZE`], `size` must be less than or equal to the size
///    of the memory minus `offset`
///  - `memory` must have been created with a memory type that reports
///    [`VkMemoryPropertyFlag::HostVisible`]
///  - `memory` must not have been allocated with multiple instances
///  - [`VkMemoryMapFlag::PlacedExt`] must not be set in `flags`
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `memory` must be a valid [VkDeviceMemory] handle
///  - `flags` must be a valid combination of [`VkMemoryMapFlag`] values
///  - `ppData` must be a valid pointer to a pointer value
///  - `memory` must have been created, allocated, or retrieved from device
///
/// # Host Synchronization
///  - Host access to `memory` must be externally synchronized
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorMemoryMapFailed`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkMapMemory = unsafe extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    data: *mut *mut c_void,
) -> VkResult;

/// The name of [`VkMapMemory`]
pub const VK_MAP_MEMORY: &CStr = c"vkMapMemory";
