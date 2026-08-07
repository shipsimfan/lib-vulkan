use crate::{VkBufferDeviceAddressInfo, VkDevice, VkDeviceAddress};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_2, VkBuffer, VkBufferCreateFlag, VkDeviceMemory, VkMemoryRequirements};

/// Query an address of a buffer
///
/// # Parameters
///  - `device` is the logical device that the buffer was created on.
///  - `info` is a pointer to a [`VkBufferDeviceAddressInfo`] structure specifying the buffer to
///    retrieve an address for.
///
/// # Description
/// The 64-bit return value, `buffer_base_address`, is an address of the start of `info.buffer`.
/// Addresses in the range `[buffer_base_address, buffer_base_address + VkBufferCreateInfo::size)`
/// can be used to access the memory bound to this buffer on the device.
///
/// A value of zero is reserved as a “null” pointer and must not be returned as a valid buffer
/// device address.
///
/// If the buffer was created with a non-zero value of
/// [`VkBufferOpaqueCaptureAddressCreateInfo::opaque_capture_address`] or
/// [`VkBufferDeviceAddressCreateInfoExt::device_address`], the return value will be the same
/// address that was returned at capture time.
///
/// The returned address must satisfy the alignment requirement specified by
/// [`VkMemoryRequirements::alignment`] for the buffer in [`VkBufferDeviceAddressInfo::buffer`].
///
/// If multiple [`VkBuffer`] objects are bound to overlapping ranges of [`VkDeviceMemory`],
/// implementations may return address ranges which overlap. In this case, it is ambiguous which
/// [`VkBuffer`] is associated with any given device address. For purposes of valid usage, if
/// multiple [`VkBuffer`] objects can be attributed to a device address, a [`VkBuffer`] is selected
/// such that valid usage passes, if it exists.
///
/// # Valid Usage
///  - The `buffer_device_address` feature or the
///    [`VkPhysicalDeviceBufferDeviceAddressFeaturesExt::buffer_device_address]` feature must be
///    enabled, and at least one of the following conditions must be met
///    - `buffer` is sparse
///    - `buffer` is bound completely and contiguously to a single [`VkDeviceMemory`] object
///    - `buffer` was created with the [`VkBufferCreateFlag::DeviceAddressCaptureReplay`] flag and
///      the [`VkPhysicalDeviceBufferDeviceAddressFeaturesExt::buffer_device_address`] feature is
///      enabled on the device
///  - If `device` was created with multiple physical devices, then the
///    `buffer_device_address_multi_device` or
///    [`VkPhysicalDeviceBufferDeviceAddressFeaturesEXT::buffer_device_address_multi_device`]
///    feature must be enabled
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `info` must be a valid pointer to a valid [`VkBufferDeviceAddressInfo`] structure
///
/// Provided by [`VK_VERSION_1_2`]
pub type VkGetBufferDeviceAddress = unsafe extern "system" fn(
    device: VkDevice,
    info: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress;

/// The name of [`VkGetBufferDeviceAddress`]
pub const VK_GET_BUFFER_DEVICE_ADDRESS: &CStr = c"vkGetBufferDeviceAddress";
