use crate::{VK_FALSE, VkBool32, VkStructureType, util::NextChainMut};
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_TRUE, VK_VERSION_1_3, VkDevice, VkDeviceCreateInfo, VkFormat, VkPhysicalDeviceFeatures2,
    VkPipelineLayout,
};

/// Structure describing the Vulkan 1.3 features that can be supported by an implementation
///
/// # Description
/// If the [`VkPhysicalDeviceVulkan13Features`] structure is included in the `next` chain of the
/// [`VkPhysicalDeviceFeatures2`] structure passed to [`VkGetPhysicalDeviceFeatures2`], it is
/// filled in to indicate whether each corresponding feature is supported. If the application
/// wishes to use a [`VkDevice`] with any features described by
/// [`VkPhysicalDeviceVulkan13Features`], it must add an instance of the structure, with the
/// desired feature members set to [`VK_TRUE`], to the `next` chain of [`VkDeviceCreateInfo`] when
/// creating the [`VkDevice`].
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceVulkan13Features {
    /// Structure describing the Vulkan 1.3 features that can be supported by an implementation
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType`]
    pub r#type: VkStructureType,

    /// `next` is [`null_mut`] or a pointer to a structure extending this structure.
    pub next: *mut c_void,

    /// `robust_image_access` enables Robust Image Access guarantees for shader image accesses.
    pub robust_image_access: VkBool32,

    /// `inline_uniform_block` indicates whether the implementation supports inline uniform block
    /// descriptors. If this feature is not enabled, [`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`]
    /// must not be used.
    pub inline_uniform_block: VkBool32,

    /// `descriptor_binding_inline_uniform_blokc_update_after_bind` indicates whether the
    /// implementation supports updating inline uniform block descriptors after a set is bound. If
    /// this feature is not enabled, [`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`] must not be
    /// used with [`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`].
    pub descriptor_binding_inline_uniform_block_update_after_bind: VkBool32,

    /// `pipeline_creation_cache_control` indicates that the implementation supports:
    ///  - The following can be used in `Vk*PipelineCreateInfo::flags`:
    ///    - [`VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT`]
    ///    - [`VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`]
    ///  - The following can be used in [`VkPipelineCacheCreateInfo::flags`]:
    ///    - [`VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`]
    pub pipeline_creation_cache_control: VkBool32,

    /// `private_data` indicates whether the implementation supports private data.
    pub private_data: VkBool32,

    /// `shader_demote_to_helper_invocation` indicates whether the implementation supports the
    /// SPIR-V `DemoteToHelperInvocationExt` capability.
    pub shader_demote_to_helper_invocation: VkBool32,

    /// `shader_terminate_invocation` specifies whether the implementation supports SPIR-V modules
    /// that use the `SPV_KHR_terminate_invocation` extension.
    pub shader_terminate_invocation: VkBool32,

    /// `subgroup_size_control` indicates whether the implementation supports controlling shader
    /// subgroup sizes via the [`VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT`]
    /// flag and the [`VkPipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure.
    pub subgroup_size_control: VkBool32,

    /// `compute_full_subgroups` indicates whether the implementation supports requiring full
    /// subgroups in compute , mesh, or task shaders via the
    /// [`VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT`] flag.
    pub compute_full_subgroups: VkBool32,

    /// `synchronization2` indicates whether the implementation supports the new set of
    /// synchronization commands introduced in [`khr_synchronization2`].
    pub synchronization2: VkBool32,

    /// `texture_compression_astc_hdr` indicates whether all of the ASTC HDR compressed texture
    /// formats are supported. If this feature is enabled, then the
    /// [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`], [`VK_FORMAT_FEATURE_BLIT_SRC_BIT`] and
    /// [`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`] features must be supported in
    /// `optimal_tiling_features` for the following formats:
    ///  - [`VkFormat::Astc4x4SFloatBlock`]
    ///  - [`VkFormat::Astc5x4SFloatBlock`]
    ///  - [`VkFormat::Astc5x5SFloatBlock`]
    ///  - [`VkFormat::Astc6x5SFloatBlock`]
    ///  - [`VkFormat::Astc6x6SFloatBlock`]
    ///  - [`VkFormat::Astc8x5SFloatBlock`]
    ///  - [`VkFormat::Astc8x6SFloatBlock`]
    ///  - [`VkFormat::Astc8x8SFloatBlock`]
    ///  - [`VkFormat::Astc10x5SFloatBlock`]
    ///  - [`VkFormat::Astc10x6SFloatBlock`]
    ///  - [`VkFormat::Astc10x8SFloatBlock`]
    ///  - [`VkFormat::Astc10x10SFloatBlock`]
    ///  - [`VkFormat::Astc12x10SFloatBlock`]
    ///  - [`VkFormat::Astc12x12SFloatBlock`]
    ///
    /// To query for additional properties, or if the feature is not enabled,
    /// [`VkGetPhysicalDeviceFormatProperties`] and [`VkGetPhysicalDeviceImageFormatProperties`]
    /// can be used to check for supported properties of individual formats as normal.
    pub texture_compression_astc_hdr: VkBool32,

    /// `shader_zero_initialize_workgroup_memory` specifies whether the implementation supports
    /// initializing a variable in Workgroup storage class.
    pub shader_zero_initialize_workgroup_memory: VkBool32,

    /// `dynamic_rendering` specifies that the implementation supports dynamic render pass
    /// instances using the [`VkCmdBeginRendering`] command.
    pub dynamic_rendering: VkBool32,

    /// `shader_integer_dot_product` specifies whether shader modules can declare the
    /// `DotProductInputAllKHR`, `DotProductInput4x8KHR`, `DotProductInput4x8PackedKHR` and
    /// `DotProductKHR` capabilities.
    pub shader_integer_dot_product: VkBool32,

    /// `maintenance4` indicates that the implementation supports the following:
    ///  - The application may destroy a [`VkPipelineLayout`] object immediately after using it to
    ///    create another object.
    ///  - `LocalSizeId` can be used as an alternative to `LocalSize` to specify the local
    ///    workgroup size with specialization constants.
    ///  - Images created with identical creation parameters will always have the same alignment
    ///    requirements.
    ///  - The size memory requirement of a buffer or image is never greater than that of another
    ///    buffer or image created with a greater or equal size.
    ///  - Push constants do not have to be initialized before they are dynamically accessed.
    ///  - The interface matching rules allow a larger output vector to match with a smaller input
    ///    vector, with additional values being discarded.
    pub maintenance4: VkBool32,
}

const impl Default for VkPhysicalDeviceVulkan13Features {
    fn default() -> Self {
        VkPhysicalDeviceVulkan13Features {
            r#type: VkStructureType::PhysicalDeviceVulkan13Features,
            next: null_mut(),
            robust_image_access: VK_FALSE,
            inline_uniform_block: VK_FALSE,
            descriptor_binding_inline_uniform_block_update_after_bind: VK_FALSE,
            pipeline_creation_cache_control: VK_FALSE,
            private_data: VK_FALSE,
            shader_demote_to_helper_invocation: VK_FALSE,
            shader_terminate_invocation: VK_FALSE,
            subgroup_size_control: VK_FALSE,
            compute_full_subgroups: VK_FALSE,
            synchronization2: VK_FALSE,
            texture_compression_astc_hdr: VK_FALSE,
            shader_zero_initialize_workgroup_memory: VK_FALSE,
            dynamic_rendering: VK_FALSE,
            shader_integer_dot_product: VK_FALSE,
            maintenance4: VK_FALSE,
        }
    }
}

impl NextChainMut for VkPhysicalDeviceVulkan13Features {
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
