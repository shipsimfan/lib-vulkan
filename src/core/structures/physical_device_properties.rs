use crate::{
    VkPhysicalDeviceLimits, VkPhysicalDeviceSparseProperties, VkPhysicalDeviceType,
    VK_MAX_PHYSICAL_DEVICE_NAME_SIZE, VK_UUID_SIZE,
};
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkPhysicalDevice, VK_VERSION_1_0};

/// Structure specifying physical device properties
///
/// On implementations that claim support for the Roadmap 2022 profile, the major and minor version
/// expressed by `api_version` must be at least Vulkan 1.3.
///
/// The `vendor_id` and `device_id` fields are provided to allow applications to adapt to device
/// characteristics that are not adequately exposed by other Vulkan queries.
///
/// The vendor identified by `vendor_id` is the entity responsible for the most salient
/// characteristics of the underlying implementation of the [`VkPhysicalDevice`] being queried.
///
/// If the vendor has a PCI vendor ID, the low 16 bits of `vendor_id` must contain that PCI vendor
/// ID, and the remaining bits must be set to zero. Otherwise, the value returned must be a valid
/// Khronos vendor ID. Khronos vendor IDs are allocated starting at 0x10000, to distinguish them
/// from the PCI vendor ID namespace. Khronos vendor IDs are symbolically defined in the
/// [`VkVendorId`] type.
///
/// The vendor is also responsible for the value returned in `device_id`. If the implementation is
/// driven primarily by a PCI device with a PCI device ID, the low 16 bits of `device_id` must
/// contain that PCI device ID, and the remaining bits must be set to zero. Otherwise, the choice
/// of what values to return may be dictated by operating system or platform policies - but should
/// uniquely identify both the device version and any major configuration options (for example,
/// core count in the case of multicore devices).
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkPhysicalDeviceProperties {
    /// `api_version` is the version of Vulkan supported by the device.
    pub api_version: u32,

    /// `driver_version` is the vendor-specified version of the driver.
    pub driver_version: u32,

    /// `vendor_id` is a unique identifier for the vendor of the physical device.
    pub vendor_id: u32,

    /// `device_id` is a unique identifier for the physical device among devices available from the
    /// vendor.
    pub device_id: u32,

    /// `device_type` is a [`VkPhysicalDeviceType`] specifying the type of device.
    pub device_type: VkPhysicalDeviceType,

    /// `device_name` is an array of [`VK_MAX_PHYSICAL_DEVICE_NAME_SIZE`] char containing a
    /// null-terminated UTF-8 string which is the name of the device.
    pub device_name: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],

    /// `pipeline_cache_uuid` is an array of [`VK_UUID_SIZE`] uint8_t values representing a universally
    /// unique identifier for the device.
    pub pipeline_cache_uuid: [u8; VK_UUID_SIZE],

    /// `limits` is the [`VkPhysicalDeviceLimits`] structure specifying device-specific limits of
    /// the physical device.
    pub limits: VkPhysicalDeviceLimits,

    /// `sparse_properties` is the [`VkPhysicalDeviceSparseProperties`] structure specifying
    /// various sparse related properties of the physical device.
    pub sparse_properties: VkPhysicalDeviceSparseProperties,
}
