use crate::{
    VkBufferView, VkDescriptorBufferInfo, VkDescriptorImageInfo, VkDescriptorSet, VkDescriptorType,
    VkStructureType, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_0, VK_WHOLE_SIZE, VkBufferUsageFlag, VkDevice, VkDeviceMemory,
    VkImageView, VkPhysicalDeviceLimits, VkSampler, VkShaderStageFlags,
};

/// Structure specifying the parameters of a descriptor set write operation
///
/// # Description
/// Members of `image_info`, `buffer_info` and `texel_buffer_view` are only accessed by the
/// implementation when they correspond to a descriptor type being defined - otherwise they are
/// ignored. The members accessed are as follows for each descriptor type:
///  - For [`VkDescriptorType::Sampler`], only the sampler member of each element of
///    [`VkWriteDescriptorSet::image_info`] is accessed.
///  - For [`VkDescriptorType::SampledImage`], [`VkDescriptorType::StorageImage`], or
///    [`VkDescriptorType::InputAttachment`], only the `image_view` and `image_layout` members of
///    each element of [`VkWriteDescriptorSet::image_info`] are accessed.
///  - For [`VkDescriptorType::CombinedImageSampler`], all members of each element of
///    [`VkWriteDescriptorSet::image_info`] are accessed.
///  - For [`VkDescriptorType::UniformBuffer`], [`VkDescriptorType::StorageBuffer`],
///    [`VkDescriptorType::UniformBufferDynamic`], or [`VkDescriptorType::StorageBufferDynamic`],
///    all members of each element of [`VkWriteDescriptorSet::buffer_info`] are accessed.
///  - For [`VkDescriptorType::UniformTexelBuffer`] or [`VkDescriptorType::StorageTexelBuffer`],
///    each element of [`VkWriteDescriptorSet::texel_buffer_view`] is accessed.
///
/// When updating descriptor sets with a `descriptor_type` of
/// [`VkDescriptorType::InlineUniformBlock`], none of the `image_info`, `buffer_info`, or
/// `texel_buffer_view` members are accessed, instead the source data of the descriptor update
/// operation is taken from the [`VkWriteDescriptorSetInlineUniformBlock`] structure in the `next`
/// chain of [`VkWriteDescriptorSet`]. When updating descriptor sets with a `descriptor_type` of
/// [`VkDescriptorType::AccelerationStructureKhr`], none of the `image_info`, `buffer_info`, or
/// `texel_buffer_view` members are accessed, instead the source data of the descriptor update
/// operation is taken from the [`VkWriteDescriptorSetAccelerationStructureKhr`] structure in the
/// `next` chain of [`VkWriteDescriptorSet`]. When updating descriptor sets with a
/// `descriptor_type` of [`VkDescriptorType::AccelerationStructureNv`], none of the `image_info`,
/// `buffer_info`, or `texel_buffer_view` members are accessed, instead the source data of the
/// descriptor update operation is taken from the [`VkWriteDescriptorSetAccelerationStructureNv`]
/// structure in the `next` chain of [`VkWriteDescriptorSet`]. When updating descriptor sets with a
/// `descriptor_type` of [`VkDescriptorType::TensorArm`], none of the `image_info`, `buffer_info`,
/// or `texel_buffer_view` members are accessed, instead the source data of the descriptor update
/// operation is taken from the instance of [`VkWriteDescriptorSetTensorArm`] in the `next` chain
/// of [`VkWriteDescriptorSet`].
///
/// If the `null_descriptor` feature is enabled, the buffer, acceleration structure, tensor,
/// `image_view`, or `buffer_view` can be [`VK_NULL_HANDLE`]. Loads from a null descriptor return
/// zero values and stores and atomics to a null descriptor are discarded. A null acceleration
/// structure descriptor results in the miss shader being invoked.
///
/// If the destination descriptor is a mutable descriptor, the active descriptor type for the
/// destination descriptor becomes `descriptor_type`.
///
/// # Consecutive Binding Updates
/// If the `dst_binding` has fewer than `descriptor_count` array elements remaining starting from
/// `dst_array_element`, then the remainder will be used to update the subsequent binding -
/// `dst_binding + 1` starting at array element zero. If a binding has a `descriptor_count` of
/// zero, it is skipped. This behavior applies recursively, with the update affecting consecutive
/// bindings as needed to update all `descriptor_count` descriptors. Consecutive bindings must have
/// identical [`VkDescriptorType`], [`VkShaderStageFlags`], [`VkDescriptorBindingFlag`], and
/// immutable samplers references. In addition, if the [`VkDescriptorType`] is
/// [`VkDescriptorType::MutableExt`], the supported descriptor types in
/// [`VkMutableDescriptorTypeCreateInfoExt`] must be equally defined.
///
/// # Valid Usage (Implicit)
///  - Both of `dst_set`, and the elements of `texel_buffer_view` that are valid handles of
///    non-ignored parameters must have been created, allocated, or retrieved from the same
///    [`VkDevice`]
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkWriteDescriptorSet {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::WriteDescriptorSet`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkWriteDescriptorSetAccelerationStructureKhr`],
    ///    [`VkWriteDescriptorSetAccelerationStructureNv`],
    ///    [`VkWriteDescriptorSetInlineUniformBlock`],
    ///    [`VkWriteDescriptorSetPartitionedAccelerationStructureNv`], or
    ///    [`VkWriteDescriptorSetTensorArm`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `dst_set` is the destination descriptor set to update.
    ///
    /// # Valid Usage
    ///  - `dst_set` must be a valid [`VkDescriptorSet`] handle
    ///  - If the [`VkDescriptorSetLayoutBinding`] for `dst_set` at `dst_binding` is
    ///    [`VkDescriptorType::MutableExt`], the new active descriptor type `descriptor_type` must
    ///    exist in the corresponding `mutable_descriptor_type_lists` list for `dst_binding`
    pub dst_set: VkDescriptorSet,

    /// `dst_binding` is the descriptor binding within that set.
    ///
    /// # Valid Usage
    ///  - `dst_binding` must be less than or equal to the maximum value of binding of all
    ///    [`VkDescriptorSetLayoutBinding`] structures specified when `dst_set`’s descriptor set
    ///    layout was created
    ///  - `dst_binding` must be a binding with a non-zero `descriptor_count`
    ///  - `dst_binding` must be a binding with a non-zero
    ///    [`VkDescriptorSetLayoutCreateInfo::binding_count`]
    pub dst_binding: u32,

    /// `dst_array_element` is the starting element in that array. If the descriptor binding
    /// identified by `dst_set` and `dst_binding` has a descriptor type of
    /// [`VkDescriptorType::InlineUniformBlock`] then `dst_array_element` specifies the starting
    /// byte offset within the binding.
    ///
    /// # Valid Usage
    ///  - The sum of `dst_array_element` and `descriptor_count` must be less than or equal to the
    ///    number of array elements in the descriptor set binding specified by `dst_binding`, and
    ///    all applicable consecutive bindings
    pub dst_array_element: u32,

    /// `descriptor_count` is the number of descriptors to update. If the descriptor binding
    /// identified by `dst_set` and `dst_binding` has a descriptor type of
    /// [`VkDescriptorType::InlineUniformBlock`], then `descriptor_count` specifies the number of
    /// bytes to update. Otherwise, `descriptor_count` is one of
    ///  - the number of elements in `image_info`
    ///  - the number of elements in `buffer_info`
    ///  - the number of elements in `texel_buffer_view`
    ///  - a value matching the `data_size` member of a [`VkWriteDescriptorSetInlineUniformBlock`]
    ///    structure in the `next` chain
    ///  - a value matching the `acceleration_structure_count` of a
    ///    [`VkWriteDescriptorSetAccelerationStructureKhr`] or
    ///    [`VkWriteDescriptorSetAccelerationStructureNv`] structure in the `next` chain
    ///  - a value matching the `descriptor_count` of a [`VkWriteDescriptorSetTensorArm`] structure
    ///    in the `next` chain
    ///
    /// # Valid Usage
    ///  - All consecutive bindings updated via a single [`VkWriteDescriptorSet`] structure, except
    ///    those with a `descriptor_count` of zero, must have identical `descriptor_type`
    ///  - All consecutive bindings updated via a single [`VkWriteDescriptorSet`] structure, except
    ///    those with a `descriptor_count` of zero, must have identical `stage_flags`
    ///  - All consecutive bindings updated via a single [`VkWriteDescriptorSet`] structure, except
    ///    those with a `descriptor_count` of zero, must all either use immutable samplers or must
    ///    all not use immutable samplers
    ///  - All consecutive bindings updated via a single [`VkWriteDescriptorSet`] structure, except
    ///    those with a `descriptor_count` of zero, must have identical [`VkDescriptorBindingFlag`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `descriptor_count` must be greater than 0
    pub descriptor_count: u32,

    /// `descriptor_type` is a [`VkDescriptorType`] specifying the type of each descriptor in
    /// `image_info`, `buffer_info`, or `texel_buffer_view`, as described below. If
    /// [`VkDescriptorSetLayoutBinding`] for `dst_set` at `dst_binding` is not equal to
    /// [`VkDescriptorType::MutableExt`], `descriptor_type` must be the same type as the
    /// `descriptor_type` specified in [`VkDescriptorSetLayoutBinding`] for `dst_set` at
    /// `dst_binding`. The type of the descriptor also controls which array the descriptors are
    /// taken from.
    ///
    /// # Valid Usage
    ///  - `descriptor_type` must match the type of `dst_binding` within `dst_set`
    ///  - If `descriptor_type` is [`VkDescriptorType::InlineUniformBlock`], `dst_array_element`
    ///    must be an integer multiple of 4
    ///  - If `descriptor_type` is [`VkDescriptorType::InlineUniformBlock`], `descriptor_count`
    ///    must be an integer multiple of 4
    ///  - If `descriptor_type` is [`VkDescriptorType::UniformTexelBuffer`] or
    ///    [`VkDescriptorType::StorageTexelBuffer`], each element of `texel_buffer_view` must be
    ///    either a valid [`VkBufferView`] handle or [`VK_NULL_HANDLE`]
    ///  - If `descriptor_type` is [`VkDescriptorType::UniformTexelBuffer`] or
    ///    [`VkDescriptorType::StorageTexelBuffer`] and the `null_descriptor` feature is not
    ///    enabled, each element of `texel_buffer_view` must not be [`VK_NULL_HANDLE`]
    ///  - If `descriptor_type` is [`VkDescriptorType::UniformBuffer`],
    ///    [`VkDescriptorType::StorageBuffer`], [`VkDescriptorType::UniformBufferDynamic`], or
    ///    [`VkDescriptorType::StorageBufferDynamic`], `buffer_info` must be a valid pointer to an
    ///    array of `descriptor_count` valid [`VkDescriptorBufferInfo`] structures
    ///  - If `descriptor_type` is [`VkDescriptorType::Sampler`] or
    ///    [`VkDescriptorType::CombinedImageSampler`], and `dst_set` was not allocated with a
    ///    layout that included immutable samplers for `dst_binding` with `descriptor_type`, the
    ///    sampler member of each element of `image_info` must be a valid [`VkSampler`] object
    ///  - If `descriptor_type` is [`VkDescriptorType::CombinedImageSampler`],
    ///    [`VkDescriptorType::SampledImage`], or [`VkDescriptorType::StorageImage`], the
    ///    `image_view` member of each element of `image_info` must be either a valid
    ///    [`VkImageView`] handle or [`VK_NULL_HANDLE`]
    ///  - If `descriptor_type` is [`VkDescriptorType::CombinedImageSampler`],
    ///    [`VkDescriptorType::SampledImage`], or [`VkDescriptorType::StorageImage`], and the
    ///    `null_descriptor` feature is not enabled, the `image_view` member of each element of
    ///    `image_info` must not be [`VK_NULL_HANDLE`]
    ///  - If `descriptor_type` is [`VkDescriptorType::SampleWeightImageQcom`],
    ///    [`VkDescriptorType::BlockMatchImageQcom`], or [`VkDescriptorType::InputAttachment`],
    ///    then the `image_view` member of each element of `image_info` must not be
    ///    [`VK_NULL_HANDLE`]
    ///  - If `descriptor_type` is [`VkDescriptorType::InlineUniformBlock`], the `next` chain must
    ///    include a [`VkWriteDescriptorSetInlineUniformBlock`] structure whose `data_size` member
    ///    equals `descriptor_count`
    ///  - If `descriptor_type` is [`VkDescriptorType::AccelerationStructureKhr`], the `next` chain
    ///    must include a [`VkWriteDescriptorSetAccelerationStructureKhr`] structure whose
    ///    `acceleration_structure_count` member equals `descriptor_count`
    ///  - If `descriptor_type` is [`VkDescriptorType::AccelerationStructureNv`], the `next` chain
    ///    must include a [`VkWriteDescriptorSetAccelerationStructureNv`] structure whose
    ///    `acceleration_structure_count` member equals `descriptor_count`
    ///  - If `descriptor_type` is [`VkDescriptorType::TensorArm`], the `next` chain must include a
    ///    [`VkWriteDescriptorSetTensorArm`] structure whose tensorViewCount member equals
    ///    `descriptor_count`
    ///  - If `descriptor_type` is [`VkDescriptorType::SampledImage`], then the `image_view` member
    ///    of each `image_info` element must have been created without a
    ///    [`VkSamplerYcbcrConversionInfo`] structure in its `next` chain
    ///  - If `descriptor_type` is [`VkDescriptorType::CombinedImageSampler`], and if any element
    ///    of `image_info` has an `image_view` member that was created with a
    ///    [`VkSamplerYcbcrConversionInfo`] structure in its `next` chain, then `dst_set` must have
    ///    been allocated with a layout that included immutable samplers for `dst_binding`, and the
    ///    corresponding immutable sampler must have been created with an identically defined
    ///    [`VkSamplerYcbcrConversionInfo`] object
    ///  - If `descriptor_type` is [`VkDescriptorType::CombinedImageSampler`], and `dst_set` was
    ///    allocated with a layout that included immutable samplers for `dst_binding`, then the
    ///    `image_view` member of each element of `image_info` which corresponds to an immutable
    ///    sampler that enables sampler Y′CBCR conversion must have been created with a
    ///    [`VkSamplerYcbcrConversionInfo`] structure in its `next` chain with an identically
    ///    defined [`VkSamplerYcbcrConversionInfo`] to the corresponding immutable sampler
    ///  - If `descriptor_type` is [`VkDescriptorType::CombinedImageSampler`], `dst_set` was
    ///    allocated with a layout that included immutable samplers for `dst_binding`, and those
    ///    samplers enable sampler Y′CBCR conversion, then `image_view` must not be
    ///    [`VK_NULL_HANDLE`]
    ///  - If `descriptor_type` is [`VkDescriptorType::UniformBuffer`] or
    ///    [`VkDescriptorType::UniformBufferDynamic`], the `offset` member of each element of
    ///    `buffer_info` must be a multiple of
    ///    [`VkPhysicalDeviceLimits::min_uniform_buffer_offset_alignment`]
    ///  - If `descriptor_type` is [`VkDescriptorType::StorageBuffer`] or
    ///    [`VkDescriptorType::StorageBufferDynamic`], the `offset` member of each element of
    ///    `buffer_info` must be a multiple of
    ///    [`VkPhysicalDeviceLimits::min_storage_buffer_offset_alignment`]
    ///  - If `descriptor_type` is [`VkDescriptorType::UniformBuffer`],
    ///    [`VkDescriptorType::UniformBufferDynamic`], [`VkDescriptorType::StorageBuffer`], or
    ///    [`VkDescriptorType::StorageBufferDynamic`], and the `buffer` member of any element of
    ///    `buffer_info` is the handle of a non-sparse buffer, then that buffer must be bound
    ///    completely and contiguously to a single [`VkDeviceMemory`] object
    ///  - If `descriptor_type` is [`VkDescriptorType::UniformBuffer`] or
    ///    [`VkDescriptorType::UniformBufferDynamic`], the `buffer` member of each element of
    ///    `buffer_info` must have been created with the [`VkBufferUsageFlag::UniformBuffer`] usage
    ///    flag set
    ///  - If `descriptor_type` is [`VkDescriptorType::StorageBuffer`] or
    ///    [`VkDescriptorType::StorageBufferDynamic`], the `buffer` member of each element of
    ///    `buffer_info` must have been created with the [`VkBufferUsageFlag::StorageBuffer`] usage
    ///    flag set
    ///  - If `descriptor_type` is [`VkDescriptorType::UniformBuffer`] or
    ///    [`VkDescriptorType::UniformBufferDynamic`], the `range` member of each element of
    ///    `buffer_info`, or the effective range if `range` is [`VK_WHOLE_SIZE`], must be less than
    ///    or equal to [`VkPhysicalDeviceLimits::max_uniform_buffer_range`]
    ///  - If `descriptor_type` is [`VkDescriptorType::StorageBuffer`] or
    ///    [`VkDescriptorType::StorageBufferDynamic`], and the `shader_64_bit_indexing` feature is
    ///    not enabled, the `range` member of each element of `buffer_info`, or the effective range
    ///    if `range` is [`VK_WHOLE_SIZE`], must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_storage_buffer_range`]
    ///  - If `descriptor_type` is [`VkDescriptorType::UniformTexelBuffer`], the
    ///    `texel_buffer_view` buffer view usage must include
    ///    [`VkBufferUsageFlag::UniformTexelBuffer`]
    ///  - If `descriptor_type` is [`VkDescriptorType::StorageTexelBuffer`], the
    ///    `texel_buffer_view` buffer view usage must include
    ///    [`VkBufferUsageFlag::StorageTexelBuffer`]
    ///  - If `descriptor_type` is [`VkDescriptorType::StorageImage`] or
    ///    [`VkDescriptorType::InputAttachment`], the `image_view` member of each element of
    ///    `image_info` must have been created with the identity swizzle
    ///  - If `descriptor_type` is [`VkDescriptorType::SampledImage`] or
    ///    [`VkDescriptorType::CombinedImageSampler`], the `image_view` member of each element of
    ///    `image_info` must have been created with the [`VkImageUsageFlag::Sampled`] usage flag
    ///    set
    ///  - If `descriptor_type` is [`VkDescriptorType::SampledImage`] the `image_layout` member of
    ///    each element of `image_info` must be a member of the list given in Sampled Image
    ///  - If `descriptor_type` is [`VkDescriptorType::CombinedImageSampler`] the `image_layout`
    ///    member of each element of `image_info` must be a member of the list given in Combined
    ///    Image Sampler
    ///  - If `descriptor_type` is [`VkDescriptorType::InputAttachment`] the `image_layout` member
    ///    of each element of `image_info` must be a member of the list given in Input Attachment
    ///  - If `descriptor_type` is [`VkDescriptorType::StorageImage`] the `image_layout` member of
    ///    each element of `image_info` must be a member of the list given in Storage Image
    ///  - If `descriptor_type` is [`VkDescriptorType::InputAttachment`], the `image_view` member
    ///    of each element of `image_info` must have been created with the
    ///    [`VkImageUsageFlag::InputAttachment`] usage flag set
    ///  - If `descriptor_type` is [`VkDescriptorType::StorageImage`], the `image_view` member of
    ///    each element of `image_info` must have been created with the
    ///    [`VkImageUsageFlag::Storage`] usage flag set
    ///  - If `descriptor_type` is [`VkDescriptorType::Sampler`], then `dst_set` must not have been
    ///    allocated with a layout that included immutable samplers for `dst_binding`
    ///  - If `descriptor_type` is [`VkDescriptorType::InputAttachment`], the `image_view` member
    ///    of each element of `image_info` must have either been created without a
    ///    [`VkImageViewMinLodCreateInfoExt`] included in the `next` chain or with a
    ///    [`VkImageViewMinLodCreateInfoExt::min_lod`] of 0.0
    ///  - If `descriptor_type` is [`VkDescriptorType::SampleWeightImageQcom`], the `image_view`
    ///    member of each element of `image_info` must have been created with a view created with
    ///    an image created with the [`VkImageUsageFlag::SampleWeightQcom`] usage flag set
    ///  - If `descriptor_type` is [`VkDescriptorType::BlockMatchImageQcom`], the `image_view`
    ///    member of each element of `image_info` must have been created with a view created with
    ///    an image created with the [`VkImageUsageFlag::SampleBlockMatchQcom`] usage flag set
    ///
    /// # Valid Usage (Implicit)
    ///  - `descriptor_type` must be a valid [`VkDescriptorType`] value
    pub descriptor_type: VkDescriptorType,

    /// `image_info` is a pointer to an array of [`VkDescriptorImageInfo`] structures or is
    /// ignored, as described below.
    pub image_info: *const VkDescriptorImageInfo,

    /// `buffer_info` is a pointer to an array of [`VkDescriptorBufferInfo`] structures or is
    /// ignored, as described below.
    pub buffer_info: *const VkDescriptorBufferInfo,

    /// `texel_buffer_view` is a pointer to an array of [`VkBufferView`] handles as described in
    /// the Buffer Views section or is ignored, as described below.
    pub texel_buffer_view: *const VkBufferView,
}

const impl Default for VkWriteDescriptorSet {
    fn default() -> Self {
        VkWriteDescriptorSet {
            r#type: VkStructureType::WriteDescriptorSet,
            next: null(),
            dst_set: VkDescriptorSet::null(),
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 0,
            descriptor_type: VkDescriptorType::Sampler,
            image_info: null(),
            buffer_info: null(),
            texel_buffer_view: null(),
        }
    }
}

impl NextChain for VkWriteDescriptorSet {
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
