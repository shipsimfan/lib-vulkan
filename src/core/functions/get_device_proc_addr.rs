use crate::{VkDevice, VkVoidFunction};
use std::ffi::{c_char, CStr};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkGetInstanceProcAddr, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

/// Return a function pointer for a command
///
/// In order to support systems with multiple Vulkan implementations, the function pointers
/// returned by [`VkGetInstanceProcAddr`] may point to dispatch code that calls a different real
/// implementation for different [`VkDevice`] objects or their child objects. The overhead of the
/// internal dispatch for [`VkDevice`] objects can be avoided by obtaining device-specific function
/// pointers for any commands that use a device or device-child object as their dispatchable
/// object.
///
/// # Description
/// The table below defines the various use cases for [`VkGetDeviceProcAddr`] and expected return
/// value (“fp” is “function pointer”) for each case. A valid returned function pointer (“fp”) must
/// not be [`null`].
///
/// The returned function pointer is of type [`VkVoidFunction`], and must be cast to the type of
/// the command being queried before use. The function pointer must only be called with a
/// dispatchable object (the first parameter) that is device or a child of device.
///
/// | `device`                          | `name`                                                   | return value |
/// |-----------------------------------|----------------------------------------------------------|--------------|
/// | [`null_mut`]                      | *                                                        | undefined    |
/// | invalid device                    | *                                                        | undefined    |
/// | device                            | [`null`]                                                 | undefined    |
/// | device                            | requested core version device-level dispatchable command | fp           |
/// | device                            | enabled extension device-level dispatchable command      | fp           |
/// | any other case not covered above  | any other case, not covered above                        | [`null`]     |
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetDeviceProcAddr =
    extern "system" fn(device: VkDevice, name: *const c_char) -> Option<VkVoidFunction>;

/// The name of [`VkGetDeviceProcAddr`]
pub const VK_GET_DEVICE_PROC_ADDR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceProcAddr\0") };
