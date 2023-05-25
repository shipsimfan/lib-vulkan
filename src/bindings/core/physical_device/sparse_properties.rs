use crate::VkBool32;

#[repr(C)]
#[derive(Default)]
pub(crate) struct VkPhysicalDeviceSparseProperties {
    pub(crate) residency_standard_2d_block_shape: VkBool32,
    pub(crate) residency_standard_2d_multisample_block_shape: VkBool32,
    pub(crate) residency_standard_3d_block_shape: VkBool32,
    pub(crate) residency_aligned_mip_size: VkBool32,
    pub(crate) residency_non_resident_strict: VkBool32,
}
