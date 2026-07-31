use crate::{VkAllocationCallbacks, VkDescriptorSetLayout, VkDevice};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a descriptor set layout object
///
/// # Parameters
///  - `device` is the logical device that destroys the descriptor set layout.
///  - `descriptor_set_layout` is the descriptor set layout to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Valid Usage
///  - If [`VkAllocationCallbacks`] were provided when `descriptor_set_layout` was created, a
///    compatible set of callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `descriptor_set_layout` was created,
///    `allocator` must be [`null`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `descriptor_set_layout` is not [`VK_NULL_HANDLE`], `descriptor_set_layout` must be a
///    valid [`VkDescriptorSetLayout`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `descriptor_set_layout` is a valid handle, it must have been created, allocated, or
///    retrieved from `device`
///
/// # Host Synchronization
///  - Host access to `descriptor_set_layout` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyDescriptorSetLayout = unsafe extern "system" fn(
    device: VkDevice,
    descriptor_set_layout: VkDescriptorSetLayout,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyDescriptorSetLayout`]
pub const VK_DESTROY_DESCRIPTOR_SET_LAYOUT: &CStr = c"vkDestroyDescriptorSetLayout";
