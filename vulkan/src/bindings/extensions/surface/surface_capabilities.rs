use super::{VkCompositeAlphaFlagsKHR, VkSurfaceTransformFlagBitsKHR, VkSurfaceTransformFlagsKHR};
use crate::{VkExtent2D, VkImageUsageFlags};

#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
    min_image_count: u32,
    max_image_count: u32,
    current_extent: VkExtent2D,
    min_image_extent: VkExtent2D,
    max_image_extent: VkExtent2D,
    max_image_array_layers: u32,
    supported_transforms: VkSurfaceTransformFlagsKHR,
    current_transform: VkSurfaceTransformFlagBitsKHR,
    supported_composite_alpha: VkCompositeAlphaFlagsKHR,
    supported_usage_flags: VkImageUsageFlags,
}

impl VkSurfaceCapabilitiesKHR {
    pub(crate) const fn null() -> Self {
        VkSurfaceCapabilitiesKHR {
            min_image_count: 0,
            max_image_count: 0,
            current_extent: VkExtent2D::new(0, 0),
            min_image_extent: VkExtent2D::new(0, 0),
            max_image_extent: VkExtent2D::new(0, 0),
            max_image_array_layers: 0,
            supported_transforms: VkSurfaceTransformFlagsKHR::new(&[]),
            current_transform: VkSurfaceTransformFlagBitsKHR::Identity,
            supported_composite_alpha: VkCompositeAlphaFlagsKHR::new(&[]),
            supported_usage_flags: VkImageUsageFlags::new(&[]),
        }
    }

    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }

    pub fn max_image_count(&self) -> u32 {
        self.max_image_count
    }

    pub fn current_extent(&self) -> &VkExtent2D {
        &self.current_extent
    }

    pub fn min_image_extent(&self) -> &VkExtent2D {
        &self.min_image_extent
    }

    pub fn max_image_extent(&self) -> &VkExtent2D {
        &self.max_image_extent
    }

    pub fn max_image_array_layers(&self) -> u32 {
        self.max_image_array_layers
    }

    pub fn supported_transforms(&self) -> VkSurfaceTransformFlagsKHR {
        self.supported_transforms
    }

    pub fn current_transform(&self) -> VkSurfaceTransformFlagBitsKHR {
        self.current_transform
    }

    pub fn supported_composite_alpha(&self) -> VkCompositeAlphaFlagsKHR {
        self.supported_composite_alpha
    }

    pub fn supported_usage_flags(&self) -> VkImageUsageFlags {
        self.supported_usage_flags
    }
}
