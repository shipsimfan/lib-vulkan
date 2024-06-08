// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Vulkan bitmasks
///
/// # Description
/// Bitmasks are passed to many commands and structures to compactly represent options, but
/// [`VkFlags`] is not used directly in the API. Instead, a `Vk*Flags` type which is an alias of
/// [`VkFlags`], and whose name matches the corresponding `Vk*FlagBits` that are valid for that
/// type, is used.
///
/// Any `Vk*Flags` member or parameter used in the API as an input must be a valid combination of
/// bit flags. A valid combination is either zero or the bitwise OR of valid bit flags.
///
/// An individual bit flag is valid for a `Vk*Flags` type if it would be a valid enumerant when
/// used with the equivalent `Vk*FlagBits` type, where the bits type is obtained by taking the flag
/// type and replacing the trailing `Flags` with `FlagBits`. For example, a flag value of type
/// [`VkColorComponentFlags`] must contain only bit flags defined by [`VkColorComponentFlagBits`].
///
/// Any `Vk*Flags` member or parameter returned from a query command or otherwise output from
/// Vulkan to the application may contain bit flags undefined in its corresponding `Vk*FlagBits`
/// type. An application cannot rely on the state of these unspecified bits.
///
/// Only the low-order 31 bits (bit positions zero through 30) are available for use as flag bits.
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkFlags = u32;
