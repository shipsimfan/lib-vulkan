use crate::{
    VkBufferMemoryBarrier2, VkDependencyFlags, VkImageMemoryBarrier2, VkMemoryBarrier2,
    VkStructureType, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkDependencyFlag};

/// Structure specifying dependency information for a synchronization command
///
/// # Description
/// This structure defines a set of memory dependencies, as well as queue family ownership transfer
/// operations and image layout transitions.
///
/// Each member of `memory_barriers`, `buffer_memory_barriers`, and `image_memory_barriers` defines
/// a separate memory dependency.
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDependencyInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DependencyInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If a [`VkTensorDependencyInfoArm`] structure is included in the `next` chain, a
    ///    [`VkTensorMemoryBarrierArm`] structure must not be included in the `next` chain
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkMemoryRangeBarriersInfoKhr`],
    ///    [`VkTensorDependencyInfoArm`], or [`VkTensorMemoryBarrierArm`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `dependency_flags` is a bitmask of [`VkDependencyFlag`]s specifying how execution and
    /// memory dependencies are formed.
    ///
    /// # Valid Usage (Implicit)
    ///  - `dependency_flags` must be a valid combination of [`VkDependencyFlag`] values
    pub dependency_flags: VkDependencyFlags,

    /// `memory_barrier_count` is the length of the `memory_barriers` array.
    pub memory_barrier_count: u32,

    /// `memory_barriers` is a pointer to an array of [`VkMemoryBarrier2`] structures defining
    /// memory dependencies between any memory accesses.
    ///
    /// # Valid Usage
    ///  - For each element of `memory_barriers`, the `r#type` value of each structure in the
    ///    `next` chain must be unique
    ///  - For each element of `memory_barriers`, `next` must be either [`null`] or a pointer to a
    ///    valid instance of [`VkMemoryBarrierAccessFlags3Khr`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `memory_barrier_count` is not 0, `memory_barriers` must be a valid pointer to an
    ///    array of `memory_barrier_count` valid [`VkMemoryBarrier2`] structures
    pub memory_barriers: *const VkMemoryBarrier2,

    /// `buffer_memory_barrier_count` is the length of the `buffer_memory_barriers` array.
    pub buffer_memory_barrier_count: u32,

    /// `buffer_memory_barriers` is a pointer to an array of [`VkBufferMemoryBarrier2`] structures
    /// defining memory dependencies between buffer ranges.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `buffer_memory_barrier_count` is not 0, `buffer_memory_barriers` must be a valid
    ///    pointer to an array of `buffer_memory_barrier_count` valid [`VkBufferMemoryBarrier2`]
    ///    structures
    pub buffer_memory_barriers: *const VkBufferMemoryBarrier2,

    /// `image_memory_barrier_count` is the length of the `image_memory_barriers` array.
    pub image_memory_barrier_count: u32,

    /// `image_memory_barriers` is a pointer to an array of [`VkImageMemoryBarrier2`] structures
    /// defining memory dependencies between image subresources.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `image_memory_barrier_count` is not 0, `image_memory_barriers` must be a valid
    ///    pointer to an array of `image_memory_barrier_count` valid [`VkImageMemoryBarrier2`]
    ///    structures
    pub image_memory_barriers: *const VkImageMemoryBarrier2,
}

impl const Default for VkDependencyInfo {
    fn default() -> Self {
        VkDependencyInfo {
            r#type: VkStructureType::DependencyInfo,
            next: null(),
            dependency_flags: VkDependencyFlags::default(),
            memory_barrier_count: 0,
            memory_barriers: null(),
            buffer_memory_barrier_count: 0,
            buffer_memory_barriers: null(),
            image_memory_barrier_count: 0,
            image_memory_barriers: null(),
        }
    }
}

impl NextChain for VkDependencyInfo {
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
