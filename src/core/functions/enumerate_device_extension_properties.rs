use crate::{VkExtensionProperties, VkPhysicalDevice, VkResult};
use std::ffi::{c_char, CStr};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

/// Returns properties of available physical device extensions
///
/// # Parameters
///  - `physical_device` is the physical device that will be queried.
///  - `layer_name` is either [`null`] or a pointer to a null-terminated UTF-8 string naming the
///    layer to retrieve extensions from.
///  - `property_count` is a pointer to an integer related to the number of extension properties
///    available or queried, and is treated in the same fashion as the
///    `VkEnumerateInstanceExtensionProperties::property_count` parameter.
///  - `properties` is either [`null_mut`] or a pointer to an array of [`VkExtensionProperties`]
///    structures.
///
/// # Description
/// When `layer_name` parameter is [`null`], only extensions provided by the Vulkan implementation
/// or by implicitly enabled layers are returned. When `layer_name` is the name of a layer, the
/// device extensions provided by that layer are returned.
///
/// Implementations must not advertise any pair of extensions that cannot be enabled together due
/// to behavioral differences, or any extension that cannot be enabled against the advertised
/// version.
///
/// Implementations claiming support for the Roadmap 2022 profile must advertise the
/// [`khr_global_priority`] extension in `properties`.
///
/// Implementations claiming support for the Roadmap 2024 profile must advertise the following
/// extensions in `properties`:
///  - [`khr_dynamic_rendering_local_read`]
///  - [`khr_load_store_op_none`]
///  - [`khr_shader_quad_control`]
///  - [`khr_shader_maximal_reconvergence`]
///  - [`khr_shader_subgroup_uniform_control_flow`]
///  - [`khr_shader_subgroup_rotate`]
///  - [`khr_shader_float_controls2`]
///  - [`khr_shader_expect_assume`]
///  - [`khr_line_rasterization`]
///  - [`khr_vertex_attribute_divisor`]
///  - [`khr_index_type_uint8`]
///  - [`khr_map_memory2`]
///  - [`khr_maintenance5`]
///  - [`khr_push_descriptor`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkEnumerateDeviceExtensionProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult;

/// The name of [`VkEnumerateInstanceExtensionProperties`]
pub const VK_ENUMERATE_DEVICE_EXTENSION_PROPERTIES: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkEnumerateDeviceExtensionProperties\0") };
