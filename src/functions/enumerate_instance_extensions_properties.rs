use crate::{VkExtensionProperties, VkResult};
use std::ffi::{c_char, CStr};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

/// Returns up to requested number of global extension properties
///
/// # Parameters
///  - `layer_name` is either [`null`] or a pointer to a null-terminated UTF-8 string naming the
///    layer to retrieve extensions from.
///  - `property_count` is a pointer to an integer related to the number of extension properties
///    available or queried, as described below.
///  - `properties` is either [`null_mut`] or a pointer to an array of [`VkExtensionProperties`]
///    structures.
///
/// # Description
/// When the `layer_name` parameter is [`null`], only extensions provided by the Vulkan
/// implementation or by implicitly enabled layers are returned. When `layer_name` is the name of
/// a layer, the instance extensions provided by that layer are returned.
///
/// If `properties` is [`null_mut`], then the number of extensions properties available is returned
/// in `property_count`. Otherwise, `property_count` must point to a variable set by the user to
/// the number of elements in the `properties` array, and on return the variable is overwritten
/// with the number of structures actually written to `properties`. If `property_count` is less
/// than the number of extension properties available, at most `property_count` structures will be
/// written, and [`VkResult::VkIncomplete`] will be returned instead of [`VkResult::VkSuccess`], to
/// indicate that not all the available properties were returned.
///
/// Because the list of available layers may change externally between calls to
/// [`VkEnumerateInstanceExtensionProperties`], two calls may retrieve different results if a
/// `layer_name` is available in one call but not in another. The extensions supported by a layer
/// may also change between two calls, e.g. if the layer implementation is replaced by a different
/// version between those calls.
///
/// Implementations must not advertise any pair of extensions that cannot be enabled together due
/// to behavioral differences, or any extension that cannot be enabled against the advertised
/// version.
/// 
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///  - [`VkResult::VkIncomplete`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorLayerNotPresent`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkEnumerateInstanceExtensionProperties = extern "system" fn(
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult;

/// The name of [`VkEnumerateInstanceExtensionProperties`]
pub const VK_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkEnumerateInstanceExtensionProperties\0") };
