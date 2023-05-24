use super::{VkSwapchainCreateFlagsKHR, VkSwapchainKHR};
use crate::{
    bindings::{VkBool32, VkStructureType, VkSurfaceKHR},
    Loader, NativeLoader, VkColorSpaceKHR, VkCompositeAlphaFlagBitsKHR, VkExtent2D, VkFormat,
    VkImageUsageFlags, VkPresentModeKHR, VkSharingMode, VkSurfaceTransformFlagBitsKHR,
};
use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

#[repr(C)]
pub struct VkSwapchainCreateInfoKHR<'a, L: Loader = NativeLoader> {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    flags: VkSwapchainCreateFlagsKHR,
    surface: VkSurfaceKHR,
    min_image_count: u32,
    image_format: VkFormat,
    image_color_space: VkColorSpaceKHR,
    image_extent: VkExtent2D,
    image_array_layers: u32,
    image_usage: VkImageUsageFlags,
    image_sharing_mode: VkSharingMode,
    queue_family_index_count: u32,
    p_queue_family_indices: Option<NonNull<u32>>,
    pre_transform: VkSurfaceTransformFlagBitsKHR,
    composite_alphas: VkCompositeAlphaFlagBitsKHR,
    present_mode: VkPresentModeKHR,
    clipped: VkBool32,
    old_swapchain: Option<VkSwapchainKHR>,
    phantom: PhantomData<&'a L>,
}

impl<'a, L: Loader> VkSwapchainCreateInfoKHR<'a, L> {
    pub fn new(
        flags: VkSwapchainCreateFlagsKHR,
        surface: &crate::VkSurfaceKHR<L>,
        min_image_count: u32,
        image_format: VkFormat,
        image_color_space: VkColorSpaceKHR,
        image_extent: VkExtent2D,
        image_array_layers: u32,
        image_usage: VkImageUsageFlags,
        image_sharing_mode: VkSharingMode,
        queue_family_indices: &'a [u32],
        pre_transform: VkSurfaceTransformFlagBitsKHR,
        composite_alphas: VkCompositeAlphaFlagBitsKHR,
        present_mode: VkPresentModeKHR,
        clipped: bool,
        old_swapchain: Option<&crate::VkSwapchainKHR>,
    ) -> Self {
        VkSwapchainCreateInfoKHR {
            s_type: VkStructureType::SwapchainCreateInfo,
            p_next: None,
            flags,
            surface: surface.inner(),
            min_image_count,
            image_format,
            image_color_space,
            image_extent,
            image_array_layers,
            image_usage,
            image_sharing_mode,
            queue_family_index_count: queue_family_indices.len() as u32,
            p_queue_family_indices: if queue_family_indices.len() == 0 {
                None
            } else {
                Some(unsafe { NonNull::new_unchecked(queue_family_indices.as_ptr() as _) })
            },
            pre_transform,
            composite_alphas,
            present_mode,
            clipped: clipped as VkBool32,
            old_swapchain: old_swapchain.map(|old_swapchain| old_swapchain.inner()),
            phantom: PhantomData,
        }
    }
}
