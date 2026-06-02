use crate::{VkBool32, VkStructureType, util::NextChainMut};
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_TRUE, VkDevice, VkDeviceCreateInfo, VkDynamicState, VkPhysicalDeviceFeatures2,
    ext_extended_dynamic_state,
};

/// Structure describing what extended dynamic state can be used
///
/// # Description
/// If the [`VkPhysicalDeviceExtendedDynamicStateFeaturesExt`] structure is included in the `next`
/// chain of the [`VkPhysicalDeviceFeatures2`] structure passed to
/// [`VkGetPhysicalDeviceFeatures2`], it is filled in to indicate whether each corresponding
/// feature is supported. If the application wishes to use a [`VkDevice`] with any features
/// described by [`VkPhysicalDeviceExtendedDynamicStateFeaturesExt`], it must add an instance of
/// the structure, with the desired feature members set to [`VK_TRUE`], to the `next` chain of
/// [`VkDeviceCreateInfo`] when creating the [`VkDevice`].
///
/// Provided by [`ext_extended_dynamic_state`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesExt {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PhysicalDeviceExtendedDynamicStateFeaturesExt`]
    pub r#type: VkStructureType,

    /// `next` is [`null_mut`] or a pointer to a structure extending this structure.
    pub next: *mut c_void,

    /// `extended_dynamic_state` indicates that the implementation supports the following dynamic
    /// states:
    ///  - [`VkDynamicState::CullMode`]
    ///  - [`VkDynamicState::FrontFace`]
    ///  - [`VkDynamicState::PrimitiveTopology`]
    ///  - [`VkDynamicState::ViewportWithCount`]
    ///  - [`VkDynamicState::ScissorWithCount`]
    ///  - [`VkDynamicState::VertexInputBindingStride`]
    ///  - [`VkDynamicState::DepthTestEnable`]
    ///  - [`VkDynamicState::DepthWriteEnable`]
    ///  - [`VkDynamicState::DepthCompareOp`]
    ///  - [`VkDynamicState::DepthBoundsTestEnable`]
    ///  - [`VkDynamicState::StencilTestEnable`]
    ///  - [`VkDynamicState::StencilOp`]
    pub extended_dynamic_state: VkBool32,
}

impl const Default for VkPhysicalDeviceExtendedDynamicStateFeaturesExt {
    fn default() -> Self {
        VkPhysicalDeviceExtendedDynamicStateFeaturesExt {
            r#type: VkStructureType::PhysicalDeviceExtendedDynamicStateFeaturesExt,
            next: null_mut(),
            extended_dynamic_state: 0,
        }
    }
}

impl NextChainMut for VkPhysicalDeviceExtendedDynamicStateFeaturesExt {
    fn next(&mut self) -> *mut c_void {
        self.next
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        (self as *mut Self).cast()
    }

    fn set_next(&mut self, next: *mut c_void) {
        self.next = next;
    }
}
