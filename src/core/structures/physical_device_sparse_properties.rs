use crate::VkBool32;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0};

/// Structure specifying physical device sparse memory properties
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceSparseProperties {
    /// `residency_standard_2d_block_shape` is [`VK_TRUE`] if the physical device will access all
    /// single-sample 2D sparse resources using the standard sparse image block shapes (based on
    /// image format), as described in the Standard Sparse Image Block Shapes (Single Sample)
    /// table. If this property is not supported the value returned in the `image_granularity`
    /// member of the [`VkSparseImageFormatProperties`] structure for single-sample 2D images is
    /// not required to match the standard sparse image block dimensions listed in the table.
    pub residency_standard_2d_block_shape: VkBool32,

    /// `residency_standard_2d_multisample_block_shape` is [`VK_TRUE`] if the physical device will
    /// access all multisample 2D sparse resources using the standard sparse image block shapes
    /// (based on image format), as described in the Standard Sparse Image Block Shapes (MSAA)
    /// table. If this property is not supported, the value returned in the `image_granularity`
    /// member of the [`VkSparseImageFormatProperties`] structure for multisample 2D images is not
    /// required to match the standard sparse image block dimensions listed in the table.
    pub residency_standard_2d_multisample_block_shape: VkBool32,

    /// `residency_standard_3d_block_shape` is [`VK_TRUE`] if the physical device will access all
    /// 3D sparse resources using the standard sparse image block shapes (based on image format),
    /// as described in the Standard Sparse Image Block Shapes (Single Sample) table. If this
    /// property is not supported, the value returned in the `image_granularity` member of the
    /// [`VkSparseImageFormatProperties`] structure for 3D images is not required to match the
    /// standard sparse image block dimensions listed in the table.
    pub residency_standard_3d_block_shape: VkBool32,

    /// `residency_aligned_mip_size` is [`VK_TRUE`] if images with mip level dimensions that are
    /// not integer multiples of the corresponding dimensions of the sparse image block may be
    /// placed in the mip tail. If this property is not reported, only mip levels with dimensions
    /// smaller than the `image_granularity` member of the [`VkSparseImageFormatProperties`]
    /// structure will be placed in the mip tail. If this property is reported the implementation
    /// is allowed to return [`VkSparseImageFormat::AlignedMipSizeBit`] in the flags member of
    /// [`VkSparseImageFormatProperties`], indicating that mip level dimensions that are not
    /// integer multiples of the corresponding dimensions of the sparse image block will be placed
    /// in the mip tail.
    pub residency_aligned_mip_size: VkBool32,

    /// `residency_non_resident_strict` specifies whether the physical device can consistently
    /// access non-resident regions of a resource. If this property is [`VK_TRUE`], access to
    /// non-resident regions of resources will be guaranteed to return values as if the resource
    /// was populated with 0; writes to non-resident regions will be discarded.
    pub residency_non_resident_strict: VkBool32,
}

impl Default for VkPhysicalDeviceSparseProperties {
    fn default() -> Self {
        VkPhysicalDeviceSparseProperties {
            residency_standard_2d_block_shape: 0,
            residency_standard_2d_multisample_block_shape: 0,
            residency_standard_3d_block_shape: 0,
            residency_aligned_mip_size: 0,
            residency_non_resident_strict: 0,
        }
    }
}
