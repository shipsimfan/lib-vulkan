use crate::VkPhysicalDeviceSparseProperties;

pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: bool,
    pub residency_standard_2d_multisample_block_shape: bool,
    pub residency_standard_3d_block_shape: bool,
    pub residency_aligned_mip_size: bool,
    pub residency_non_resident_strict: bool,
}

impl From<VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
    fn from(properties: VkPhysicalDeviceSparseProperties) -> Self {
        PhysicalDeviceSparseProperties {
            residency_standard_2d_block_shape: properties.residency_standard_2d_block_shape != 0,
            residency_standard_2d_multisample_block_shape: properties
                .residency_standard_2d_multisample_block_shape
                != 0,
            residency_standard_3d_block_shape: properties.residency_standard_3d_block_shape != 0,
            residency_aligned_mip_size: properties.residency_aligned_mip_size != 0,
            residency_non_resident_strict: properties.residency_non_resident_strict != 0,
        }
    }
}
