use crate::bindings::VkBool32;

#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
    residency_standard_2d_block_shape: VkBool32,
    residency_standard_2d_multisample_shape: VkBool32,
    residency_standard_3d_block_shape: VkBool32,
    residency_aligned_mip_size: VkBool32,
    residency_non_resident_strict: VkBool32,
}

impl VkPhysicalDeviceSparseProperties {
    pub(crate) const fn null() -> Self {
        VkPhysicalDeviceSparseProperties {
            residency_standard_2d_block_shape: 0,
            residency_standard_2d_multisample_shape: 0,
            residency_standard_3d_block_shape: 0,
            residency_aligned_mip_size: 0,
            residency_non_resident_strict: 0,
        }
    }

    pub fn residency_standard_2d_block_shape(&self) -> bool {
        self.residency_standard_2d_block_shape != 0
    }

    pub fn residency_standard_2d_multisample_shape(&self) -> bool {
        self.residency_standard_2d_multisample_shape != 0
    }

    pub fn residency_standard_3d_block_shape(&self) -> bool {
        self.residency_standard_3d_block_shape != 0
    }

    pub fn residency_aligned_mip_size(&self) -> bool {
        self.residency_aligned_mip_size != 0
    }

    pub fn residency_non_resident_strict(&self) -> bool {
        self.residency_non_resident_strict != 0
    }
}
