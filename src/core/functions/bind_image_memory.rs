use crate::{VkDevice, VkDeviceMemory, VkDeviceSize, VkImage, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkImageCreateFlag, VkMemoryHeapFlag,
    VkMemoryPropertyFlag, VkMemoryRequirements, VkPhysicalDeviceProperties,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Bind device memory to an image object
///
/// # Parameters
///  - `device` is the logical device that owns the image and memory.
///  - `image` is the image.
///  - `memory` is the [`VkDeviceMemory`] object describing the device memory to attach.
///  - `memory_offset` is the start offset of the region of memory which is to be bound to the
///    image. The number of bytes returned in the [`VkMemoryRequirements::size`] member in memory,
///    starting from `memory_offset` bytes, will be bound to the specified image.
///
/// # Description
/// [`VkBindImageMemory`] is equivalent to passing the same parameters through
/// [`VkBindImageMemoryInfo`] to [`VkBindImageMemory2`].
///
/// # Valid Usage
///  - `image` must not have been bound to a memory object
///  - `image` must not have been created with any sparse memory binding flags
///  - `memory_offset` must be less than the size of `memory`
///  - If `image` requires a dedicated allocation (as reported by [`VkGetImageMemoryRequirements2`]
///    in [`VkMemoryDedicatedRequirements::requires_dedicated_allocation`] for image), `memory`
///    must have been created with [`VkMemoryDedicatedAllocateInfo::image`] equal to `image`
///  - If the `dedicated_allocation_image_aliasing` feature is not enabled, and the
///    [`VkMemoryAllocateInfo`] provided when `memory` was allocated included a
///    [`VkMemoryDedicatedAllocateInfo`] structure in its `next` chain, and
///    [`VkMemoryDedicatedAllocateInfo::image`] was not [`VK_NULL_HANDLE`], then `image` must equal
///    [`VkMemoryDedicatedAllocateInfo::image`] and `memory_offset` must be zero
///  - If the `dedicated_allocation_image_aliasing` feature is enabled, and the
///    [`VkMemoryAllocateInfo`] provided when `memory` was allocated included a
///    [`VkMemoryDedicatedAllocateInfo`] structure in its `next` chain, and
///    [`VkMemoryDedicatedAllocateInfo::image`] was not [`VK_NULL_HANDLE`], then `memory_offset`
///    must be zero, and `image` must be either equal to [`VkMemoryDedicatedAllocateInfo::image`]
///    or an image that was created using the same parameters in [`VkImageCreateInfo`], with the
///    exception that `extent` and `array_layers` may differ subject to the following restrictions:
///    every dimension in the `extent` parameter of the image being bound must be equal to or
///    smaller than the original image for which the allocation was created; and the `array_layers`
///    parameter of the image being bound must be equal to or smaller than the original image for
///    which the allocation was created
///  - If the [`VkMemoryAllocateInfo`] provided when `memory` was allocated included a
///    [`VkMemoryDedicatedAllocateInfo`] structure in its `next` chain,
///    [`VkMemoryDedicatedAllocateInfo::buffer`] must have been [`VK_NULL_HANDLE`]
///  - If `image` was created with the [`VkImageCreateFlag::Protected`] bit set, the image must be
///    bound to a memory object allocated with a memory type that reports
///    [`VkMemoryPropertyFlag::Protected`]
///  - If `image` was created with the [`VkImageCreateFlag::Protected`] bit not set, the image must
///    not be bound to a memory object created with a memory type that reports
///    [`VkMemoryPropertyFlag::Protected`]
///  - If `image` was created with [`VkDedicatedAllocationImageCreateInfoNv::dedicated_allocation`]
///    equal to [`VK_TRUE`], `memory` must have been created with
///    [`VkDedicatedAllocationMemoryAllocateInfoNv::image`] equal to an image handle created with
///    identical creation parameters to image and `memory_offset` must be zero
///  - If the [`khr_dedicated_allocation`] extension is not enabled,
///    [`VkPhysicalDeviceProperties::api_version`] is less than Vulkan 1.1, and `image` was not
///    created with [`VkDedicatedAllocationImageCreateInfoNv::dedicated_allocation`] equal to
///    [`VK_TRUE`], `memory` must not have been allocated dedicated for a specific buffer or image
///  - If the value of [`VkExportMemoryAllocateInfo::handle_types`] used to allocate `memory` is
///    not 0, it must include at least one of the handles set in
///    [`VkExternalMemoryImageCreateInfo::handle_types`] when `image` was created
///  - If `memory` was created by a memory import operation, that is not
///    [`VkImportAndroidHardwareBufferInfoAndroid`] with a non-[`null_mut`] `buffer` value, the
///    external handle type of the imported memory must also have been set in   
///    [`VkExternalMemoryImageCreateInfo::handle_types`] when `image` was created
///  - If `memory` was created with the [`VkImportAndroidHardwareBufferInfoAndroid`] memory import
///    operation with a non-[`null_mut`] `buffer` value,
///    [`VkExternalMemoryHandleTypeFlagAndroid::HardwareBufferAndroid`] must also have been set in
///    [`VkExternalMemoryImageCreateInfo::handle_types`] when `image` was created
///  - If the `image` was created with the [`VkImageCreateFlag::DescriptorHeapCaptureReplayExt`]
///    bit set, `memory` must have been allocated with the [`VkMemoryAllocateFlag::DeviceAddress`]
///    bit set
///  - If the `image` was created with the [`VkImageCreateFlag::DescriptorHeapCaptureReplayExt`]
///    bit set, `memory` must have been allocated with the
///    [`VkMemoryAllocateFlag::DeviceAddressCaptureReplay`] bit set
///  - `image` must not have been created with the [`VkImageCreateFlag::Disjoint`] set
///  - `memory` must have been allocated using one of the memory types allowed in the
///    `memory_type_bits` member of the [`VkMemoryRequirements`] structure returned from a call to
///    [`VkGetImageMemoryRequirements`] with `image`
///  - If `memory` was not allocated from a memory heap with the
///    [`VkMemoryHeapFlag::TileMemoryQcom`] property set, `memory_offset` must be an integer
///    multiple of the `alignment` member of the [`VkMemoryRequirements`] structure returned from a
///    call to [`VkGetImageMemoryRequirements`] with `image`
///  - If `memory` was allocated from a memory heap with the [`VkMemoryHeapFlag::TileMemoryQcom`]
///    property set, `memory_offset` must be an integer multiple of the alignment member of the
///    [`VkTileMemoryRequirementsQcom`] structure returned from a call to
///    [`VkGetImageMemoryRequirements2`] with `image`
///  - If `memory` was not allocated from a memory heap with the
///    [`VkMemoryHeapFlag::TileMemoryQcom`] property set, `size` member of the
///    [`VkMemoryRequirements`] structure returned from a call to [`VkGetImageMemoryRequirements`]
///    with `image` must be less than or equal to the size of `memory` minus `memory_offset`
///  - If `memory` was allocated from a memory heap with the [`VkMemoryHeapFlag::TileMemoryQcom`]
///    property set, `size` member of the [`VkTileMemoryRequirementsQcom`] structure returned from
///    a call to [`VkGetImageMemoryRequirements2`] with image must be less than or equal to the
///    size of `memory` minus `memory_offset`
///  - If `image` was created with [`VkBufferCollectionImageCreateInfoFuchsia`] chained to
///    [`VkImageCreateInfo::next`], `memory` must be allocated with a
///    [`VkImportMemoryBufferCollectionFuchsia`] chained to [`VkMemoryAllocateInfo::next`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `image` must be a valid [`VkImage`] handle
///  - `memory` must be a valid [`VkDeviceMemory`] handle
///  - `image` must have been created, allocated, or retrieved from `device`
///  - `memory` must have been created, allocated, or retrieved from `device`
///
/// # Host Synchronization
///  - Host access to `image` must be externally synchronized
///
/// # Return Codes
///
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkBindImageMemory = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memory_offset: VkDeviceSize,
) -> VkResult;

/// The name of [`VkBindImageMemory`]
pub const VK_BIND_IMAGE_MEMORY: &CStr = c"vkBindImageMemory";
