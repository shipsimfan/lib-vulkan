use crate::{VkAllocationCallbacks, VkDevice, VkResult, VkSwapchainCreateInfoKHR, VkSwapchainKHR};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VkSampleCountFlagBits, VkSurfaceKHR, VK_NULL_HANDLE};

/// Create a swapchain
///
/// # Parameters
///  - `device` is the device to create the swapchain for.
///  - `create_info` is a pointer to a [`VkSwapchainCreateInfoKHR`] structure specifying the
///    parameters of the created swapchain.
///  - `allocator` is the allocator used for host memory allocated for the swapchain object when
///    there is no more specific allocator available.
///  - `swapchain` is a pointer to a [`VkSwapchainKHR`] handle in which the created swapchain
///    object will be returned.
///
/// # Description
/// As mentioned above, if [`VkCreateSwapchainKHR`] succeeds, it will return a handle to a
/// swapchain containing an array of at least `create_info->min_image_count` presentable images.
///
/// While acquired by the application, presentable images can be used in any way that equivalent
/// non-presentable images can be used. A presentable image is equivalent to a non-presentable
/// image created with the following [`VkImageCreateInfo`] parameters:
///  * `flags` - [`VkImageCreateFlagBits::SplitInstanceBindRegionsBit`] is set if
///              [`VkSwapchainCreateFlagBitsKHR::SplitInstanceBindRegionsBitKhr`] is set.
///              [`VkImageCreateFlagBits::ProtectedBit`] is set if
///              [`VkSwapchainCreateFlagBitsKHR::ProtectedBitKhr`] is set.
///              [`VkImageCreateFlagBits::MutableFormatBit`] and
///              [`VkImageCreateFlagBits::ExtendedUsageBitKhr`] are both set if
///              [`VkSwapchainCreateFlagBitsKHR::MutableFormatBitKhr`] isset. All other bits are
///              unset.
///  * `image_type` - [`VkImageType::_2D`]
///  * `format` - `create_info.image_format`
///  * `extent` -  `{ create_info.image_extent.width, create_info.image_extent.height, 1 }`
///  * `mip_levels` - 1
///  * `array_layers` - `create_info.image_array_layers`
///  * `samples` - [`VkSampleCountFlagBits::_1Bit`]
///  * `tiling` - [`VkImageTiling::Optimal`]
///  * `usage` - `create_info.image_usage`
///  * `sharing_mode` - `create_info.image_sharing_mode`
///  * `queue_family_index_count` - `create_info.queue_family_index_count`
///  * pQueueFamilyIndices - `create_info.queue_family_indices`
///  * initialLayout - [`VkImageLayout::Undefined`]
///
/// The `create_info.surface` must not be destroyed until after the swapchain is destroyed.
///
/// If `old_swapchain` is [`VK_NULL_HANDLE`], and the native window referred to by
/// `create_info->surface` is already associated with a Vulkan swapchain,
/// [`VkResult::VkErrorNativeWindowInUseKHR`] must be returned.
///
/// If the native window referred to by `create_info.surface` is already associated with a
/// non-Vulkan graphics API surface, [`VkResult::VkErrorNativeWindowInUseKHR`] must be returned.
///
/// The native window referred to by `create_info->surface` must not become associated with a
/// non-Vulkan graphics API surface before all associated Vulkan swapchains have been destroyed.
///
/// [`VkCreateSwapchainKHR`] will return [`VkResult::VkErrorDeviceLost`] if the logical device was
/// lost. The [`VkSwapchainKHR`] is a child of the device, and must be destroyed before the device.
/// However, [`VkSurfaceKHR`] is not a child of any [`VkDevice`] and is not affected by the lost
/// device. After successfully recreating a [`VkDevice`], the same [`VkSurfaceKHR`] can be used to
/// create a new [`VkSwapchainKHR`], provided the previous one was destroyed.
///
/// If the `old_swapchain` parameter of `create_info` is a valid swapchain, which has exclusive
/// full-screen access, that access is released from `create_info.old_swapchain`. If the command
/// succeeds in this case, the newly created swapchain will automatically acquire exclusive
/// full-screen access from `create_info.old_swapchain`.
///
/// In some cases, swapchain creation may fail if exclusive full-screen mode is requested for
/// application control, but for some implementation-specific reason exclusive full-screen access
/// is unavailable for the particular combination of parameters provided. If this occurs,
/// [`VkResult::VkErrorInitializationFailed`] will be returned.
///
/// If the `next` chain of [`VkSwapchainCreateInfoKHR`] includes a
/// [`VkSwapchainPresentBarrierCreateInfoNV`] structure, then that structure includes additional
/// swapchain creation parameters specific to the present barrier. Swapchain creation may fail if
/// the state of the current system restricts the usage of the present barrier feature
/// [`VkSurfaceCapabilitiesPresentBarrierNV`], or a swapchain itself does not satisfy all the
/// required conditions. In this scenario [`VkResult::VkErrorInitializationFailed`] is returned.
///
/// When the [`VkSurfaceKHR`] in [`VkSwapchainCreateInfoKHR`] is a display surface, then the
/// [`VkDisplayModeKHR`] in display surface’s [`VkDisplaySurfaceCreateInfoKHR`] is associated with
/// a particular [`VkDisplayKHR`]. Swapchain creation may fail if that [`VkDisplayKHR`] is not
/// acquired by the application. In this scenario [`VkResult::VkErrorInitializationFailed`] is
/// returned.
///
/// Provided by [`khr_swapchain`]
pub type VkCreateSwapchainKHR = extern "system" fn(
    device: VkDevice,
    create_info: *const VkSwapchainCreateInfoKHR,
    allocator: *const VkAllocationCallbacks,
    swapchain: *mut VkSwapchainKHR,
) -> VkResult;

/// The name of [`VkCreateSwapchainKHR`]
pub const VK_CREATE_SWAPCHAIN_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkCreateSwapchainKHR\0") };
