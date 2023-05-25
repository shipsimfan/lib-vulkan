use crate::{VkBool32, VkDeviceSize, VkSampleCountFlags};

#[repr(C)]
#[derive(Default)]
pub(crate) struct VkPhysicalDeviceLimits {
    pub(crate) max_image_dimension_1d: u32,
    pub(crate) max_image_dimension_2d: u32,
    pub(crate) max_image_dimension_3d: u32,
    pub(crate) max_image_dimension_cube: u32,
    pub(crate) max_image_array_layers: u32,
    pub(crate) max_texel_buffer_elements: u32,
    pub(crate) max_uniform_buffer_range: u32,
    pub(crate) max_storage_buffer_range: u32,
    pub(crate) max_push_constants_size: u32,
    pub(crate) max_memory_allocation_count: u32,
    pub(crate) max_sampler_allocation_count: u32,
    pub(crate) buffer_image_granularity: VkDeviceSize,
    pub(crate) sparse_address_space_size: VkDeviceSize,
    pub(crate) max_bound_descriptor_sets: u32,
    pub(crate) max_per_stage_descriptor_samplers: u32,
    pub(crate) max_per_stage_descriptor_uniform_buffers: u32,
    pub(crate) max_per_stage_descriptor_storage_buffers: u32,
    pub(crate) max_per_stage_descriptor_sampled_images: u32,
    pub(crate) max_per_stage_descriptor_storage_images: u32,
    pub(crate) max_per_stage_descriptor_input_attachments: u32,
    pub(crate) max_per_stage_resources: u32,
    pub(crate) max_descriptor_set_samplers: u32,
    pub(crate) max_descriptor_set_uniform_buffers: u32,
    pub(crate) max_descriptor_set_uniform_buffers_dynamic: u32,
    pub(crate) max_descriptor_set_storage_buffers: u32,
    pub(crate) max_descriptor_set_storage_buffers_dynamic: u32,
    pub(crate) max_descriptor_set_sampled_images: u32,
    pub(crate) max_descriptor_set_storage_images: u32,
    pub(crate) max_descriptor_set_input_attachments: u32,
    pub(crate) max_vertex_input_attributes: u32,
    pub(crate) max_vertex_input_bindings: u32,
    pub(crate) max_vertex_input_attribute_offset: u32,
    pub(crate) max_vertex_input_binding_stride: u32,
    pub(crate) max_vertex_output_components: u32,
    pub(crate) max_tessellation_generation_level: u32,
    pub(crate) max_tessellation_patch_size: u32,
    pub(crate) max_tessellation_control_per_vertex_input_components: u32,
    pub(crate) max_tessellation_control_per_vertex_output_components: u32,
    pub(crate) max_tessellation_control_per_patch_output_components: u32,
    pub(crate) max_tessellation_control_total_output_components: u32,
    pub(crate) max_tessellation_evaluation_input_components: u32,
    pub(crate) max_tessellation_evaluation_output_components: u32,
    pub(crate) max_geometry_shader_invocations: u32,
    pub(crate) max_geometry_input_components: u32,
    pub(crate) max_geometry_output_components: u32,
    pub(crate) max_geometry_output_vertices: u32,
    pub(crate) max_geometry_total_output_components: u32,
    pub(crate) max_fragment_input_components: u32,
    pub(crate) max_fragment_output_attachments: u32,
    pub(crate) max_fragment_dual_src_attachments: u32,
    pub(crate) max_fragment_combined_output_resources: u32,
    pub(crate) max_compute_shared_memory_size: u32,
    pub(crate) max_compute_work_group_count: [u32; 3],
    pub(crate) max_compute_work_group_invocations: u32,
    pub(crate) max_compute_work_group_size: [u32; 3],
    pub(crate) sub_pixel_precision_bits: u32,
    pub(crate) sub_texel_precision_bits: u32,
    pub(crate) mipmap_precision_bits: u32,
    pub(crate) max_draw_indexed_index_value: u32,
    pub(crate) max_draw_indirect_count: u32,
    pub(crate) max_sampler_lod_bias: f32,
    pub(crate) max_sampler_anisotropy: f32,
    pub(crate) max_viewports: u32,
    pub(crate) max_viewport_dimensions: [u32; 2],
    pub(crate) viewport_bounds_range: [f32; 2],
    pub(crate) viewport_sub_pixel_bits: u32,
    pub(crate) min_memory_map_alignment: usize,
    pub(crate) min_texel_buffer_offset_alignment: VkDeviceSize,
    pub(crate) min_uniform_buffer_offset_alignment: VkDeviceSize,
    pub(crate) min_storage_buffer_offset_alignment: VkDeviceSize,
    pub(crate) min_texel_offset: i32,
    pub(crate) max_texel_offset: u32,
    pub(crate) min_texel_gather_offset: i32,
    pub(crate) max_texel_gather_offset: u32,
    pub(crate) min_interpolation_offset: f32,
    pub(crate) max_interpolation_offset: f32,
    pub(crate) sub_pixel_interpolation_offset_bits: u32,
    pub(crate) max_framebuffer_width: u32,
    pub(crate) max_framebuffer_height: u32,
    pub(crate) max_framebuffer_layers: u32,
    pub(crate) framebuffer_color_sample_counts: VkSampleCountFlags,
    pub(crate) framebuffer_depth_sample_counts: VkSampleCountFlags,
    pub(crate) framebuffer_stencil_sample_counts: VkSampleCountFlags,
    pub(crate) framebuffer_no_attachments_sample_counts: VkSampleCountFlags,
    pub(crate) max_color_attachments: u32,
    pub(crate) sampled_image_color_sample_counts: VkSampleCountFlags,
    pub(crate) sampled_image_integer_sample_counts: VkSampleCountFlags,
    pub(crate) sampled_image_depth_sample_counts: VkSampleCountFlags,
    pub(crate) sampled_image_stencil_sample_counts: VkSampleCountFlags,
    pub(crate) storage_image_sample_counts: VkSampleCountFlags,
    pub(crate) max_sample_mask_words: u32,
    pub(crate) timestamp_compute_and_graphics: VkBool32,
    pub(crate) timestamp_period: f32,
    pub(crate) max_clip_distances: u32,
    pub(crate) max_cull_distances: u32,
    pub(crate) max_combined_clip_and_cull_distances: u32,
    pub(crate) discrete_queue_priorities: u32,
    pub(crate) point_size_range: [f32; 2],
    pub(crate) line_width_range: [f32; 2],
    pub(crate) point_size_granularity: f32,
    pub(crate) line_width_granularity: f32,
    pub(crate) strict_lines: VkBool32,
    pub(crate) standard_sample_locations: VkBool32,
    pub(crate) optimal_buffer_copy_offset_alignment: VkDeviceSize,
    pub(crate) optimal_buffer_copy_row_pitch_alignment: VkDeviceSize,
    pub(crate) non_coherent_atom_size: VkDeviceSize,
}
