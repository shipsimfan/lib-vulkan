use crate::{VkInstance, VkVoidFunction};
use std::ffi::{c_char, CStr};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

/// Return a function pointer for a command
///
/// # Parameters
///  * `instance` is the instance that the function pointer will be compatible with, or
///    [`null_mut`] for commands not dependent on any instance.
///  * `name` is the name of the command to obtain.
///
/// # Description
/// [`VkGetInstanceProcAddr`] itself is obtained in a platform- and loader- specific manner.
/// Typically, the loader library will export this command as a function symbol, so applications
/// can link against the loader library, or load it dynamically and look up the symbol using
/// platform-specific APIs.
///
/// The table below defines the various use cases for [`VkGetInstanceProcAddr`] and expected return
/// value (“fp” is “function pointer”) for each case. A valid returned function pointer (“fp”) must
/// not be [`null`].
///
/// The returned function pointer is of type [`VkVoidFunction`], and must be cast to the type of
/// the command being queried before use.
///
/// | instance                          | name                                                           | return value |
/// |-----------------------------------|----------------------------------------------------------------|--------------|
/// | *                                 | [`null`]                                                       | undefined    |
/// | invalid non-[`null_mut`] instance | *                                                              | undefined    |
/// | [`null_mut`]                      | global command                                                 | fp           |
/// | [`null_mut`]                      | "vkGetInstanceProcAddr"                                        | fp           |
/// | instance                          | "vkGetInstanceProcAddr"                                        | fp           |
/// | instance                          | core dispatchable command                                      | fp           |
/// | instance                          | enabled instance extension dispatchable command for `instance` | fp           |
/// | instance                          | available device extension dispatchable command for `instance` | fp           |
/// | any other case not covered above  | any other case, not covered above                              | [`null`]     |
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetInstanceProcAddr =
    extern "system" fn(instance: VkInstance, name: *const c_char) -> Option<VkVoidFunction>;

/// The name of [`VkGetInstanceProcAddr`]
pub const VK_GET_INSTANCE_PROC_ADDR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkGetInstanceProcAddr\0") };
