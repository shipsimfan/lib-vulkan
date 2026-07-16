use crate::{
    VkBufferCreateFlags, VkBufferUsageFlags, VkDeviceSize, VkSharingMode, VkStructureType,
    util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_TRUE, VK_VERSION_1_0, VkBuffer, VkBufferCreateFlag, VkBufferUsageFlag};

/// Structure specifying the parameters of a newly created buffer object
///
/// # Description
/// `usage` defines the effective usage flags for the buffer. If the `next` chain includes a
/// [`VkBufferUsageFlagFlags2CreateInfo`] structure, usage is ignored, and the effective usage
/// flags are defined by [`VkBufferUsageFlagFlags2CreateInfo::usage`].
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::BufferCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkExternalMemoryBufferCreateInfo`] structure, its
    ///    `handle_types` member must only contain bits that are also in
    ///    `VkExternalBufferProperties::external_memory_properties.compatible_handle_types`, as
    ///    returned by [`VkGetPhysicalDeviceExternalBufferProperties`] with
    ///    `external_buffer_info.handle_type` equal to any one of the handle types specified in
    ///    [`VkExternalMemoryBufferCreateInfo::handle_types`]
    ///  - If the `next` chain includes a [`VkVideoProfileListInfoKhr`] structure and for any
    ///    element of its pProfiles member `video_codec_operation` is
    ///    [`VkVideoCodecOperationFlag::DecodeVp9Khr`], then the `video_decode_vp9` feature must be
    ///    enabled
    ///  - If the `next` chain includes a [`VkVideoProfileListInfoKhr`] structure and for any
    ///    element of its `profiles` member `video_codec_operation` is
    ///    [`VkVideoCodecOperationFlag::EncodeAv1Khr`], then the `video_encode_av1` feature must be
    ///    enabled
    ///  - If the `next` chain includes a [`VkVideoEncodeProfileRgbConversionInfoValve`] structure,
    ///    then the `video_encode_rgb_conversion` feature must be enabled
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkBufferCollectionBufferCreateInfoFuchsia`], [`VkBufferDeviceAddressCreateInfoExt`],
    ///    [`VkBufferOpaqueCaptureAddressCreateInfo`], [`VkBufferUsageFlagFlags2CreateInfo`],
    ///    [`VkDedicatedAllocationBufferCreateInfoNv`], [`VkExternalMemoryBufferCreateInfo`],
    ///    [`VkOpaqueCaptureDescriptorDataCreateInfoExt`], or [`VkVideoProfileListInfoKhr`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkBufferCreateFlag`]s specifying additional parameters of the
    /// buffer.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkBufferCreateFlag`] values
    ///  - If the `sparse_binding` feature is not enabled, `flags` must not contain
    ///    [`VkBufferCreateFlag::SparseBinding`]
    ///  - If the `sparse_residency_buffer` feature is not enabled, `flags` must not contain
    ///    [`VkBufferCreateFlag::SparseResidency`]
    ///  - If the sparseResidencyAliased feature is not enabled, `flags` must not contain
    ///    [`VkBufferCreateFlag::SparseAliased`]
    ///  - If `flags` contains [`VkBufferCreateFlag::SparseResidency`] or
    ///    [`VkBufferCreateFlag::SparseAliased`], it must also contain
    ///    [`VkBufferCreateFlag::SparseBinding`]
    ///  - If the `protected_memory` feature is not enabled, `flags` must not contain
    ///    [`VkBufferCreateFlag::Protected`]
    ///  - If any of the bits [`VkBufferCreateFlag::SparseBinding`],
    ///    [`VkBufferCreateFlag::SparseResidency`], or [`VkBufferCreateFlag::SparseAliased`] are
    ///    set, [`VkBufferCreateFlag::Protected`] must not also be set
    ///  - If the `next` chain includes a [`VkDedicatedAllocationBufferCreateInfoNv`] structure,
    ///    and the `dedicated_allocation` member of the chained structure is [`VK_TRUE`], then
    ///    `flags` must not include [`VkBufferCreateFlag::SparseBinding`],
    ///    [`VkBufferCreateFlag::SparseResidency`], or [`VkBufferCreateFlag::SparseAliased`]
    ///  - If [`VkBufferDeviceAddressCreateInfoExt::device_address`] is not zero, `flags` must
    ///    include [`VkBufferCreateFlag::DeviceAddressCaptureReplay`]
    ///  - If [`VkBufferOpaqueCaptureAddressCreateInfo::opaque_capture_address`] is not zero,
    ///    `flags` must include [`VkBufferCreateFlag::DeviceAddressCaptureReplay`]
    ///  - If `flags` includes [`VkBufferCreateFlag::DeviceAddressCaptureReplay`], the
    ///    [`VkPhysicalDeviceBufferDeviceAddressFeaturesExt::buffer_device_address_capture_replay`]
    ///    feature or the `buffer_device_address_capture_replay` feature must be enabled
    ///  - If `flags` includes [`VkBufferCreateFlag::VideoProfileIndependentKhr`], then
    ///    `video_maintenance1` must be enabled
    ///  - If `flags` includes [`VkBufferCreateFlag::DescriptorBufferCaptureReplayExt`], the
    ///    `descriptor_buffer_capture_replay` feature must be enabled
    ///  - If the `next` chain includes a [`VkOpaqueCaptureDescriptorDataCreateInfoExt`] structure,
    ///    `flags` must contain [`VkBufferCreateFlag::DescriptorBufferCaptureReplayExt`]
    ///  - If usage includes [`VkBufferUsageFlag::TileMemoryQcom`], then `flags` must not contain any of
    ///    the [`VkBufferCreateFlag::DeviceAddressCaptureReplay`],
    ///    [`VkBufferCreateFlag::DescriptorBufferCaptureReplayExt`],
    ///    [`VkBufferCreateFlag::VideoProfileIndependentKhr`],
    ///    [`VkBufferCreateFlag::SparseBinding`], [`VkBufferCreateFlag::SparseResidency`],
    ///    [`VkBufferCreateFlag::SparseAliased`], or [`VkBufferCreateFlag::Protected`] flags
    ///  - If `flags` includes [`VkBufferCreateFlag::Protected`], then the effective `usage` flags
    ///    must not contain bits other than:
    ///    - [`VkBufferUsageFlag::TransferSrc`]
    ///    - [`VkBufferUsageFlag::TransferDst`]
    ///    - [`VkBufferUsageFlag::UniformTexelBuffer`]
    ///    - [`VkBufferUsageFlag::StorageTexelBuffer`]
    ///    - [`VkBufferUsageFlag::UniformBuffer`]
    ///    - [`VkBufferUsageFlag::StorageBuffer`]
    ///    - [`VkBufferUsageFlag::ShaderDeviceAddress`]
    ///    - [`VkBufferUsageFlag::VideoDecodeSrcKhr`]
    ///    - [`VkBufferUsageFlag::VideoEncodeDstKhr`]
    ///    - [`VkBufferUsageFlag::DescriptorHeapExt`]
    ///  - If the `protected_descriptor_heaps` property is not supported and the effective `usage`
    ///    flags include the [`VkBufferUsageFlag::DescriptorHeapExt`] flag, flags must not include
    ///    the [`VkBufferCreateFlag::Protected`] flag
    ///  - If the `sparse_descriptor_heaps` property is not supported and the effective `usage`
    ///    flags include the [`VkBufferUsageFlag::DescriptorHeapExt`] flag, flags must not include
    ///    any of the [`VkBufferCreateFlag::SparseBinding`],
    ///    [`VkBufferCreateFlag::SparseResidency`], or [`VkBufferCreateFlag::SparseAliased`] flags
    pub flags: VkBufferCreateFlags,

    /// `size` is the size in bytes of the buffer to be created.
    ///
    /// # Valid Usage
    ///  - `size` must be greater than 0
    ///  - `size` must be less than or equal to
    ///    [`VkPhysicalDeviceMaintenance4Properties::max_buffer_size`]
    pub size: VkDeviceSize,

    /// `usage` is a bitmask of [`VkBufferUsageFlags`] specifying allowed usages of the buffer.
    ///
    /// # Valid Usage
    ///  - If the `next` chain does not include a [`VkBufferUsageFlagFlags2CreateInfo`] structure,
    ///    usage must be a valid combination of [`VkBufferUsageFlags`] values
    ///  - If the `next` chain does not include a [`VkBufferUsageFlagFlags2CreateInfo`] structure,
    ///    usage must not be 0
    ///  - If usage includes [`VkBufferUsageFlag::VideoDecodeSrcKhr`] or
    ///    [`VkBufferUsageFlag::VideoDecodeDstKhr`], and flags does not include
    ///    [`VkBufferCreateFlag::VideoProfileIndependentKhr`], then the `next` chain must include a
    ///    [`VkVideoProfileListInfoKhr`] structure with `profile_count` greater than 0 and
    ///    `profiles` including at least one [`VkVideoProfileInfoKhr`] structure with a
    ///    `video_codec_operation` member specifying a decode operation
    ///  - If usage includes [`VkBufferUsageFlag::VideoEncodeSrcKhr`] or
    ///    [`VkBufferUsageFlag::VideoEncodeDstKhr`], and flags does not include
    ///    [`VkBufferCreateFlag::VideoProfileIndependentKhr`], then the `next` chain must include a
    ///    [`VkVideoProfileListInfoKhr`] structure with `profile_count` greater than 0 and
    ///    `profiles` including at least one [`VkVideoProfileInfoKhr`] structure with a
    ///    `video_codec_operation` member specifying an encode operation
    ///  - If `usage` includes [`VkBufferUsageFlag::SamplerDescriptorBufferExt`], creating this
    ///    [`VkBuffer`] must not cause the total required space for all currently valid buffers
    ///    using this flag on the device to exceed
    ///    [`VkPhysicalDeviceDescriptorBufferPropertiesExt::sampler_descriptor_buffer_address_space_size`]
    ///    or
    ///    [`VkPhysicalDeviceDescriptorBufferPropertiesExt::descriptor_buffer_address_space_size`]
    ///  - If `usage` includes [`VkBufferUsageFlag::ResourceDescriptorBufferExt`], creating this
    ///    [`VkBuffer`] must not cause the total required space for all currently valid buffers
    ///    using this flag on the device to exceed
    ///    [`VkPhysicalDeviceDescriptorBufferPropertiesExt::resource_descriptor_buffer_address_space_size`]
    ///    or
    ///    [`VkPhysicalDeviceDescriptorBufferPropertiesExt::descriptor_buffer_address_space_size`]
    ///  - If `usage` includes [`VkBufferUsageFlag::PushDescriptorsDescriptorBufferExt`], the
    ///    `descriptor_buffer_push_descriptors` feature must be enabled
    ///  - If `usage` includes [`VkBufferUsageFlag::PushDescriptorsDescriptorBufferExt`],
    ///    [`VkPhysicalDeviceDescriptorBufferPropertiesExt::bufferless_push_descriptors`] must be
    ///    [`VK_FALSE`]
    ///  - If `usage` includes [`VkBufferUsageFlag::PushDescriptorsDescriptorBufferExt`], usage
    ///    must contain at least one of [`VkBufferUsageFlag::ResourceDescriptorBufferExt`] or
    ///    [`VkBufferUsageFlag::SamplerDescriptorBufferExt`]
    ///  - If the `tile_memory_heap` feature is not enabled, usage must not include
    ///    [`VkBufferUsageFlag::TileMemoryQcom`]
    ///  - If `usage` includes [`VkBufferUsageFlag::TileMemoryQcom`], then only the following
    ///    usages may be set:
    ///    - [`VkBufferUsageFlag::UniformTexelBuffer`]
    ///    - [`VkBufferUsageFlag::StorageTexelBuffer`]
    ///    - [`VkBufferUsageFlag::UniformBuffer`]
    ///    - [`VkBufferUsageFlag::StorageBuffer`]
    ///    - [`VkBufferUsageFlag::ShaderDeviceAddress`]
    ///    - and if [`VkPhysicalDeviceTileMemoryHeapPropertiesQcom::tile_buffer_transfers`] is
    ///      [`VK_TRUE`] then additionally [`VkBufferUsageFlag::TransferSrc`] or
    ///      [`VkBufferUsageFlag::TransferDst`]
    pub usage: VkBufferUsageFlags,

    /// `sharing_mode` is a [`VkSharingMode`] value specifying the sharing mode of the buffer when
    /// it will be accessed by multiple queue families.
    ///
    /// # Valid Usage (Implicit)
    ///  - `sharing_mode` must be a valid [`VkSharingMode`] value
    pub sharing_mode: VkSharingMode,

    /// `queue_family_index_count` is the number of entries in the `queue_family_indices` array.
    ///
    /// # Valid Usage
    ///  - If the `maintenance11` feature is enabled and `sharing_mode` is
    ///    [`VkSharingMode::Concurrent`], then `queue_family_index_count` must be greater than 0
    ///  - If the `maintenance11` feature is not enabled and `sharing_mode` is
    ///    [`VkSharingMode::Concurrent`], then `queue_family_index_count` must be greater than 1
    pub queue_family_index_count: u32,

    /// `queue_family_indices` is a pointer to an array of queue families that will access this
    /// buffer. It is ignored if `sharing_mode` is not [`VkSharingMode::Concurrent`].
    ///
    /// # Valid Usage
    ///  - If `sharing_mode` is [`VkSharingMode::Concurrent`], `queue_family_indices` must be a
    ///    valid pointer to an array of `queue_family_index_count` [`u32`] values
    ///  - If `sharing_mode` is [`VkSharingMode::Concurrent`], each element of
    ///    `queue_family_indices` must be unique and must be less than
    ///    `queue_family_property_count` returned by either
    ///    [`VkGetPhysicalDeviceQueueFamilyProperties2`] or
    ///    [`VkGetPhysicalDeviceQueueFamilyProperties`] for the `physical_device` that was used to
    ///    create device
    pub queue_family_indices: *const u32,
}

const impl Default for VkBufferCreateInfo {
    fn default() -> Self {
        VkBufferCreateInfo {
            r#type: VkStructureType::BufferCreateInfo,
            next: null(),
            flags: VkBufferCreateFlags::default(),
            size: 0,
            usage: VkBufferUsageFlags::default(),
            sharing_mode: VkSharingMode::Exclusive,
            queue_family_index_count: 0,
            queue_family_indices: null(),
        }
    }
}

impl NextChain for VkBufferCreateInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
