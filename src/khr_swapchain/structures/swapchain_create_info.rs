use crate::{
    VkBool32, VkColorSpaceKHR, VkCompositeAlphaFlagsKHR, VkExtent2D, VkFormat, VkImageUsageFlags,
    VkPresentModeKHR, VkSharingMode, VkStructureType, VkSurfaceKHR, VkSurfaceTransformFlagsKHR,
    VkSwapchainCreateFlagsKHR, VkSwapchainKHR,
};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    khr_swapchain, VkCompositeAlphaFlagBitsKHR, VkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    VkImageUsageFlagBits, VkSurfaceTransformFlagBitsKHR, VkSwapchainCreateFlagBitsKHR, VK_FALSE,
    VK_NULL_HANDLE, VK_TRUE,
};

/// Structure specifying parameters of a newly created swapchain object
///
/// Upon calling [`VkCreateSwapchainKHR`] with an `old_swapchain` that is not [`VK_NULL_HANDLE`],
/// `old_swapchain` is retired — even if creation of the new swapchain fails. The new swapchain is
/// created in the non-retired state whether or not `old_swapchain` is [`VK_NULL_HANDLE`].
///
/// Upon calling [`VkCreateSwapchainKHR`] with an `old_swapchain` that is not [`VK_NULL_HANDLE`],
/// any images from `old_swapchain` that are not acquired by the application may be freed by the
/// implementation, which may occur even if creation of the new swapchain fails. The application
/// can destroy `old_swapchain` to free all memory associated with `old_swapchain`.
///
/// Provided by [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSwapchainCreateInfoKHR {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkSwapchainCreateFlagBitsKHR`] indicating parameters of the
    /// swapchain creation.
    pub flags: VkSwapchainCreateFlagsKHR,

    /// `surface` is the surface onto which the swapchain will present images. If the creation
    /// succeeds, the swapchain becomes associated with `surface`.
    pub surface: VkSurfaceKHR,

    /// `min_image_count` is the minimum number of presentable images that the application needs.
    /// The implementation will either create the swapchain with at least that many images, or it
    /// will fail to create the swapchain.
    pub min_image_count: u32,

    /// `image_format` is a [`VkFormat`] value specifying the format the swapchain image(s) will be
    /// created with.
    pub image_format: VkFormat,

    /// `image_color_space` is a [`VkColorSpaceKHR`] value specifying the way the swapchain
    /// interprets image data.
    pub image_color_space: VkColorSpaceKHR,

    /// `image_extent` is the size (in pixels) of the swapchain image(s). The behavior is
    /// platform-dependent if the image extent does not match the surface’s `current_extent` as
    /// returned by [`VkGetPhysicalDeviceSurfaceCapabilitiesKHR`].
    pub image_extent: VkExtent2D,

    /// `image_array_layers` is the number of views in a multiview/stereo surface. For
    /// non-stereoscopic-3D applications, this value is 1.
    pub image_array_layers: u32,

    /// `image_usage` is a bitmask of [`VkImageUsageFlagBits`] describing the intended usage of the
    /// (acquired) swapchain images.
    pub image_usage: VkImageUsageFlags,

    /// `image_sharing_mode` is the sharing mode used for the image(s) of the swapchain.
    pub image_sharing_mode: VkSharingMode,

    /// `queue_family_index_count` is the number of queue families having access to the image(s) of
    /// the swapchain when `image_sharing_mode` is [`VkSharingMode::Concurrent`].
    pub queue_family_index_count: u32,

    /// `queue_family_indices` is a pointer to an array of queue family indices having access to
    /// the images(s) of the swapchain when `image_sharing_mode` is [`VkSharingMode::Concurrent`].
    pub queue_family_indices: *const u32,

    /// `pre_transform` is a [`VkSurfaceTransformFlagBitsKHR`] value describing the transform,
    /// relative to the presentation engine’s natural orientation, applied to the image content
    /// prior to presentation. If it does not match the `current_transform` value returned by
    /// [`VkGetPhysicalDeviceSurfaceCapabilitiesKHR`], the presentation engine will transform the
    /// image content as part of the presentation operation.
    pub pre_transform: VkSurfaceTransformFlagsKHR,

    /// `composite_alpha` is a [`VkCompositeAlphaFlagBitsKHR`] value indicating the alpha
    /// compositing mode to use when this surface is composited together with other surfaces on
    /// certain window systems.
    pub composite_alpha: VkCompositeAlphaFlagsKHR,

    /// `present_mode` is the presentation mode the swapchain will use. A swapchain’s present mode
    /// determines how incoming present requests will be processed and queued internally.
    pub present_mode: VkPresentModeKHR,

    /// `clipped` specifies whether the Vulkan implementation is allowed to discard rendering
    /// operations that affect regions of the surface that are not visible.
    ///  - If set to [`VK_TRUE`], the presentable images associated with the swapchain may not own
    ///    all of their pixels. Pixels in the presentable images that correspond to regions of the
    ///    target surface obscured by another window on the desktop, or subject to some other
    ///    clipping mechanism will have undefined content when read back. Fragment shaders may not
    ///    execute for these pixels, and thus any side effects they would have had will not occur.
    ///    Setting [`VK_TRUE`] does not guarantee any clipping will occur, but allows more
    ///    efficient presentation methods to be used on some platforms.
    ///  - If set to [`VK_FALSE`], presentable images associated with the swapchain will own all of
    ///    the pixels they contain.
    pub clipped: VkBool32,

    /// `old_swapchain` is [`VK_NULL_HANDLE`], or the existing non-retired swapchain currently
    /// associated with surface. Providing a valid `old_swapchain` may aid in the resource reuse,
    /// and also allows the application to still present any images that are already acquired from
    /// it.
    pub old_swapchain: VkSwapchainKHR,
}

impl Default for VkSwapchainCreateInfoKHR {
    fn default() -> Self {
        VkSwapchainCreateInfoKHR {
            r#type: VkStructureType::SwapchainCreateInfoKHR,
            next: null(),
            flags: 0,
            surface: null_mut(),
            min_image_count: 0,
            image_format: VkFormat::Undefined,
            image_color_space: VkColorSpaceKHR::SRGBNonlinearKHR,
            image_extent: VkExtent2D::default(),
            image_array_layers: 0,
            image_usage: 0,
            image_sharing_mode: VkSharingMode::Exclusive,
            queue_family_index_count: 0,
            queue_family_indices: null(),
            pre_transform: 0,
            composite_alpha: 0,
            present_mode: VkPresentModeKHR::FIFOKHR,
            clipped: 0,
            old_swapchain: null_mut(),
        }
    }
}
