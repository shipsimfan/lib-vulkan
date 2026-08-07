use crate::{VkDescriptorSet, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkDescriptorPoolCreateFlag, VkDescriptorSetLayoutCreateFlag, VkDescriptorType,
    VkDevice, VkWriteDescriptorSet,
};

/// Structure specifying a copy descriptor set operation
///
/// # Description
/// If the [`VkDescriptorSetLayoutBinding`] for `dst_binding` is [`VkDescriptorType::MutableExt`]
/// and `src_binding` is not [`VkDescriptorType::MutableExt`], the new active descriptor type
/// becomes the descriptor type of `src_binding`. If both [`VkDescriptorSetLayoutBinding`] for
/// `src_binding` and `dst_binding` are [`VkDescriptorType::MutableExt`], the active descriptor
/// type in each source descriptor is copied into the corresponding destination descriptor. The
/// active descriptor type can be different for each source descriptor.
///
/// # Valid Usage (Implicit)
///  - Both of `dst_set`, and `src_set` must have been created, allocated, or retrieved from the
///    same [`VkDevice`]
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkCopyDescriptorSet {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::CopyDescriptorSet`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `src_set` is the source set.
    ///
    /// # Valid Usage
    ///  - If `src_set` is equal to `dst_set`, then the source and destination ranges of
    ///    descriptors must not overlap, where the ranges may include array elements from
    ///    consecutive bindings
    ///
    /// # Valid Usage (Implicit)
    ///  - `src_set` must be a valid [`VkDescriptorSet`] handle
    pub src_set: VkDescriptorSet,

    /// `src_binding` is the source binding.
    ///
    /// # Valid Usage
    ///  - `src_binding` must be a valid binding within `src_set`
    ///  - The sum of `src_array_element` and `descriptor_count` must be less than or equal to the
    ///    number of array elements in the descriptor set binding specified by `src_binding`, and
    ///    all applicable consecutive bindings
    pub src_binding: u32,

    /// `src_array_element` is the array element. If the descriptor binding identified by `src_set`
    /// and `src_binding` has a descriptor type of [`VkDescriptorType::InlineUniformBlock`] then
    /// `src_array_element` specifies the starting byte offset within the binding to copy from.
    ///
    /// # Valid Usage
    ///  - If the descriptor type of the descriptor set binding specified by `src_binding` is
    ///    [`VkDescriptorType::InlineUniformBlock`], `src_array_element` must be an integer
    ///    multiple of 4
    pub src_array_element: u32,

    /// `dst_set` is the destination set.
    ///
    /// # Valid Usage
    ///  - If `src_set`’s layout was created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] flag set, then `dst_set`’s
    ///    layout must also have been created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] flag set
    ///  - If `src_set`’s layout was created without either the
    ///    [`VkDescriptorSetLayoutCreateFlag::HostOnlyPoolExt`] flag or the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] flag set, then `dst_set`’s
    ///    layout must have been created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] flag set
    ///  - If the descriptor pool from which `src_set` was allocated was created with the
    ///    [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] flag set, then the descriptor pool from
    ///    which `dst_set` was allocated must also have been created with the
    ///    [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] flag set
    ///  - If the descriptor pool from which `src_set` was allocated was created without either the
    ///    [`VkDescriptorPoolCreateFlag::HostOnlyExt`] flag or the
    ///    [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] flag set, then the descriptor pool from
    ///    which `dst_set` was allocated must have been created without the
    ///    [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] flag set
    ///  - If the descriptor type of the descriptor set binding specified by `dst_binding` is
    ///    [`VkDescriptorType::Sampler`], then `dst_set` must not have been allocated with a layout
    ///    that included immutable samplers for `dst_binding`
    ///
    /// # Valid Usage (Implicit)
    ///  - `dst_set` must be a valid [`VkDescriptorSet`] handle
    pub dst_set: VkDescriptorSet,

    /// `dst_binding`, is the destination binding.
    ///
    /// # Valid Usage
    ///  - `dst_binding` must be a valid binding within `dst_set`
    ///  - The sum of `dst_array_element` and `descriptor_count` must be less than or equal to the
    ///    number of array elements in the descriptor set binding specified by `dst_binding`, and
    ///    all applicable consecutive bindings
    ///  - The type of `dst_binding` within `dst_set` must be equal to the type of `src_binding`
    ///    within `src_set`
    ///  - If [`VkDescriptorSetLayoutBinding`] for `dst_set` at `dst_binding` is
    ///    [`VkDescriptorType::MutableExt`], the new active descriptor type must exist in the
    ///    corresponding `mutable_descriptor_type_lists` list for `dst_binding` if the new active
    ///    descriptor type is not [`VkDescriptorType::MutableExt`]
    ///  - If [`VkDescriptorSetLayoutBinding`] for `src_set` at `src_binding` is
    ///    [`VkDescriptorType::MutableExt`] and the [`VkDescriptorSetLayoutBinding`] for `dst_set`
    ///    at `dst_binding` is not [`VkDescriptorType::MutableExt`], the active descriptor type for
    ///    the source descriptor must match the descriptor type of `dst_binding`
    ///  - If [`VkDescriptorSetLayoutBinding`] for `dst_set` at `dst_binding` is
    ///    [`VkDescriptorType::MutableExt`], and the new active descriptor type is
    ///    [`VkDescriptorType::MutableExt`], the `mutable_descriptor_type_lists` for `src_binding`
    ///    and `dst_binding` must match exactly
    pub dst_binding: u32,

    /// `dst_array_element` is the destination array element. If the descriptor binding identified
    /// by `dst_set` and `dst_binding` has a descriptor type of
    /// [`VkDescriptorType::InlineUniformBlock`] then `dst_array_element` specifies the starting
    /// byte offset within the binding to copy to.
    ///
    /// # Valid Usage
    ///  - If the descriptor type of the descriptor set binding specified by `dst_binding` is
    ///    [`VkDescriptorType::InlineUniformBlock`], `dst_array_element` must be an integer
    ///    multiple of 4
    pub dst_array_element: u32,

    /// `descriptor_count` is the number of descriptors to copy from the source to destination. If
    /// `descriptor_count` is greater than the number of remaining array elements in the source or
    /// destination binding, those affect consecutive bindings in a manner similar to
    /// [`VkWriteDescriptorSet`]. If the descriptor binding identified by `src_set` and
    /// `src_binding` has a descriptor type of [`VkDescriptorType::InlineUniformBlock`] then
    /// `descriptor_count` specifies the number of bytes to copy and the remaining array elements
    /// in the source or destination binding refer to the remaining number of bytes in those.
    ///
    /// # Valid Usage
    ///  - If the descriptor type of the descriptor set binding specified by either `src_binding`
    ///    or `dst_binding` is [`VkDescriptorType::InlineUniformBlock`], `descriptor_count` must be
    ///    an integer multiple of 4
    pub descriptor_count: u32,
}

const impl Default for VkCopyDescriptorSet {
    fn default() -> Self {
        VkCopyDescriptorSet {
            r#type: VkStructureType::CopyDescriptorSet,
            next: null(),
            src_set: VkDescriptorSet::null(),
            src_binding: 0,
            src_array_element: 0,
            dst_set: VkDescriptorSet::null(),
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 0,
        }
    }
}

impl NextChain for VkCopyDescriptorSet {
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
