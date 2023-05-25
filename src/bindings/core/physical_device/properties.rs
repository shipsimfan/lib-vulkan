use crate::{
    VkPhysicalDeviceLimits, VkPhysicalDeviceSparseProperties, VkPhysicalDeviceType,
    VK_MAX_PHYSICAL_DEVICE_NAME_SIZE, VK_UUID_SIZE,
};

#[repr(C)]
pub(crate) struct VkPhysicalDeviceProperties {
    pub(crate) api_version: u32,
    pub(crate) driver_version: u32,
    pub(crate) vendor_id: u32,
    pub(crate) device_id: u32,
    pub(crate) device_type: VkPhysicalDeviceType,
    pub(crate) device_name: [u8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub(crate) pipelane_chache_uuid: [u8; VK_UUID_SIZE],
    pub(crate) limits: VkPhysicalDeviceLimits,
    pub(crate) sparse_properties: VkPhysicalDeviceSparseProperties,
}

impl Default for VkPhysicalDeviceProperties {
    fn default() -> Self {
        VkPhysicalDeviceProperties {
            api_version: 0,
            driver_version: 0,
            vendor_id: 0,
            device_id: 0,
            device_type: VkPhysicalDeviceType::default(),
            device_name: [0; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
            pipelane_chache_uuid: [0; VK_UUID_SIZE],
            limits: VkPhysicalDeviceLimits::default(),
            sparse_properties: VkPhysicalDeviceSparseProperties::default(),
        }
    }
}
