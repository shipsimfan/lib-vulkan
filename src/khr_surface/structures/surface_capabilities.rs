use crate::{
    VkExtent2D, VkImageUsageFlags,
    khr_surface::{
        VkCompositeAlphaFlagsKhr, VkSurfaceTransformFlagKhr, VkSurfaceTransformFlagsKhr,
    },
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VkImageUsageFlag,
    khr_surface::{self, VkCompositeAlphaFlagKhr, VkPresentModeKhr},
};

/// Structure describing capabilities of a surface
///
/// Provided by [`khr_surface`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSurfaceCapabilitiesKhr {
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

    /// `supported_transforms` is a bitmask of [`VkSurfaceTransformFlagKhr`] indicating the
    /// presentation transforms supported for the surface on the specified device. At least one bit
    /// will be set.
    pub supported_transforms: VkSurfaceTransformFlagsKhr,

    /// `current_transform` is [`VkSurfaceTransformFlagKhr`] value indicating the surface’s
    /// current transform relative to the presentation engine’s natural orientation.
    pub current_transform: VkSurfaceTransformFlagKhr,

    /// `supported_composite_alpha` is a bitmask of [`VkCompositeAlphaFlagKhr`], representing
    /// the alpha compositing modes supported by the presentation engine for the surface on the
    /// specified device, and at least one bit will be set. Opaque composition can be achieved in
    /// any alpha compositing mode by either using an image format that has no alpha component, or
    /// by ensuring that all pixels in the presentable images have an alpha value of 1.0.
    pub supported_composite_alpha: VkCompositeAlphaFlagsKhr,

    /// `supported_usage_flags` is a bitmask of [`VkImageUsageFlag`] representing the ways the
    /// application can use the presentable images of a swapchain created with [`VkPresentModeKhr`]
    /// set to [`VkPresentModeKhr::ImmediateModeKhr`], [`VkPresentModeKhr::MailboxKhr`],
    /// [`VkPresentModeKhr::FIFOKhr`] or [`VkPresentModeKhr::FIFORelaxedKhr`] for the surface on
    /// the specified device. [`VkImageUsageFlag::ColorAttachmentBit`] must be included in the
    /// set. Implementations may support additional usages.
    pub supported_usage_flags: VkImageUsageFlags,
}

impl Default for VkSurfaceCapabilitiesKhr {
    fn default() -> Self {
        VkSurfaceCapabilitiesKhr {
            min_image_count: 0,
            max_image_count: 0,
            current_extent: VkExtent2D::default(),
            min_image_extent: VkExtent2D::default(),
            max_image_extent: VkExtent2D::default(),
            max_image_array_layers: 0,
            supported_transforms: VkSurfaceTransformFlagsKhr::new(),
            current_transform: VkSurfaceTransformFlagKhr::IdentityBitKhr,
            supported_composite_alpha: VkCompositeAlphaFlagsKhr::new(),
            supported_usage_flags: VkImageUsageFlags::new(),
        }
    }
}
