use crate::{VkDescriptorPoolCreateFlags, VkDescriptorPoolSize, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkDescriptorPoolCreateFlag, VkDescriptorSetLayoutCreateInfo, VkDescriptorType,
    VkResult,
};

/// Structure specifying parameters of a newly created descriptor pool
///
/// # Description
/// If multiple [`VkDescriptorPoolSize`] structures containing the same descriptor type appear in
/// the `pool_sizes` array then the pool will be created with enough storage for the total number
/// of descriptors of each type.
///
/// Fragmentation of a descriptor pool is possible and may lead to descriptor set allocation
/// failures. A failure due to fragmentation is defined as failing a descriptor set allocation
/// despite the sum of all outstanding descriptor set allocations from the pool plus the requested
/// allocation requiring no more than the total number of descriptors requested at pool creation.
/// Implementations provide certain guarantees of when fragmentation must not cause allocation
/// failure, as described below.
///
/// If a descriptor pool has not had any descriptor sets freed since it was created or most
/// recently reset then fragmentation must not cause an allocation failure (note that this is
/// always the case for a pool created without the
/// [`VkDescriptorPoolCreateFlag::FreeDescriptorSet`] bit set). Additionally, if all sets allocated
/// from the pool since it was created or most recently reset use the same number of descriptors
/// (of each type) and the requested allocation also uses that same number of descriptors (of each
/// type), then fragmentation must not cause an allocation failure.
///
/// If an allocation failure occurs due to fragmentation, an application can create an additional
/// descriptor pool to perform further descriptor set allocations.
///
/// If `flags` has the [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] bit set, descriptor pool
/// creation may fail with the error [`VkResult::VkErrorFragmentation`] if the total number of
/// descriptors across all pools (including this one) created with this bit set exceeds
/// `max_update_after_bind_descriptors_in_all_pools`, or if fragmentation of the underlying
/// hardware resources occurs.
///
/// If a `pool_sizes[i]::type` is [`VkDescriptorType::MutableExt`], a
/// [`VkMutableDescriptorTypeCreateInfoExt`] structure in the `next` chain can be used to specify
/// which mutable descriptor types can be allocated from the pool. If included in the `next` chain,
/// `VkMutableDescriptorTypeCreateInfoExt::mutable_descriptor_type_lists[i]` specifies which kind
/// of [`VkDescriptorType::MutableExt`] descriptors can be allocated from this pool entry. If
/// [`VkMutableDescriptorTypeCreateInfoExt`] does not exist in the `next` chain, or
/// `VkMutableDescriptorTypeCreateInfoExt::mutable_descriptor_type_lists[i]` is out of range, the
/// descriptor pool allocates enough memory to be able to allocate a
/// [`VkDescriptorType::MutableExt`] descriptor with any supported [`VkDescriptorType`] as a
/// mutable descriptor. A mutable descriptor can be allocated from a pool entry if the type list in
/// [`VkDescriptorSetLayoutCreateInfo`] is a subset of the type list declared in the descriptor
/// pool, or if the pool entry is created without a descriptor type list. Multiple `pool_sizes`
/// entries with [`VkDescriptorType::MutableExt`] can be declared. When multiple such pool entries
/// are present in `pool_sizes`, they specify sets of supported descriptor types which either fully
/// overlap, partially overlap, or are disjoint. Two sets fully overlap if the sets of supported
/// descriptor types are equal. If the sets are not disjoint they partially overlap. A pool entry
/// without a [`VkMutableDescriptorTypeListExt`] assigned to it is considered to partially overlap
/// any other pool entry which has a [`VkMutableDescriptorTypeListExt`] assigned to it. The
/// application must ensure that partial overlap does not exist in `pool_sizes`.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorPoolCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DescriptorPoolCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If `pool_sizes` contains a descriptorType of [`VkDescriptorType::INLINE_UNIFORM_BLOCK, the `next` chain must include a VkDescriptorPoolInlineUniformBlockCreateInfo structure whose maxInlineUniformBlockBindings member is not zero
    ///  - If a VkDataGraphProcessingEngineCreateInfoARM structure is included in the `next` chain, each member of pProcessingEngines must be identical to an VkQueueFamilyDataGraphPropertiesARM::engine retrieved from vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM with the physicalDevice that was used to create device
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be either [`null`] or a pointer to a valid instance of VkDataGraphProcessingEngineCreateInfoARM, VkDescriptorPoolInlineUniformBlockCreateInfo, or VkMutableDescriptorTypeCreateInfoEXT
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of VkDescriptorPoolCreateFlagBits specifying certain supported operations on the pool.
    ///
    /// # Valid Usage
    ///  - If `flags` has the [`VkDescriptorPoolCreateFlag::ALLOW_OVERALLOCATION_SETS_BIT_NV or [`VkDescriptorPoolCreateFlag::ALLOW_OVERALLOCATION_POOLS_BIT_NV bits set, then descriptorPoolOverallocation must be enabled
    ///  - If `flags` has the [`VkDescriptorPoolCreateFlag::HOST_ONLY_BIT_EXT bit set, then the [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] bit must not be set
    ///  - If `flags` has the [`VkDescriptorPoolCreateFlag::HOST_ONLY_BIT_EXT bit set, VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT::mutableDescriptorType must be enabled
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of VkDescriptorPoolCreateFlagBits values
    pub flags: VkDescriptorPoolCreateFlags,

    /// maxSets is the maximum number of descriptor sets that can be allocated from the pool.
    ///
    /// # Valid Usage
    ///  - If the descriptorPoolOverallocation feature is not enabled, or `flags` does not have [`VkDescriptorPoolCreateFlag::ALLOW_OVERALLOCATION_SETS_BIT_NV set, maxSets must be greater than 0
    pub max_sets: u32,

    /// poolSizeCount is the number of elements in `pool_sizes`.
    pub pool_size_count: u32,

    /// `pool_sizes` is a pointer to an array of VkDescriptorPoolSize structures, each containing a descriptor type and number of descriptors of that type to be allocated in the pool.
    ///
    /// # Valid Usage
    ///  - If VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT::mutableDescriptorType is not enabled, `pool_sizes` must not contain a descriptorType of [`VkDescriptorType::MutableExt`]
    ///  - If `pool_sizes` contains a descriptorType of [`VkDescriptorType::MutableExt`], any other [`VkDescriptorType::MutableExt`] element in `pool_sizes` must not have sets of supported descriptor types which partially overlap
    ///
    /// # Valid Usage (Implicit)
    ///  - If poolSizeCount is not 0, `pool_sizes` must be a valid pointer to an array of poolSizeCount valid VkDescriptorPoolSize structures
    pub pool_sizes: *const VkDescriptorPoolSize,
}

const impl Default for VkDescriptorPoolCreateInfo {
    fn default() -> Self {
        VkDescriptorPoolCreateInfo {
            r#type: VkStructureType::DescriptorPoolCreateInfo,
            next: null(),
            flags: VkDescriptorPoolCreateFlags::default(),
            max_sets: 0,
            pool_size_count: 0,
            pool_sizes: null(),
        }
    }
}

impl NextChain for VkDescriptorPoolCreateInfo {
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
