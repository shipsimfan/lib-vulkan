use crate::{VkDescriptorPool, VkDescriptorSetLayout, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkDescriptorPoolCreateFlag, VkDescriptorSetLayoutBinding,
    VkDescriptorSetLayoutCreateFlag, VkDevice, VkPhysicalDeviceProperties,
};

/// Structure specifying the allocation parameters for descriptor sets
///
/// # Valid Usage (Implicit)
///  - Both of `descriptor_pool`, and the elements of `set_layouts` must have been created,
///    allocated, or retrieved from the same [`VkDevice`]
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorSetAllocateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DescriptorSetAllocateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If `set_layouts[i]` was created with an element of `binding_flags` that includes
    ///    [`VkDescriptorBindingFlag::VariableDescriptorCount`], and
    ///    [`VkDescriptorSetVariableDescriptorCountAllocateInfo`] is included in the `next` chain,
    ///    and [`VkDescriptorSetVariableDescriptorCountAllocateInfo::descriptor_set_count`] is not
    ///    zero, then `VkDescriptorSetVariableDescriptorCountAllocateInfo::descriptor_counts[i]`
    ///    must be less than or equal to [`VkDescriptorSetLayoutBinding::descriptor_count`] for the
    ///    corresponding binding used to create `set_layouts[i]`
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`] or a pointer to a valid instance of
    ///    [`VkDescriptorSetVariableDescriptorCountAllocateInfo`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `descriptor_pool` is the pool which the sets will be allocated from.
    ///
    /// # Valid Usage
    ///  - If the [`khr_maintenance1`] extension is not enabled and
    ///    [`VkPhysicalDeviceProperties::api_version`] is less than Vulkan 1.1, `descriptor_pool`
    ///    must have enough free descriptor capacity remaining to allocate the descriptor sets of
    ///    the specified layouts
    ///  - If any element of `set_layouts` was created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set, `descriptor_pool` must
    ///    have been created with the [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] flag set
    ///  - If any element of `set_layouts` was created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::HostOnlyPoolExt`] bit set, `descriptor_pool` must
    ///    have been created with the [`VkDescriptorPoolCreateFlag::HostOnlyExt`] flag set
    ///
    /// # Valid Usage (Implicit)
    ///  - `descriptor_pool` must be a valid [`VkDescriptorPool`] handle
    ///
    /// # Host Synchronization
    ///  - Host access to `descriptor_pool` must be externally synchronized
    pub descriptor_pool: VkDescriptorPool,

    /// `descriptor_set_count` determines the number of descriptor sets to be allocated from the
    /// pool.
    ///
    /// # Valid Usage
    ///  - If the [`khr_maintenance1`] extension is not enabled and
    ///    [`VkPhysicalDeviceProperties::api_version`] is less than Vulkan 1.1,
    ///    `descriptor_set_count` must not be greater than the number of sets that are currently
    ///    available for allocation in `descriptor_pool`
    ///
    /// # Valid Usage (Implicit)
    ///  - `descriptor_set_count` must be greater than 0
    pub descriptor_set_count: u32,

    /// `set_layouts` is a pointer to an array of descriptor set layouts, with each member
    /// specifying how the corresponding descriptor set is allocated.
    ///
    /// # Valid Usage
    ///  - Each element of `set_layouts` must not have been created with
    ///    [`VkDescriptorSetLayoutCreateFlag::PushDescriptor`] set
    ///  - Each element of `set_layouts` must not have been created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`] bit set
    ///
    /// # Valid Usage (Implicit)
    ///  - `set_layouts` must be a valid pointer to an array of `descriptor_set_count` valid
    ///    [`VkDescriptorSetLayout`] handles
    pub set_layouts: *const VkDescriptorSetLayout,
}

const impl Default for VkDescriptorSetAllocateInfo {
    fn default() -> Self {
        VkDescriptorSetAllocateInfo {
            r#type: VkStructureType::DescriptorSetAllocateInfo,
            next: null(),
            descriptor_pool: VkDescriptorPool::null(),
            descriptor_set_count: 0,
            set_layouts: null(),
        }
    }
}

impl NextChain for VkDescriptorSetAllocateInfo {
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
