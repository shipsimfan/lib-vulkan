use crate::{
    VkBool32, VkExtent2D, VkFormat, VkImageUsageFlags, VkSharingMode, VkStructureType,
    khr_surface::{
        VkColorSpaceKhr, VkCompositeAlphaFlagsKhr, VkPresentModeKhr, VkSurfaceKhr,
        VkSurfaceTransformFlagsKhr,
    },
    khr_swapchain::{VkSwapchainCreateFlagsKhr, VkSwapchainKhr},
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_NULL_HANDLE, VK_TRUE, VkImageUsageFlag, VkInstance,
    khr_surface::{
        VkCompositeAlphaFlagKhr, VkGetPhysicalDeviceSurfaceCapabilitiesKhr,
        VkGetPhysicalDeviceSurfaceFormatsKhr, VkSurfaceCapabilitiesKhr, VkSurfaceFormatKhr,
        VkSurfaceTransformFlagKhr,
    },
    khr_swapchain::{self, VkCreateSwapchainKhr, VkSwapchainCreateFlagKhr},
};

/// Structure specifying parameters of a newly created swapchain object
///
/// # Description
/// Upon calling [`VkCreateSwapchainKhr`] with an `old_swapchain` that is not [`VK_NULL_HANDLE`],
/// `old_swapchain` is retired — even if creation of the new swapchain fails. The new swapchain is
/// created in the non-retired state whether or not `old_swapchain` is [`VK_NULL_HANDLE`].
///
/// Upon calling [`VkCreateSwapchainKhr`] with an `old_swapchain` that is not [`VK_NULL_HANDLE`],
/// any images from `old_swapchain` that are not acquired by the application may be freed by the
/// implementation, which may occur even if creation of the new swapchain fails. The application
/// can destroy `old_swapchain` to free all memory associated with `old_swapchain`.
///
/// # Valid Usage
///  - The implied image creation parameters of the swapchain must be supported as reported by
///    [`VkGetPhysicalDeviceImageFormatProperties`]
///
/// Provided by [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSwapchainCreateInfoKhr {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::SwapchainCreateInfoKhr`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `swapchain_maintenance1` feature is not enabled, then the `next` chain must not
    ///    include a [`VkSwapchainPresentModesCreateInfoKhr`] structure
    ///  - If `flags` contains [`VkSwapchainCreateFlagKhr::MutableFormatBitKhr`] then the `next`
    ///    chain must include a [`VkImageFormatListCreateInfo`] structure with a
    ///    `view_format_count` greater than zero and `view_formats` must have an element equal to
    ///    `image_format`
    ///  - If a [`VkImageFormatListCreateInfo`] structure was included in the `next` chain and
    ///    [`VkImageFormatListCreateInfo::view_format_count`] is not zero then all of the formats
    ///    in [`VkImageFormatListCreateInfo::view_formats`] must be compatible with the format as
    ///    described in the compatibility table
    ///  - If `flags` does not contain [`VkSwapchainCreateFlagKhr::MutableFormatBitKhr`] and the
    ///    `next` chain include a [`VkImageFormatListCreateInfo`] structure then
    ///    [`VkImageFormatListCreateInfo::view_format_count`] must be 0 or 1
    ///  - If the `next` chain includes a [`VkSurfaceFullScreenExclusiveInfoExt`] structure with
    ///    its `fullscreen_exclusive` member set to
    ///    [`VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`], and surface was created using
    ///    [`VkCreateWin32SurfaceKhr`], a [`VkSurfaceFullScreenExclusiveWin32InfoExt`] structure
    ///    must be included in the `next` chain
    ///  - If the [`image_compression_control_swapchain`] feature is not enabled, the `next` chain
    ///    must not include an [`VkImageCompressionControlExt`] structure
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkDeviceGroupSwapchainCreateInfoKhr`], [`VkImageCompressionControlExt`],
    ///    [`VkImageFormatListCreateInfo`], [`VkSurfaceFullScreenExclusiveInfoExt`],
    ///    [`VkSurfaceFullScreenExclusiveWin32InfoExt`], [`VkSwapchainCounterCreateInfoExt`],
    ///    [`VkSwapchainDisplayNativeHdrCreateInfoAmd`], [`VkSwapchainLatencyCreateInfoNv`],
    ///    [`VkSwapchainPresentBarrierCreateInfoNv`], [`VkSwapchainPresentModesCreateInfoKhr`], or
    ///    [`VkSwapchainPresentScalingCreateInfoKhr`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkSwapchainCreateFlagKhr`] indicating parameters of the
    /// swapchain creation.
    ///
    /// # Valid Usage
    ///  - If the `swapchain_maintenance1` feature is not enabled, then `flags` must not include
    ///    [`VkSwapchainCreateFlagKhr::DeferredMemoryAllocationBitExt`]
    ///  - If the logical device was created with
    ///    [`VkDeviceGroupDeviceCreateInfo::physical_device_count`] equal to 1, `flags` must not
    ///    contain [`VkSwapchainCreateFlagKhr::SplitInstanceBindRegionsBitKhr`]
    ///  - If `flags` contains [`VkSwapchainCreateFlagKhr::ProtectedBitKhr`], then
    ///    [`VkSurfaceProtectedCapabilitiesKhr::supports_protected`] must be [`VK_TRUE`] in the
    ///    [`VkSurfaceProtectedCapabilitiesKhr`] structure returned by
    ///    [`VkGetPhysicalDeviceSurfaceCapabilities2Khr`] for `surface`
    ///  - If none of the `present_timing`, `present_at_absolute_time`, or
    ///    [`present_at_relative_time`] features are enabled, `flags` must not contain
    ///    [`VkSwapchainCreateFlagKhr::PresentTimingBitExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkSwapchainCreateFlagKhr`] values
    pub flags: VkSwapchainCreateFlagsKhr,

    /// `surface` is the surface onto which the swapchain will present images. If the creation
    /// succeeds, the swapchain becomes associated with `surface`.
    ///
    /// # Valid Usage
    ///  - `surface` must be a surface that is supported by the device as determined using
    ///    [`VkGetPhysicalDeviceSurfaceSupportKhr`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `surface` must be a valid [`VkSurfaceKhr`] handle
    ///  - Both of `old_swapchain`, and `surface` that are valid handles of non-ignored parameters
    ///    must have been created, allocated, or retrieved from the same [`VkInstance`]
    ///
    /// # Host Synchronization
    ///  - Host access to `surface` must be externally synchronized
    pub surface: VkSurfaceKhr,

    /// `min_image_count` is the minimum number of presentable images that the application needs.
    /// The implementation will either create the swapchain with at least that many images, or it
    /// will fail to create the swapchain.
    ///
    /// # Valid Usage
    ///  - `min_image_count` must be less than or equal to the value returned in the
    ///    `max_image_count` member of the [`VkSurfaceCapabilitiesKhr`] structure returned by
    ///    [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`] for the surface if the returned
    ///    `max_image_count` is not zero
    ///  - If `present_mode` is not [`VkPresentModeKhr::SharedDemandRefreshKhr`] nor
    ///    [`VkPresentModeKhr::SharedContinuousRefreshKhr`], then `min_image_count` must be
    ///    greater than or equal to the value returned in the `min_image_count` member of the
    ///    [`VkSurfaceCapabilitiesKhr`] structure returned by
    ///    [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`] for the surface
    ///  - `min_image_count` must be 1 if `present_mode` is either
    ///    [`VkPresentModeKhr::SharedDemandRefreshKhr`] or
    ///    [`VkPresentModeKhr::SharedContinuousRefreshKhr`]
    pub min_image_count: u32,

    /// `image_format` is a [`VkFormat`] value specifying the format the swapchain image(s) will be
    /// created with.
    ///
    /// # Valid Usage
    ///  - `image_format` and `image_color_space` must match the `format` and `color_space`
    ///    members, respectively, of one of the [`VkSurfaceFormatKhr`] structures returned by
    ///    [`VkGetPhysicalDeviceSurfaceFormatsKhr`] for the surface
    ///
    /// # Valid Usage (Implicit)
    ///  - `image_format` must be a valid [`VkFormat`] value
    pub image_format: VkFormat,

    /// `image_color_space` is a [`VkColorSpaceKhr`] value specifying the way the swapchain
    /// interprets image data.
    ///
    /// # Valid Usage
    ///  - `image_format` and `image_color_space` must match the `format` and `color_space`
    ///    members, respectively, of one of the [`VkSurfaceFormatKhr`] structures returned by
    ///    [`VkGetPhysicalDeviceSurfaceFormatsKhr`] for the surface
    ///
    /// # Valid Usage (Implicit)
    ///  - `image_color_space` must be a valid [`VkColorSpaceKhr`] value
    pub image_color_space: VkColorSpaceKhr,

    /// `image_extent` is the size (in pixels) of the swapchain image(s). The behavior is
    /// platform-dependent if the image extent does not match the surface’s `current_extent` as
    /// returned by [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`].
    ///
    /// # Valid Usage
    ///  - If a [`VkSwapchainPresentScalingCreateInfoKhr`] structure was not included in the `next`
    ///    chain, or it is included and
    ///    [`VkSwapchainPresentScalingCreateInfoKhr::scaling_behavior`] is zero then `image_extent`
    ///    must be between `min_image_extent` and `max_image_extent`, inclusive, where
    ///    `min_image_extent` and `max_image_extent` are members of the
    ///    [`VkSurfaceCapabilitiesKhr`] structure returned by
    ///    [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`] for the surface
    ///  - If a [`VkSwapchainPresentScalingCreateInfoKhr`] structure was included in the `next`
    ///    chain and [`VkSwapchainPresentScalingCreateInfoKhr::scaling_behavior`] is not zero then
    ///    `image_extent` must be between `min_scaled_image_extent` and `max_scaled_image_extent`,
    ///    inclusive, where `min_scaled_image_extent` and `max_scaled_image_extent` are members of
    ///    the [`VkSurfacePresentScalingCapabilitiesKhr`] structure returned by
    ///    [`VkGetPhysicalDeviceSurfaceCapabilities2Khr`] for the surface and `present_mode`
    ///  - `image_extent` members `width` and `height` must both be non-zero
    pub image_extent: VkExtent2D,

    /// `image_array_layers` is the number of views in a multiview/stereo surface. For
    /// non-stereoscopic-3D applications, this value is 1.
    ///
    /// # Valid Usage
    ///  - `image_array_layers` must be greater than 0 and less than or equal to the
    ///    `max_image_array_layers` member of the [`VkSurfaceCapabilitiesKhr`] structure returned
    ///    by [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`] for the surface
    pub image_array_layers: u32,

    /// `image_usage` is a bitmask of [`VkImageUsageFlag`] describing the intended usage of the
    /// (acquired) swapchain images.
    ///
    /// # Valid Usage
    ///  - If `present_mode` is [`VkPresentMode::FifoLatestReadyKhr`],
    ///    [`VkPresentMode::ImmediateKhr`], [`VkPresentMode::MailboxKhr`],
    ///    [`VkPresentMode::FifoKhr`] or [`VkPresentMode::FifoRelaxedKhr`], `image_usage` must be
    ///    a subset of the supported usage flags present in the `supported_usage_flags` member of
    ///    the [`VkSurfaceCapabilitiesKhr`] structure returned by
    ///    [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`] for `surface`
    ///  - If `present_mode` is [`VkPresentMode::SharedDemandRefreshKhr`] or
    ///    [`VkPresentMode::SharedContinuousREFRESH_KHR`], `image_usage` must be a subset of the
    ///    supported usage flags present in the `shared_present_supported_usage_flags` member of
    ///    the [`VkSharedPresentSurfaceCapabilitiesKhr`] structure returned by
    ///    [`VkGetPhysicalDeviceSurfaceCapabilities2Khr`] for `surface`
    ///
    /// # Valid Usage (Implicit)
    ///  - `image_usage` must be a valid combination of [`VkImageUsageFlag`] values
    ///  - `image_usage` must not be 0
    pub image_usage: VkImageUsageFlags,

    /// `image_sharing_mode` is the sharing mode used for the image(s) of the swapchain.
    ///
    /// # Valid Usage (Implicit)
    ///  - `image_sharing_mode` must be a valid [`VkSharingMode`] value
    pub image_sharing_mode: VkSharingMode,

    /// `queue_family_index_count` is the number of queue families having access to the image(s) of
    /// the swapchain when `image_sharing_mode` is [`VkSharingMode::Concurrent`].
    ///
    /// # Valid Usage
    ///  - If `image_sharing_mode` is [`VkSharingMode::Concurrent`], `queue_family_index_count`
    ///    must be greater than 1
    pub queue_family_index_count: u32,

    /// `queue_family_indices` is a pointer to an array of queue family indices having access to
    /// the images(s) of the swapchain when `image_sharing_mode` is [`VkSharingMode::Concurrent`].
    ///
    /// # Valid Usage
    ///  - If `image_sharing_mode` is [`VkSharingMode::Concurrent`], `queue_family_indices` must be
    ///    a valid pointer to an array of `queue_family_index_count` [`u32`] values
    ///  - If `image_sharing_mode` is [`VkSharingMode::Concurrent`], each element of
    ///    `queue_family_indices` must be unique and must be less than
    ///    `queue_family_property_count` returned by either
    ///    [`VkGetPhysicalDeviceQueueFamilyProperties`] or
    ///    [`VkGetPhysicalDeviceQueueFamilyProperties2`] for the `physical_device` that was used to
    ///    create device
    pub queue_family_indices: *const u32,

    /// `pre_transform` is a [`VkSurfaceTransformFlagKhr`] value describing the transform,
    /// relative to the presentation engine’s natural orientation, applied to the image content
    /// prior to presentation. If it does not match the `current_transform` value returned by
    /// [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`], the presentation engine will transform the
    /// image content as part of the presentation operation.
    ///
    /// # Valid Usage
    ///  - `pre_transform` must be one of the bits present in the `supported_transforms` member of
    ///    the [`VkSurfaceCapabilitiesKhr`] structure returned by
    ///    [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`] for the surface
    ///
    /// # Valid Usage (Implicit)
    ///  - `pre_transform` must be a valid [`VkSurfaceTransformFlagKhr`] value
    pub pre_transform: VkSurfaceTransformFlagsKhr,

    /// `composite_alpha` is a [`VkCompositeAlphaFlagKhr`] value indicating the alpha
    /// compositing mode to use when this surface is composited together with other surfaces on
    /// certain window systems.
    ///
    /// # Valid Usage
    ///  - `composite_alpha` must be one of the bits present in the `supported_composite_alpha`
    ///    member of the [`VkSurfaceCapabilitiesKhr`] structure returned by
    ///    [`VkGetPhysicalDeviceSurfaceCapabilitiesKhr`] for the surface
    ///
    /// # Valid Usage (Implicit)
    ///  - `composite_alpha` must be a valid [`VkCompositeAlphaFlagKhr`] value
    pub composite_alpha: VkCompositeAlphaFlagsKhr,

    /// `present_mode` is the presentation mode the swapchain will use. A swapchain’s present mode
    /// determines how incoming present requests will be processed and queued internally.
    ///
    /// # Valid Usage
    ///  - `present_mode` must be one of the [`VkPresentModeKhr`] values returned by
    ///    [`VkGetPhysicalDeviceSurfacePresentModesKhr`] for the surface
    ///  - If the `present_mode_fifo_latest_ready` feature is not enabled, `present_mode` must not
    ///    be [`VkPResentMode::FifoLatestReadyKhr`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `present_mode` must be a valid [`VkPresentModeKhr`] value
    pub present_mode: VkPresentModeKhr,

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
    ///
    /// # Valid Usage
    ///  - If `old_swapchain` is not [`VK_NULL_HANDLE`], `old_swapchain` must be a non-retired
    ///    swapchain associated with native window referred to by surface
    ///
    /// # Valid Usage (Implicit)
    ///  - If `old_swapchain` is not [`VK_NULL_HANDLE`], `old_swapchain` must be a valid
    ///    [`VkSwapchainKhr`] handle
    ///  - Both of `old_swapchain`, and `surface` that are valid handles of non-ignored parameters
    ///    must have been created, allocated, or retrieved from the same [`VkInstance`]
    ///
    /// # Host Synchronization
    ///  - Host access to `old_swapchain` must be externally synchronized
    pub old_swapchain: VkSwapchainKhr,
}

impl const Default for VkSwapchainCreateInfoKhr {
    fn default() -> Self {
        VkSwapchainCreateInfoKhr {
            r#type: VkStructureType::SwapchainCreateInfoKhr,
            next: null(),
            flags: VkSwapchainCreateFlagsKhr::new(),
            surface: VkSurfaceKhr::null(),
            min_image_count: 0,
            image_format: VkFormat::Undefined,
            image_color_space: VkColorSpaceKhr::SRGBNonlinearKhr,
            image_extent: VkExtent2D::default(),
            image_array_layers: 0,
            image_usage: VkImageUsageFlags::new(),
            image_sharing_mode: VkSharingMode::Exclusive,
            queue_family_index_count: 0,
            queue_family_indices: null(),
            pre_transform: VkSurfaceTransformFlagsKhr::new(),
            composite_alpha: VkCompositeAlphaFlagsKhr::new(),
            present_mode: VkPresentModeKhr::FIFOKhr,
            clipped: 0,
            old_swapchain: VkSwapchainKhr::null(),
        }
    }
}
