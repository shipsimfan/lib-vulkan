use crate::{VkAllocationCallbacks, VkDevice, VkImageView, VkImageViewCreateInfo, VkResult};

pub(crate) type VkCreateImageView = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkImageViewCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_view: *mut Option<VkImageView>,
) -> VkResult;

pub(crate) type VkDestroyImageView = extern "system" fn(
    device: VkDevice,
    image_view: VkImageView,
    p_allocator: *const VkAllocationCallbacks,
);
