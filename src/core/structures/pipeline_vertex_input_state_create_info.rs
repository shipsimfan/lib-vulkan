use crate::{
    VkPipelineVertexInputStateCreateFlags, VkStructureType, VkVertexInputAttributeDescription,
    VkVertexInputBindingDescription, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkPhysicalDeviceLimits};

/// Structure specifying parameters of a newly created pipeline vertex input state
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineVertexInputStateCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineVertexInputStateCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`] or a pointer to a valid instance of
    ///    [`VkPipelineVertexInputDivisorStateCreateInfo`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkPipelineVertexInputStateCreateFlags,

    /// `vertex_binding_description_count` is the number of vertex binding descriptions provided in
    /// `vertex_binding_descriptions`.
    ///
    /// # Valid Usage
    ///  - `vertex_binding_description_count` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_vertex_input_bindings`]
    pub vertex_binding_description_count: u32,

    /// `vertex_binding_descriptions` is a pointer to an array of
    /// [`VkVertexInputBindingDescription`] structures.
    ///
    /// # Valid Usage
    ///  - All elements of `vertex_binding_descriptions` must describe distinct binding numbers
    ///
    /// # Valid Usage (Implicit)
    ///  - If `vertex_binding_description_count` is not 0, `vertex_binding_descriptions` must be a
    ///    valid pointer to an array of `vertex_binding_description_count` valid
    ///    [`VkVertexInputBindingDescription`] structures
    pub vertex_binding_descriptions: *const VkVertexInputBindingDescription,

    /// `vertex_attribute_description_count` is the number of vertex attribute descriptions
    /// provided in `vertex_attribute_descriptions`.
    ///
    /// # Valid Usage
    ///  - `vertex_attribute_description_count` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_vertex_input_attributes`]
    pub vertex_attribute_description_count: u32,

    /// `vertex_attribute_descriptions` is a pointer to an array of
    /// [`VkVertexInputAttributeDescription`] structures.
    ///
    /// # Valid Usage
    ///  - For every binding specified by each element of `vertex_attribute_descriptions`, a
    ///    [`VkVertexInputBindingDescription`] must exist in `vertex_binding_descriptions` with the
    ///    same value of binding
    ///  - All elements of `vertex_attribute_descriptions` must describe distinct attribute
    ///    locations
    ///
    /// # Valid Usage (Implicit)
    ///  - If `vertex_attribute_description_count` is not 0, `vertex_attribute_descriptions` must
    ///    be a valid pointer to an array of `vertex_attribute_description_count` valid
    ///    [`VkVertexInputAttributeDescription`] structures
    pub vertex_attribute_descriptions: *const VkVertexInputAttributeDescription,
}

const impl Default for VkPipelineVertexInputStateCreateInfo {
    fn default() -> Self {
        VkPipelineVertexInputStateCreateInfo {
            r#type: VkStructureType::PipelineVertexInputStateCreateInfo,
            next: null(),
            flags: VkPipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: 0,
            vertex_binding_descriptions: null(),
            vertex_attribute_description_count: 0,
            vertex_attribute_descriptions: null(),
        }
    }
}

impl NextChain for VkPipelineVertexInputStateCreateInfo {
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
