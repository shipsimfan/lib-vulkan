use crate::{
    VkComponentMapping, VkFormat, VkImage, VkImageSubresourceRange, VkImageViewCreateFlags,
    VkImageViewType, VkStructureType,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkImageViewCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkImageViewCreateFlags,
    pub(crate) image: VkImage,
    pub(crate) view_type: VkImageViewType,
    pub(crate) format: VkFormat,
    pub(crate) components: VkComponentMapping,
    pub(crate) subresource_range: VkImageSubresourceRange,
}
