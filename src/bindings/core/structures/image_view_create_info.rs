use super::{VkImageSubresourceRange, VkImageViewCreateFlags};
use crate::{
    bindings::{VkImage, VkStructureType},
    VkComponentMapping, VkFormat, VkImageViewType,
};
use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

#[repr(C)]
pub struct VkImageViewCreateInfo<'a> {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    flags: VkImageViewCreateFlags,
    image: VkImage,
    view_type: VkImageViewType,
    format: VkFormat,
    components: VkComponentMapping,
    subresource_range: VkImageSubresourceRange,
    phantom: PhantomData<&'a ()>,
}

impl<'a> VkImageViewCreateInfo<'a> {
    pub fn new(
        flags: VkImageViewCreateFlags,
        image: &'a crate::VkImage,
        view_type: VkImageViewType,
        format: VkFormat,
        components: VkComponentMapping,
        subresource_range: VkImageSubresourceRange,
    ) -> Self {
        VkImageViewCreateInfo {
            s_type: VkStructureType::ImageViewCreateInfo,
            p_next: None,
            flags,
            image: image.inner(),
            view_type,
            format,
            components,
            subresource_range,
            phantom: PhantomData,
        }
    }
}
