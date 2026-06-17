use crate::{VK_FALSE, VkBool32, VkStructureType, util::NextChainMut};
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_TRUE, VK_VERSION_1_2, VkDevice, VkDeviceCreateInfo, VkGetPhysicalDeviceFeatures2,
    VkPhysicalDeviceFeatures2,
};

/// Structure describing the Vulkan 1.1 features that can be supported by an implementation
///
/// # Description
/// If the [`VkPhysicalDeviceVulkan11Features`] structure is included in the `next` chain of the
/// [`VkPhysicalDeviceFeatures2`] structure passed to [`VkGetPhysicalDeviceFeatures2`], it is
/// filled in to indicate whether each corresponding feature is supported. If the application
/// wishes to use a [`VkDevice`] with any features described by
/// [`VkPhysicalDeviceVulkan11Features`], it must add an instance of the structure, with the
/// desired feature members set to [`VK_TRUE`], to the `next` chain of [`VkDeviceCreateInfo`] when
/// creating the [`VkDevice`].
///
/// Provided by [`VK_VERSION_1_2`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceVulkan11Features {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PhysicalDeviceVulkan11Features`]
    pub r#type: VkStructureType,

    /// `next` is [`null_mut`] or a pointer to a structure extending this structure.
    pub next: *mut c_void,

    /// `storage_buffer_16_bit_access` specifies whether objects in the `StorageBuffer`,
    /// `ShaderRecordBufferKHR`, or `PhysicalStorageBuffer` storage class with the `Block`
    /// decoration can have 16-bit integer and 16-bit floating-point members. If this feature is
    /// not enabled, 16-bit integer or 16-bit floating-point members must not be used in such
    /// objects unless the `storage_buffer_8_bit_access` or
    /// `uniform_and_storage_buffer_8_bit_access` features are enabled or they are accessed in
    /// 32-bit multiples if the `shader_untyped_pointers` feature is enabled. This also specifies
    /// whether shader modules can declare the `StorageBuffer16BitAccess` capability.
    pub storage_buffer_16_bit_access: VkBool32,

    /// `uniform_and_storage_buffer_16_bit_access` specifies whether objects in the `Uniform`
    /// storage class with the `Block` decoration can have 16-bit integer and 16-bit floating-point
    /// members. If this feature is not enabled, 16-bit integer or 16-bit floating-point members
    /// must not be used in such objects unless the `uniform_and_storage_buffer_8_bit_access` are
    /// enabled or they are accessed in 32-bit multiples if the `shader_untyped_pointers` feature
    /// is enabled. This also specifies whether shader modules can declare the
    /// `UniformAndStorageBuffer16BitAccess` capability.
    pub uniform_and_storage_buffer_16_bit_access: VkBool32,

    /// `storage_push_constant_16` specifies whether objects in the `PushConstant` storage class
    /// can have 16-bit integer and 16-bit floating-point members. If this feature is not enabled,
    /// 16-bit integer or floating-point members must not be used in such objects unless the
    /// `storage_push_constant_8` feature is enabled or they are accessed in 32-bit multiples if
    /// the `shader_untyped_pointers` feature is enabled. This also specifies whether shader
    /// modules can declare the `StoragePushConstant16` capability.
    pub storage_push_constant_16: VkBool32,

    /// `storage_input_output_16` specifies whether objects in the `Input` and `Output` storage
    /// classes can have 16-bit integer and 16-bit floating-point members. If this feature is not
    /// enabled, 16-bit integer or 16-bit floating-point members must not be used in such objects.
    /// This also specifies whether shader modules can declare the `StorageInputOutput16`
    /// capability.
    pub storage_input_output_16: VkBool32,

    /// `multiview` specifies whether the implementation supports multiview rendering within a
    /// render pass. If this feature is not enabled, the view mask of each subpass must always be
    /// zero.
    pub multiview: VkBool32,

    /// `multiview_geometry_shader` specifies whether the implementation supports multiview
    /// rendering within a render pass, with geometry shaders. If this feature is not enabled, then
    /// a pipeline compiled against a subpass with a non-zero view mask must not include a geometry
    /// shader.
    pub multiview_geometry_shader: VkBool32,

    /// `multiview_tessellation_shader` specifies whether the implementation supports multiview
    /// rendering within a render pass, with tessellation shaders. If this feature is not enabled,
    /// then a pipeline compiled against a subpass with a non-zero view mask must not include any
    /// tessellation shaders.
    pub multiview_tessellation_shader: VkBool32,

    /// `variable_pointers_storage_buffer` specifies whether the implementation supports the SPIR-V
    /// `VariablePointersStorageBuffer` capability. When this feature is not enabled, shader
    /// modules must not declare the `SPV_KHR_variable_pointers` extension or the
    /// `VariablePointersStorageBuffer` capability.
    pub variable_pointers_storage_buffer: VkBool32,

    /// `variable_pointers` specifies whether the implementation supports the SPIR-V
    /// `VariablePointers` capability. When this feature is not enabled, shader modules must not
    /// declare the `VariablePointers` capability.
    pub variable_pointers: VkBool32,

    /// `protected_memory` specifies whether protected memory is supported.
    pub protected_memory: VkBool32,

    /// `sampler_ycbcr_conversion` specifies whether the implementation supports sampler Y′CBCR
    /// conversion. If `sampler_ycbcr_conversion` is [`VK_FALSE`], sampler Y′CBCR conversion is not
    /// supported, and samplers using sampler Y′CBCR conversion must not be used.
    pub sampler_ycbcr_conversion: VkBool32,

    /// `shader_draw_parameters` specifies whether the implementation supports the SPIR-V
    /// `DrawParameters` capability. When this feature is not enabled, shader modules must not
    /// declare the `SPV_KHR_shader_draw_parameters` extension or the `DrawParameters` capability.
    pub shader_draw_parameters: VkBool32,
}

impl const Default for VkPhysicalDeviceVulkan11Features {
    fn default() -> Self {
        VkPhysicalDeviceVulkan11Features {
            r#type: VkStructureType::PhysicalDeviceVulkan11Features,
            next: null_mut(),
            storage_buffer_16_bit_access: VK_FALSE,
            uniform_and_storage_buffer_16_bit_access: VK_FALSE,
            storage_push_constant_16: VK_FALSE,
            storage_input_output_16: VK_FALSE,
            multiview: VK_FALSE,
            multiview_geometry_shader: VK_FALSE,
            multiview_tessellation_shader: VK_FALSE,
            variable_pointers_storage_buffer: VK_FALSE,
            variable_pointers: VK_FALSE,
            protected_memory: VK_FALSE,
            sampler_ycbcr_conversion: VK_FALSE,
            shader_draw_parameters: VK_FALSE,
        }
    }
}

impl NextChainMut for VkPhysicalDeviceVulkan11Features {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&mut self) -> *mut c_void {
        self.next
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        (self as *mut Self).cast()
    }

    fn set_next(&mut self, next: Option<&mut dyn NextChainMut>) {
        self.next = next.map_or(null_mut(), |n| n.as_mut_ptr());
    }
}
