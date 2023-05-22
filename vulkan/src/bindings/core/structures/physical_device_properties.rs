use crate::{
    bindings::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE, VkPhysicalDeviceLimits,
    VkPhysicalDeviceSparseProperties, VkPhysicalDeviceType, VkVersion, VK_UUID_SIZE,
};

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    api_version: VkVersion,
    driver_version: u32,
    vendor_id: u32,
    device_id: u32,
    device_type: VkPhysicalDeviceType,
    device_name: [u8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pipeline_cache_uuid: [u8; VK_UUID_SIZE],
    limits: VkPhysicalDeviceLimits,
    sparse_properties: VkPhysicalDeviceSparseProperties,
}

impl VkPhysicalDeviceProperties {
    pub(crate) const fn null() -> Self {
        VkPhysicalDeviceProperties {
            api_version: VkVersion::new(0, 0, 0, 0),
            driver_version: 0,
            vendor_id: 0,
            device_id: 0,
            device_type: VkPhysicalDeviceType::Other,
            device_name: [0; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
            pipeline_cache_uuid: [0; VK_UUID_SIZE],
            limits: VkPhysicalDeviceLimits::null(),
            sparse_properties: VkPhysicalDeviceSparseProperties::null(),
        }
    }

    pub fn api_version(&self) -> VkVersion {
        self.api_version
    }

    pub fn driver_version(&self) -> u32 {
        self.driver_version
    }

    pub fn vendor_id(&self) -> u32 {
        self.vendor_id
    }

    pub fn device_id(&self) -> u32 {
        self.device_id
    }

    pub fn device_type(&self) -> VkPhysicalDeviceType {
        self.device_type
    }

    pub fn device_name(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.device_name) }.trim_matches('\0')
    }

    pub fn pipeline_cache_uuid(&self) -> &[u8; VK_UUID_SIZE] {
        &self.pipeline_cache_uuid
    }

    pub fn limits(&self) -> &VkPhysicalDeviceLimits {
        &self.limits
    }

    pub fn sparse_properties(&self) -> &VkPhysicalDeviceSparseProperties {
        &self.sparse_properties
    }
}
