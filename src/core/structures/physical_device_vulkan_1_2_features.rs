use crate::{VK_FALSE, VkBool32, VkStructureType, util::NextChainMut};
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_TRUE, VK_VERSION_1_2, VkCreateDevice, VkDescriptorType, VkDevice, VkDeviceCreateInfo,
    VkGetBufferDeviceAddress, VkGetPhysicalDeviceFeatures2, VkImageAspectFlag, VkImageLayout,
    VkImageMemoryBarrier, VkPhysicalDeviceFeatures2, VkSamplerAddressMode,
};

/// Structure describing the Vulkan 1.2 features that can be supported by an implementation
///
/// # Description
/// If the [`VkPhysicalDeviceVulkan12Features`] structure is included in the `next` chain of the
/// [`VkPhysicalDeviceFeatures2`] structure passed to [`VkGetPhysicalDeviceFeatures2`], it is
/// filled in to indicate whether each corresponding feature is supported. If the application
/// wishes to use a [`VkDevice`] with any features described by
/// [`VkPhysicalDeviceVulkan12Features`], it must add an instance of the structure, with the
/// desired feature members set to [`VK_TRUE`], to the `next` chain of [`VkDeviceCreateInfo`] when
/// creating the [`VkDevice`].
///
/// Provided by [`VK_VERSION_1_2`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceVulkan12Features {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PhysicalDeviceVulkan12Features`]
    pub r#type: VkStructureType,

    /// `next` is [`null_mut`] or a pointer to a structure extending this structure.
    pub next: *mut c_void,

    /// `sampler_mirror_clamp_to_edge` indicates whether the implementation supports the
    /// [`VkSamplerAddressMode::MirrorClampToEdge`] sampler address mode. If this feature is not
    /// enabled, the [`VkSamplerAddressMode::MirrorClampToEdge`] sampler address mode must not be
    /// used.
    pub sampler_mirror_clamp_to_edge: VkBool32,

    /// `draw_indirect_count` indicates whether the implementation supports the
    /// [`VkCmdDrawIndirectCount`] and [`VkCmdDrawIndexedIndirectCount`] functions. If this feature
    /// is not enabled, these functions must not be used.
    pub draw_indirect_count: VkBool32,

    /// `storage_buffer_8_bit_access` indicates whether objects in the `StorageBuffer`,
    /// `ShaderRecordBufferKHR`, or `PhysicalStorageBuffer` storage class with the `Block`
    /// decoration can have 8-bit integer members. If this feature is not enabled, 8-bit integer
    /// members must not be used in such objects unless the `shader_untyped_pointer` feature is
    /// enabled and they are accessed in 32-bit multiples or 16-bit multiples if the
    /// `storage_buffer_16_bit_access` feature is enabled. This also indicates whether shader
    /// modules can declare the `StorageBuffer8BitAccess` capability.
    pub storage_buffer_8_bit_access: VkBool32,

    /// `uniform_and_storage_buffer_8_bit_access` indicates whether objects in the `Uniform`
    /// storage class with the `Block` decoration can have 8-bit integer members. If this feature
    /// is not enabled, 8-bit integer members must not be used in such objects unless the
    /// `shader_untyped_pointers` feature is enabled and they are accessed in 32-bit multiples or
    /// 16-bit multiples if the `uniform_and_storage_buffer_16_bit_access` feature is enabled. This
    /// also indicates whether shader modules can declare the `UniformAndStorageBuffer8BitAccess`
    /// capability.
    pub uniform_and_storage_buffer_8_bit_access: VkBool32,

    /// `storage_push_constant_8` indicates whether objects in the `PushConstant` storage class can
    /// have 8-bit integer members. If this feature is not enabled, 8-bit integer members must not
    /// be used in such objects unless the `shader_untyped_pointers` feature is enabled and they
    /// are accessed in 32-bit multiples or 16-bit multiples if the `storage_push_constant_16`
    /// feature is enabled. This also indicates whether shader modules can declare the
    /// `StoragePushConstant8` capability.
    pub storage_push_constant_8: VkBool32,

    /// `shader_buffer_int64_atomics` indicates whether shaders can perform 64-bit unsigned and
    /// signed integer atomic operations on buffers.
    pub shader_buffer_int64_atomics: VkBool32,

    /// `shader_shared_int64_atomics` indicates whether shaders can perform 64-bit unsigned and
    /// signed integer atomic operations on shared and payload memory.
    pub shader_shared_int64_atomics: VkBool32,

    /// `shader_float_16` indicates whether 16-bit floats (halfs) are supported in shader code.
    /// This also indicates whether shader modules can declare the `Float16` capability. However,
    /// this only enables a subset of the storage classes that SPIR-V allows for the `Float16`
    /// SPIR-V capability: Declaring and using 16-bit floats in the `Private`, `Workgroup` (for
    /// non-`Block` variables), and `Function` storage classes is enabled, while declaring them in
    /// the interface storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`,
    /// `Input`, `Output`, and `PushConstant`) is not enabled.
    pub shader_float_16: VkBool32,

    /// `shader_int_8` indicates whether 8-bit integers (signed and unsigned) are supported in
    /// shader code. This also indicates whether shader modules can declare the `Int8` capability.
    /// However, this only enables a subset of the storage classes that SPIR-V allows for the
    /// `Int8` SPIR-V capability: Declaring and using 8-bit integers in the `Private`, `Workgroup`
    /// (for non-`Block` variables), and `Function` storage classes is enabled, while declaring
    /// them in the interface storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`,
    /// `Input`, `Output`, and `PushConstant`) is not enabled.
    pub shader_int_8: VkBool32,

    /// `descriptor_indexing` indicates whether the implementation supports the minimum set of
    /// descriptor indexing features. Enabling this feature when [`VkCreateDevice`] is called does
    /// not imply the other minimum descriptor indexing features are also enabled. Those other
    /// descriptor indexing features must be enabled individually as needed by the application.
    pub descriptor_indexing: VkBool32,

    /// `shader_input_attachment_array_dynamic_indexing` indicates whether arrays of input
    /// attachments can be indexed by integer expressions that are dynamically uniform within
    /// either the subgroup or the invocation group in shader code. If this feature is not enabled,
    /// resources with a descriptor type of [`VkDescriptorType::InputAttachment`] must be indexed
    /// only by constant integral expressions when aggregated into arrays in shader code. This also
    /// indicates whether shader modules can declare the `InputAttachmentArrayDynamicIndexing`
    /// capability.
    pub shader_input_attachment_array_dynamic_indexing: VkBool32,

    /// `shader_uniform_texel_buffer_array_dynamic_indexing` indicates whether arrays of uniform
    /// texel buffers can be indexed by integer expressions that are dynamically uniform within
    /// either the subgroup or the invocation group in shader code. If this feature is not enabled,
    /// resources with a descriptor type of [`VkDescriptorType::UniformTexelBuffer`] must be
    /// indexed only by constant integral expressions when aggregated into arrays in shader code.
    /// This also indicates whether shader modules can declare the
    /// `UniformTexelBufferArrayDynamicIndexing` capability.
    pub shader_uniform_texel_buffer_array_dynamic_indexing: VkBool32,

    /// `shader_storage_texel_buffer_array_dynamic_indexing` indicates whether arrays of storage
    /// texel buffers can be indexed by integer expressions that are dynamically uniform within
    /// either the subgroup or the invocation group in shader code. If this feature is not enabled,
    /// resources with a descriptor type of [`VkDescriptorType::StorageTexelBuffer`] must be
    /// indexed only by constant integral expressions when aggregated into arrays in shader code.
    /// This also indicates whether shader modules can declare the
    /// `StorageTexelBufferArrayDynamicIndexing` capability.
    pub shader_storage_texel_buffer_array_dynamic_indexing: VkBool32,

    /// `shader_uniform_buffer_array_non_uniform_indexing` indicates whether arrays of uniform
    /// buffers can be indexed by non-uniform integer expressions in shader code. If this feature
    /// is not enabled, resources with a descriptor type of [`VkDescriptorType::UniformBuffer`] or
    /// [`VkDescriptorType::UniformBufferDynamic`] must not be indexed by non-uniform integer
    /// expressions when aggregated into arrays in shader code. This also indicates whether shader
    /// modules can declare the `UniformBufferArrayNonUniformIndexing` capability.
    pub shader_uniform_buffer_array_non_uniform_indexing: VkBool32,

    /// `shader_sampled_image_array_non_uniform_indexing` indicates whether arrays of samplers or
    /// sampled images can be indexed by non-uniform integer expressions in shader code. If this
    /// feature is not enabled, resources with a descriptor type of [`VkDescriptorType::Sampler`],
    /// [`VkDescriptorType::CombinedImageSampler`], or [`VkDescriptorType::SampledImage`] must not
    /// be indexed by non-uniform integer expressions when aggregated into arrays in shader code.
    /// This also indicates whether shader modules can declare the
    /// `SampledImageArrayNonUniformIndexing` capability.
    pub shader_sampled_image_array_non_uniform_indexing: VkBool32,

    /// `shader_storage_buffer_array_non_uniform_indexing` indicates whether arrays of storage
    /// buffers can be indexed by non-uniform integer expressions in shader code. If this feature
    /// is not enabled, resources with a descriptor type of [`VkDescriptorType::StorageBuffer`] or
    /// [`VkDescriptorType::StorageBufferDynamic`] must not be indexed by non-uniform integer
    /// expressions when aggregated into arrays in shader code. This also indicates whether shader
    /// modules can declare the `StorageBufferArrayNonUniformIndexing` capability.
    pub shader_storage_buffer_array_non_uniform_indexing: VkBool32,

    /// `shader_storage_image_array_non_uniform_indexing` indicates whether arrays of storage
    /// images can be indexed by non-uniform integer expressions in shader code. If this feature is
    /// not enabled, resources with a descriptor type of [`VkDescriptorType::StorageImage`] must
    /// not be indexed by non-uniform integer expressions when aggregated into arrays in shader
    /// code. This also indicates whether shader modules can declare the
    /// `StorageImageArrayNonUniformIndexing` capability.
    pub shader_storage_image_array_non_uniform_indexing: VkBool32,

    /// `shader_input_attachment_array_non_uniform_indexing` indicates whether arrays of input
    /// attachments can be indexed by non-uniform integer expressions in shader code. If this
    /// feature is not enabled, resources with a descriptor type of
    /// [`VkDescriptorType::InputAttachment`] must not be indexed by non-uniform integer
    /// expressions when aggregated into arrays in shader code. This also indicates whether shader
    /// modules can declare the `InputAttachmentArrayNonUniformIndexing` capability.
    pub shader_input_attachment_array_non_uniform_indexing: VkBool32,

    /// `shader_uniform_texel_buffer_array_non_uniform_indexing` indicates whether arrays of
    /// uniform texel buffers can be indexed by non-uniform integer expressions in shader code. If
    /// this feature is not enabled, resources with a descriptor type of
    /// [`VkDescriptorType::UniformTexelBuffer`] must not be indexed by non-uniform integer
    /// expressions when aggregated into arrays in shader code. This also indicates whether shader
    /// modules can declare the `UniformTexelBufferArrayNonUniformIndexing` capability.
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: VkBool32,

    /// `shader_storage_texel_buffer_array_non_uniform_indexing` indicates whether arrays of
    /// storage texel buffers can be indexed by non-uniform integer expressions in shader code. If
    /// this feature is not enabled, resources with a descriptor type of
    /// [`VkDescriptorType::StorageTexelBuffer`] must not be indexed by non-uniform integer
    /// expressions when aggregated into arrays in shader code. This also indicates whether shader
    /// modules can declare the `StorageTexelBufferArrayNonUniformIndexing` capability.
    pub shader_storage_texel_buffer_array_non_uniform_indexing: VkBool32,

    /// `descriptor_binding_uniform_buffer_update_after_bind` indicates whether the implementation
    /// supports updating uniform buffer descriptors after a set is bound. If this feature is not
    /// enabled, [`VkDescriptorBindingFlag::UpdateAfterBind`] must not be used with
    /// [`VkDescriptorType::UniformBuffer`].
    pub descriptor_binding_uniform_buffer_update_after_bind: VkBool32,

    /// `descriptor_binding_sampled_image_update_after_bind` indicates whether the implementation
    /// supports updating sampled image descriptors after a set is bound. If this feature is not
    /// enabled, [`VkDescriptorBindingFlag::UpdateAfterBind`] must not be used with
    /// [`VkDescriptorType::Sampler`], [`VkDescriptorType::CombinedImageSampler`], or
    /// [`VkDescriptorType::SampledImage`].
    pub descriptor_binding_sampled_image_update_after_bind: VkBool32,

    /// `descriptor_binding_storage_image_update_after_bind` indicates whether the implementation
    /// supports updating storage image descriptors after a set is bound. If this feature is not
    /// enabled, [`VkDescriptorBindingFlag::UpdateAfterBind`] must not be used with
    /// [`VkDescriptorType::StorageImage`].
    pub descriptor_binding_storage_image_update_after_bind: VkBool32,

    /// `descriptor_binding_storage_buffer_update_after_bind` indicates whether the implementation
    /// supports updating storage image descriptors after a set is bound. If this feature is not
    /// enabled, [`VkDescriptorBindingFlag::UpdateAfterBind`] must not be used with
    /// [`VkDescriptorType::StorageBuffer`].
    pub descriptor_binding_storage_buffer_update_after_bind: VkBool32,

    /// `descriptor_binding_uniform_texel_buffer_update_after_bind` indicates whether the
    /// implementation supports updating uniform texel buffer descriptors after a set is bound. If
    /// this feature is not enabled, [`VkDescriptorBindingFlag::UpdateAfterBind`] must not be used
    /// with [`VkDescriptorType::UniformTexelBuffer`].
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: VkBool32,

    /// `descriptor_binding_storage_texel_buffer_update_after_bind` indicates whether the
    /// implementation supports updating storage texel buffer descriptors after a set is bound. If
    /// this feature is not enabled, [`VkDescriptorBindingFlag::UpdateAfterBind`] must not be used
    /// with [`VkDescriptorType::StorageTexelBuffer`].
    pub descriptor_binding_storage_texel_buffer_update_after_bind: VkBool32,

    /// `descriptor_binding_update_unused_while_pending` indicates whether the implementation
    /// supports updating descriptors while the set is in use. If this feature is not enabled,
    /// [`VkDescriptorBindingFlag::UpdateUnusedWhilePending`] must not be used.
    pub descriptor_binding_update_unused_while_pending: VkBool32,

    /// `descriptor_binding_partially_bound` indicates whether the implementation supports
    /// statically using a descriptor set binding in which some descriptors are not valid. If this
    /// feature is not enabled, [`VkDescriptorBindingFlag::PartiallyBound`] must not be used.
    pub descriptor_binding_partially_bound: VkBool32,

    /// `descriptor_binding_variable_descriptor_count` indicates whether the implementation
    /// supports descriptor sets with a variable-sized last binding. If this feature is not
    /// enabled, [`VkDescriptorBindingFlag::VariableDescriptorCount`] must not be used.
    pub descriptor_binding_variable_descriptor_count: VkBool32,

    /// `runtime_descriptor_array` indicates whether the implementation supports the SPIR-V
    /// `RuntimeDescriptorArray` capability. If this feature is not enabled, descriptors must not
    /// be declared in runtime arrays.
    pub runtime_descriptor_array: VkBool32,

    /// `sampler_filter_minmax` indicates whether the implementation supports a minimum set of
    /// required formats supporting min/max filtering as defined by the
    /// `filter_minmax_single_component_formats` property minimum requirements. If this feature is
    /// not enabled, then [`VkSamplerReductionModeCreateInfo`] must only use
    /// [`VkSamplerReductionMode::WeightedAverage`].
    pub sampler_filter_minmax: VkBool32,

    /// `scalar_block_layout` indicates that the implementation supports the layout of resource
    /// blocks in shaders using scalar alignment.
    pub scalar_block_layout: VkBool32,

    /// `imageless_framebuffer` indicates that the implementation supports specifying the image
    /// view for attachments at render pass begin time via [`VkRenderPassAttachmentBeginInfo`].
    pub imageless_framebuffer: VkBool32,

    /// `uniform_buffer_standard_layout` indicates that the implementation supports the same
    /// layouts for uniform buffers as for storage and other kinds of buffers.
    pub uniform_buffer_standard_layout: VkBool32,

    /// `shader_subgroup_extended_types` is a boolean specifying whether subgroup operations can
    /// use 8-bit integer, 16-bit integer, 64-bit integer, 16-bit floating-point, and vectors of
    /// these types in group operations with subgroup scope, if the implementation supports the
    /// types.
    pub shader_subgroup_extended_types: VkBool32,

    /// `separate_depth_stencil_layouts` indicates whether the implementation supports a
    /// [`VkImageMemoryBarrier`] for a depth/stencil image with only one of
    /// [`VkImageAspectFlag::Depth`] or [`VkImageAspectFlag::Stencil`] set, and whether
    /// [`VkImageLayout::DepthAttachmentOptimal`], [`VkImageLayout::DepthReadOnlyOptimal`],
    /// [`VkImageLayout::StencilAttachmentOptimal`], or [`VkImageLayout::StencilReadOnlyOptimal`]
    /// can be used.
    pub separate_depth_stencil_layouts: VkBool32,

    /// `host_query_reset` indicates that the implementation supports resetting queries from the
    /// host with [`VkResetQueryPool`].
    pub host_query_reset: VkBool32,

    /// `timeline_semaphore` indicates whether semaphores created with a [`VkSemaphoreType`] of
    /// [`VkSemaphoreType::Timeline`] are supported.
    pub timeline_semaphore: VkBool32,

    /// `buffer_device_address` indicates that the implementation supports accessing buffer memory
    /// in shaders as storage buffers via an address queried from [`VkGetBufferDeviceAddress`].
    pub buffer_device_address: VkBool32,

    /// `buffer_device_address_capture_replay` indicates that the implementation supports saving
    /// and reusing buffer and device addresses, e.g. for trace capture and replay.
    pub buffer_device_address_capture_replay: VkBool32,

    /// `buffer_device_address_multi_device` indicates that the implementation supports the
    /// `buffer_device_address`, `ray_tracing_pipeline` and `ray_query` features for logical
    /// devices created with multiple physical devices. If this feature is not supported, buffer
    /// and acceleration structure addresses must not be queried on a logical device created with
    /// more than one physical device.
    pub buffer_device_address_multi_device: VkBool32,

    /// `vulkan_memory_model` indicates whether shader modules can declare the `VulkanMemoryModel`
    /// capability.
    pub vulkan_memory_model: VkBool32,

    /// `vulkan_memory_model_device_scope` indicates whether the Vulkan Memory Model can use
    /// `Device` scope synchronization. This also indicates whether shader modules can declare the
    /// `VulkanMemoryModelDeviceScope` capability.
    pub vulkan_memory_model_device_scope: VkBool32,

    /// `vulkan_memory_model_availability_visibility_chains` indicates whether the Vulkan Memory
    /// Model can use availability and visibility chains with more than one element.
    pub vulkan_memory_model_availability_visibility_chains: VkBool32,

    /// `shader_output_viewport_index` indicates whether the implementation supports the
    /// `ShaderViewportIndex` SPIR-V capability enabling variables decorated with the
    /// `ViewportIndex` built-in to be exported from mesh, vertex or tessellation evaluation
    /// shaders. If this feature is not enabled, the `ViewportIndex` built-in decoration must not
    /// be used on outputs in mesh, vertex or tessellation evaluation shaders.
    pub shader_output_viewport_index: VkBool32,

    /// `shader_output_layer` indicates whether the implementation supports the `ShaderLayer`
    /// SPIR-V capability enabling variables decorated with the `Layer` built-in to be exported
    /// from mesh, vertex or tessellation evaluation shaders. If this feature is not enabled, the
    /// `Layer` built-in decoration must not be used on outputs in mesh, vertex or tessellation
    /// evaluation shaders.
    pub shader_output_layer: VkBool32,

    /// `subgroup_broadcast_dynamic_id` is [`VK_TRUE`], the “Id” operand of
    /// `OpGroupNonUniformBroadcast` can be dynamically uniform within a subgroup, and the “Index”
    /// operand of `OpGroupNonUniformQuadBroadcast` can be dynamically uniform within the
    /// derivative group. If it is [`VK_FALSE`], these operands must be constants.
    pub subgroup_broadcast_dynamic_id: VkBool32,
}

const impl Default for VkPhysicalDeviceVulkan12Features {
    fn default() -> Self {
        VkPhysicalDeviceVulkan12Features {
            r#type: VkStructureType::PhysicalDeviceVulkan12Features,
            next: null_mut(),
            sampler_mirror_clamp_to_edge: VK_FALSE,
            draw_indirect_count: VK_FALSE,
            storage_buffer_8_bit_access: VK_FALSE,
            uniform_and_storage_buffer_8_bit_access: VK_FALSE,
            storage_push_constant_8: VK_FALSE,
            shader_buffer_int64_atomics: VK_FALSE,
            shader_shared_int64_atomics: VK_FALSE,
            shader_float_16: VK_FALSE,
            shader_int_8: VK_FALSE,
            descriptor_indexing: VK_FALSE,
            shader_input_attachment_array_dynamic_indexing: VK_FALSE,
            shader_uniform_texel_buffer_array_dynamic_indexing: VK_FALSE,
            shader_storage_texel_buffer_array_dynamic_indexing: VK_FALSE,
            shader_uniform_buffer_array_non_uniform_indexing: VK_FALSE,
            shader_sampled_image_array_non_uniform_indexing: VK_FALSE,
            shader_storage_buffer_array_non_uniform_indexing: VK_FALSE,
            shader_storage_image_array_non_uniform_indexing: VK_FALSE,
            shader_input_attachment_array_non_uniform_indexing: VK_FALSE,
            shader_uniform_texel_buffer_array_non_uniform_indexing: VK_FALSE,
            shader_storage_texel_buffer_array_non_uniform_indexing: VK_FALSE,
            descriptor_binding_uniform_buffer_update_after_bind: VK_FALSE,
            descriptor_binding_sampled_image_update_after_bind: VK_FALSE,
            descriptor_binding_storage_image_update_after_bind: VK_FALSE,
            descriptor_binding_storage_buffer_update_after_bind: VK_FALSE,
            descriptor_binding_uniform_texel_buffer_update_after_bind: VK_FALSE,
            descriptor_binding_storage_texel_buffer_update_after_bind: VK_FALSE,
            descriptor_binding_update_unused_while_pending: VK_FALSE,
            descriptor_binding_partially_bound: VK_FALSE,
            descriptor_binding_variable_descriptor_count: VK_FALSE,
            runtime_descriptor_array: VK_FALSE,
            sampler_filter_minmax: VK_FALSE,
            scalar_block_layout: VK_FALSE,
            imageless_framebuffer: VK_FALSE,
            uniform_buffer_standard_layout: VK_FALSE,
            shader_subgroup_extended_types: VK_FALSE,
            separate_depth_stencil_layouts: VK_FALSE,
            host_query_reset: VK_FALSE,
            timeline_semaphore: VK_FALSE,
            buffer_device_address: VK_FALSE,
            buffer_device_address_capture_replay: VK_FALSE,
            buffer_device_address_multi_device: VK_FALSE,
            vulkan_memory_model: VK_FALSE,
            vulkan_memory_model_device_scope: VK_FALSE,
            vulkan_memory_model_availability_visibility_chains: VK_FALSE,
            shader_output_viewport_index: VK_FALSE,
            shader_output_layer: VK_FALSE,
            subgroup_broadcast_dynamic_id: VK_FALSE,
        }
    }
}

impl NextChainMut for VkPhysicalDeviceVulkan12Features {
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
