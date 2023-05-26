use crate::{
    PhysicalDeviceLimits, PhysicalDeviceSparseProperties, PhysicalDeviceType,
    VkPhysicalDeviceProperties, VK_UUID_SIZE,
};
use std::ffi::CStr;

pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: String,
    pub pipelane_chache_uuid: [u8; VK_UUID_SIZE],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

impl From<VkPhysicalDeviceProperties> for PhysicalDeviceProperties {
    fn from(properties: VkPhysicalDeviceProperties) -> Self {
        PhysicalDeviceProperties {
            api_version: properties.api_version,
            driver_version: properties.driver_version,
            vendor_id: properties.vendor_id,
            device_id: properties.device_id,
            device_type: properties.device_type,
            device_name: CStr::from_bytes_until_nul(&properties.device_name)
                .unwrap()
                .to_string_lossy()
                .to_string(),
            pipelane_chache_uuid: properties.pipelane_chache_uuid,
            limits: properties.limits.into(),
            sparse_properties: properties.sparse_properties.into(),
        }
    }
}
