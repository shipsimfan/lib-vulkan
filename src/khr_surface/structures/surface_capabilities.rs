use crate::{
    VkCompositeAlphaFlagsKHR, VkExtent2D, VkImageUsageFlags, VkSurfaceTransformFlagBitsKHR,
    VkSurfaceTransformFlagsKHR,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_surface, VkCompositeAlphaFlagBitsKHR, VkImageUsageFlagBits, VkPresentModeKHR};

/// Structure describing capabilities of a surface
///
/// Provided by [`khr_surface`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceCapabilitiesKHR {
    /// `min_image_count` is the minimum number of images the specified device supports for a
    /// swapchain created for the surface, and will be at least one.
    pub min_image_count: u32,

    /// `max_image_count` is the maximum number of images the specified device supports for a
    /// swapchain created for the surface, and will be either 0, or greater than or equal to
    /// `min_image_count`. A value of 0 means that there is no limit on the number of images,
    /// though there may be limits related to the total amount of memory used by presentable
    /// images.
    pub max_image_count: u32,

    /// `current_extent` is the current width and height of the surface, or the special value
    /// `(0xFFFFFFFF, 0xFFFFFFFF)` indicating that the surface size will be determined by the
    /// extent of a swapchain targeting the surface.
    pub current_extent: VkExtent2D,

    /// `min_image_extent` contains the smallest valid swapchain extent for the surface on the
    /// specified device. The `width` and `height` of the extent will each be less than or equal to
    /// the corresponding `width` and `height` of `current_extent`, unless `current_extent` has the
    /// special value described above.
    pub min_image_extent: VkExtent2D,

    /// `max_image_extent` contains the largest valid swapchain extent for the surface on the
    /// specified device. The `width` and `height` of the extent will each be greater than or equal
    /// to the corresponding `width` and `height` of `min_image_extent`. The `width` and `height`
    /// of the extent will each be greater than or equal to the corresponding `width` and `height`
    /// of `current_extent`, unless `current_extent` has the special value described above.
    pub max_image_extent: VkExtent2D,

    /// `max_image_array_layers` is the maximum number of layers presentable images can have for a
    /// swapchain created for this device and surface, and will be at least one.
    pub max_image_array_layers: u32,

    /// `supported_transforms` is a bitmask of [`VkSurfaceTransformFlagBitsKHR`] indicating the
    /// presentation transforms supported for the surface on the specified device. At least one bit
    /// will be set.
    pub supported_transforms: VkSurfaceTransformFlagsKHR,

    /// `current_transform` is [`VkSurfaceTransformFlagBitsKHR`] value indicating the surface’s
    /// current transform relative to the presentation engine’s natural orientation.
    pub current_transform: VkSurfaceTransformFlagBitsKHR,

    /// `supported_composite_alpha` is a bitmask of [`VkCompositeAlphaFlagBitsKHR`], representing
    /// the alpha compositing modes supported by the presentation engine for the surface on the
    /// specified device, and at least one bit will be set. Opaque composition can be achieved in
    /// any alpha compositing mode by either using an image format that has no alpha component, or
    /// by ensuring that all pixels in the presentable images have an alpha value of 1.0.
    pub supported_composite_alpha: VkCompositeAlphaFlagsKHR,

    /// `supported_usage_flags` is a bitmask of [`VkImageUsageFlagBits`] representing the ways the
    /// application can use the presentable images of a swapchain created with [`VkPresentModeKHR`]
    /// set to [`VkPresentModeKHR::ImmediateModeKHR`], [`VkPresentModeKHR::MailboxKHR`],
    /// [`VkPresentModeKHR::FIFOKHR`] or [`VkPresentModeKHR::FIFORelaxedKHR`] for the surface on
    /// the specified device. [`VkImageUsageFlagBits::ColorAttachmentBit`] must be included in the
    /// set. Implementations may support additional usages.
    pub supported_usage_flags: VkImageUsageFlags,
}

impl Default for VkSurfaceCapabilitiesKHR {
    fn default() -> Self {
        VkSurfaceCapabilitiesKHR {
            min_image_count: 0,
            max_image_count: 0,
            current_extent: VkExtent2D::default(),
            min_image_extent: VkExtent2D::default(),
            max_image_extent: VkExtent2D::default(),
            max_image_array_layers: 0,
            supported_transforms: 0,
            current_transform: VkSurfaceTransformFlagBitsKHR::IdentityBitKHR,
            supported_composite_alpha: 0,
            supported_usage_flags: 0,
        }
    }
}
