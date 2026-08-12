use crate::{
    VkDescriptorSetLayoutBinding, VkDescriptorSetLayoutCreateFlags, VkStructureType,
    util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkDescriptorSetLayoutCreateFlag, VkDescriptorType};

/// Structure specifying parameters of a newly created descriptor set layout
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorSetLayoutCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DescriptorSetLayoutCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If any element `bindings[i]` has a `descriptor_type` of
    ///    [`VkDescriptorType::MutableExt`], then the `next` chain must include a
    ///    [`VkMutableDescriptorTypeCreateInfoExt`] structure, and
    ///    `mutable_descriptor_type_list_count` must be greater than `i`
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkDescriptorSetLayoutBindingFlagsCreateInfo`] or
    ///    [`VkMutableDescriptorTypeCreateInfoExt`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkDescriptorSetLayoutCreateFlag`] specifying options for
    /// descriptor set layout creation.
    ///
    /// # Valid Usage
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::PushDescriptor`], and the
    ///    [`khr_push_descriptor`] extension is not enabled, `push_descriptor` must be enabled
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::PushDescriptor`], `flags` must
    ///    not contain [`VkDescriptorSetLayoutCreateFlag::HostOnlyPoolExt`]
    ///  - If any binding has the [`VkDescriptorBindingFlag::UpdateAfterBind`] bit set, `flags`
    ///    must include [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`], `flags`
    ///    must not contain [`VkDescriptorSetLayoutCreateFlag::HostOnlyPoolExt`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::HostOnlyPoolExt`],
    ///    [`VkPhysicalDeviceMutableDescriptorTypeFeaturesExt::mutable_descriptor_type`] must be
    ///    enabled
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::EmbeddedImmutableSamplersExt`],
    ///    `flags` must also contain [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`], then
    ///    `flags` must not contain [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`], then
    ///    `flags` must not contain [`VkDescriptorSetLayoutCreateFlag::HostOnlyPoolExt`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::PerStageNv`], then
    ///    `per_stage_descriptor_set` must be enabled
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkDescriptorSetLayoutCreateFlag`] values
    pub flags: VkDescriptorSetLayoutCreateFlags,

    /// `binding_count` is the number of elements in `bindings`.
    pub binding_count: u32,

    /// `bindings` is a pointer to an array of [`VkDescriptorSetLayoutBinding`] structures.
    ///
    /// # Valid Usage
    ///  - If the `per_stage_descriptor_set` feature is not enabled, or `flags` does not contain
    ///    [`VkDescriptorSetLayoutCreateFlag::PerStageNv`], then the
    ///    [`VkDescriptorSetLayoutBinding::binding`] members of the elements of the `bindings`
    ///    array must each have different values
    ///  - If flags contains [`VkDescriptorSetLayoutCreateFlag::PushDescriptor`], then all elements
    ///    of `bindings` must not have a `descriptor_type` of
    ///    [`VkDescriptorType::UniformBufferDynamic`] or [`VkDescriptorType::StorageBufferDynamic`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::PushDescriptor`], then all
    ///    elements of `bindings` must not have a `descriptor_type` of
    ///    [`VkDescriptorType::InlineUniformBlock`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::PushDescriptor`], then the total
    ///    number of elements of all bindings must be less than or equal to
    ///    [`VkPhysicalDevicePushDescriptorProperties::max_push_descriptors`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::PushDescriptor`], `bindings` must
    ///    not have a `descriptor_type` of [`VkDescriptorType::MutableExt`]
    ///  - If any binding has the [`VkDescriptorBindingFlag::UpdateAfterBind`] bit set, then all
    ///    bindings must not have `descriptor_type` of [`VkDescriptorType::UniformBufferDynamic`]
    ///    or [`VkDescriptorType::StorageBufferDynamic`]
    ///  - If a binding has a `descriptor_type` value of [`VkDescriptorType::MutableExt`], then
    ///    `immutable_samplers` must be [`null`]
    ///  - If [`VkPhysicalDeviceMutableDescriptorTypeFeaturesExt::mutable_descriptor_type`] is not
    ///    enabled, `bindings` must not contain a `descriptor_type` of
    ///    [`VkDescriptorType::MutableExt`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`], then all
    ///    elements of `bindings` must not have a `descriptor_type` of
    ///    [`VkDescriptorType::UniformBufferDynamic`] or [`VkDescriptorType::StorageBufferDynamic`]
    ///  - If `flags` contains [`VkDescriptorSetLayoutCreateFlag::PerStageNv`], then there must not
    ///    be any two elements of the `bindings` array with the same
    ///    [`VkDescriptorSetLayoutBinding::binding`] value and their
    ///    [`VkDescriptorSetLayoutBinding::stage_flags`] containing the same bit
    ///
    /// # Valid Usage (Implicit)
    ///  - If `binding_count` is not 0, `bindings` must be a valid pointer to an array of
    ///    `binding_count` valid [`VkDescriptorSetLayoutBinding`] structures
    pub bindings: *const VkDescriptorSetLayoutBinding,
}

const impl Default for VkDescriptorSetLayoutCreateInfo {
    fn default() -> Self {
        VkDescriptorSetLayoutCreateInfo {
            r#type: VkStructureType::DescriptorSetLayoutCreateInfo,
            next: null(),
            flags: VkDescriptorSetLayoutCreateFlags::default(),
            binding_count: 0,
            bindings: null(),
        }
    }
}

impl NextChain for VkDescriptorSetLayoutCreateInfo {
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
