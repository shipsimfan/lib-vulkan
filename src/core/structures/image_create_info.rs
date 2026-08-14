use crate::{
    VkExtent3D, VkFormat, VkImageCreateFlags, VkImageLayout, VkImageTiling, VkImageType,
    VkImageUsageFlags, VkSampleCountFlag, VkSharingMode, VkStructureType,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VkImageCreateFlag, VkImageUsageFlag, VkResult};

/// Structure specifying the parameters of a newly created image object
///
/// # Description
/// `flags` defines the effective create flags for the image. If the `next` chain includes a
/// [`VkImageCreateFlags2CreateInfoKhr`] structure, `flags` is ignored, and the effective create
/// flags are defined by [`VkImageCreateFlags2CreateInfoKhr::flags`].
///
/// `usage` defines the effective usage flags for the image. If the `next` chain includes a
/// [`VkImageUsageFlags2CreateInfoKhr`] structure, `usage` is ignored, and the effective usage
/// flags are defined by [`VkImageUsageFlags2CreateInfoKhr::usage`].
///
/// Images created with tiling equal to [`VkImageTiling::Linear`] have further restrictions on
/// their limits and capabilities compared to images created with tiling equal to
/// [`VkImageTiling::Optimal`]. Creation of images with tiling [`VkImageTiling::Linear`] may not be
/// supported unless other parameters meet all of the constraints:
///  - `image_type` is [`VkImageType::_2d`]
///  - `format` is not a depth/stencil format
///  - `mip_levels` is 1
///  - `array_layers` is 1
///  - `samples` is [`VkSampleCountFlag::_1`]
///  - `usage` only includes [`VkImageUsageFlag::TransferSrc`] and/or
///    [`VkImageUsageFlag::TransferDst`]
///
/// Images created with one of the formats that require a sampler Y′CBCR conversion, have further
/// restrictions on their limits and capabilities compared to images created with other formats.
/// Creation of images with a format requiring Y′CBCR conversion may not be supported unless other
/// parameters meet all of the constraints:
///  - `image_type` is [`VkImageType::_2d`]
///  - `mip_levels` is 1
///  - `array_layers` is 1, unless the `ycbcr_image_arrays` feature is enabled, or otherwise
///    indicated by [`VkImageFormatProperties::max_array_layers`], as returned by
///    [`VkGetPhysicalDeviceImageFormatProperties`]
///  - `samples` is [`VkSampleCountFlag::_1`]
///
/// Images created with the [`VkImageUsageFlag::TileMemoryQcom`] usage flag set have further
/// restrictions on their limits and capabilities compared to images created without this flag.
/// Creation of images with usage including [`VkImageUsageFlag::TileMemoryQcom`] may not be
/// supported unless parameters meet all of the constraints:
///  - `flags` is 0 or only includes [`VkImageCreateFlag::Alias`]
///  - `image_type` is [`VkImageType::_2d`]
///  - `mip_levels` is 1
///  - `array_layers` is 1
///  - `samples` is [`VkSampleCountFlag::_1`]
///  - tiling is [`VkImageTiling::Optimal`]
///  - usage includes [`VkImageUsageFlag::TileMemoryQcom`] and any valid combination of the
///    following [`VkImageUsageFlag::Sampled`], [`VkImageUsageFlag::Storage`],
///    [`VkImageUsageFlag::ColorAttachment`], [`VkImageUsageFlag::DepthStencilAttachment`],
///    [`VkImageUsageFlag::InputAttachment`]
///
/// Implementations may support additional limits and capabilities beyond those listed above.
///
/// To determine the set of valid usage bits for a given format, call
/// [`VkGetPhysicalDeviceFormatProperties`].
///
/// If the size of the resultant image would exceed `max_resource_size`, then [`VkCreateImage`]
/// must fail and return [`VkResult::VkErrorOutOfDeviceMemory`]. This failure may occur even when
/// all image creation parameters satisfy their valid usage requirements.
///
/// If the implementation reports [`VK_TRUE`] in
/// [`VkPhysicalDeviceHostImageCopyProperties::identical_memory_type_requirements`], usage of
/// [`VkImageUsageFlag::HostTransfer`] must not affect the memory type requirements of the image as
/// described in Sparse Resource Memory Requirements and Resource Memory Association.
///
/// TODO: Add valid usage
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkImageCreateFlag`]s describing additional parameters of the
    /// image.
    pub flags: VkImageCreateFlags,

    /// `image_type` is a [`VkImageType`] value specifying the basic dimensionality of the image.
    /// Layers in array textures do not count as a dimension for the purposes of the image type.
    pub image_type: VkImageType,

    /// `format` is a [`VkFormat`] describing the format and type of the texel blocks that will be
    /// contained in the image.
    pub format: VkFormat,

    /// `extent` is a [`VkExtent3D`] describing the number of texels/pixels in each dimension of
    /// the base level.
    pub extent: VkExtent3D,

    /// `mip_levels` describes the number of levels of detail available for minified sampling of
    /// the image.
    pub mip_levels: u32,

    /// `array_layers` is the number of layers in the image.
    pub array_layers: u32,

    /// `samples` is a [`VkSampleCountFlag`] value specifying the number of samples per texel.
    pub samples: VkSampleCountFlag,

    /// `tiling` is a [`VkImageTiling`] value specifying the tiling arrangement of the texel blocks
    /// in memory.
    pub tiling: VkImageTiling,

    /// `usage` is a bitmask of [`VkImageUsageFlag`] describing the intended usage of the image.
    pub usage: VkImageUsageFlags,

    /// `sharing_mode` is a [`VkSharingMode`] value specifying the sharing mode of the image when
    /// it will be accessed by multiple queue families.
    pub sharing_mode: VkSharingMode,

    /// `queue_family_index_count` is the number of entries in the `queue_family_indices` array.
    pub queue_family_index_count: u32,

    /// `queue_family_indices` is a pointer to an array of queue families that will access this
    /// image. It is ignored if `sharing_mode` is not [`VkSharingMode::Concurrent`].
    pub queue_family_indices: *const u32,

    /// `initial_layout` is a [`VkImageLayout`] value specifying the initial [`VkImageLayout`] of
    /// all image subresources of the image.
    pub initial_layout: VkImageLayout,
}

const impl Default for VkImageCreateInfo {
    fn default() -> Self {
        VkImageCreateInfo {
            r#type: VkStructureType::ImageCreateInfo,
            next: null(),
            flags: VkImageCreateFlags::default(),
            image_type: VkImageType::_1d,
            format: VkFormat::Undefined,
            extent: VkExtent3D::default(),
            mip_levels: 0,
            array_layers: 0,
            samples: VkSampleCountFlag::_1,
            tiling: VkImageTiling::Optimal,
            usage: VkImageUsageFlags::default(),
            sharing_mode: VkSharingMode::Exclusive,
            queue_family_index_count: 0,
            queue_family_indices: null(),
            initial_layout: VkImageLayout::Undefined,
        }
    }
}
