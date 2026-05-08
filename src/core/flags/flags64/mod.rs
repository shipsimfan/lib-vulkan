mod clone;
mod contains;
mod display;
mod eq;
mod into;
mod new;
mod set;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkAccessFlag2, VkAccessFlags2, VkFlags};

/// Vulkan 64-bit bitmasks
///
/// # Description
/// When the 31 bits available in [`VkFlags`] are insufficient, the [`VkFlags64`] type can be
/// passed to commands and structures to represent up to 64 options. [`VkFlags64`] is not used
/// directly in the API. Instead, a `Vk*Flags2` type which is an alias of [`VkFlags64`], and whose
/// name matches the corresponding `Vk*Flag2` that are valid for that type, is used.
///
/// Any `Vk*Flags2` member or parameter used in the API as an input must be a valid combination of
/// bit flags. A valid combination is either zero or the bitwise OR of valid bit flags.
///
/// An individual bit flag is valid for a `Vk*Flags2` type if it would be a valid enumerant when
/// used with the equivalent `Vk*Flag2` type, where the bits type is obtained by taking the flag
/// type and replacing the trailing `Flags2` with `Flag2`. For example, a flag value of type
/// [`VkAccessFlags2`] must contain only bit flags defined by [`VkAccessFlag2`].
///
/// Any `Vk*Flags2` member or parameter returned from a query command or otherwise output from
/// Vulkan to the application may contain bit flags undefined in its corresponding `Vk*Flag2` type.
/// An application cannot rely on the state of these unspecified bits.
///
/// Provided by [`VK_VERSION_1_3`], [`khr_synchronization2`]
#[repr(C)]
#[derive(Debug)]
pub struct VkFlags64(pub u64);
