use super::VkSampleCountFlags;
use crate::bindings::{VkBool32, VkDeviceSize};

#[repr(C)]
pub struct VkPhysicalDeviceLimits {
    max_image_dimension_1d: u32,
    max_image_dimension_2d: u32,
    max_image_dimension_3d: u32,
    max_image_dimension_cube: u32,
    max_image_array_layers: u32,
    max_texel_buffer_elements: u32,
    max_uniform_buffer_range: u32,
    max_storage_buffer_range: u32,
    max_push_constants_size: u32,
    max_memory_allocation_count: u32,
    max_sampler_allocation_count: u32,
    buffer_image_granularity: VkDeviceSize,
    sparse_address_space_size: VkDeviceSize,
    max_bound_descriptor_sets: u32,
    max_per_stage_descriptor_samplers: u32,
    max_per_stage_descriptor_uniform_buffers: u32,
    max_per_stage_descriptor_storage_buffers: u32,
    max_per_stage_descriptor_sampled_images: u32,
    max_per_stage_descriptor_storage_images: u32,
    max_per_stage_descriptor_input_attachments: u32,
    max_per_stage_resources: u32,
    max_descriptor_set_samplers: u32,
    max_descriptor_set_uniform_buffers: u32,
    max_descriptor_set_uniform_buffers_dynamic: u32,
    max_descriptor_set_storage_buffers: u32,
    max_descriptor_set_storage_buffers_dynamic: u32,
    max_descriptor_set_sampled_images: u32,
    max_descriptor_set_storage_images: u32,
    max_descriptor_set_input_attachments: u32,
    max_vertex_input_attributes: u32,
    max_vertex_input_bindings: u32,
    max_vertex_input_attribute_offset: u32,
    max_vertex_input_binding_stride: u32,
    max_vertex_output_components: u32,
    max_tessellation_generation_level: u32,
    max_tessellation_patch_size: u32,
    max_tessellation_control_per_vertex_input_components: u32,
    max_tessellation_control_per_vertex_output_components: u32,
    max_tessellation_control_per_patch_output_components: u32,
    max_tessellation_control_total_output_components: u32,
    max_tessellation_evaluation_input_components: u32,
    max_tessellation_evaluation_output_components: u32,
    max_geometry_shader_invocations: u32,
    max_geometry_input_components: u32,
    max_geometry_output_components: u32,
    max_geometry_output_vertices: u32,
    max_geometry_total_output_components: u32,
    max_fragment_input_components: u32,
    max_fragment_output_attachments: u32,
    max_fragment_dual_src_attachments: u32,
    max_fragment_combined_output_resources: u32,
    max_compute_shared_memory_size: u32,
    max_compute_work_group_count: [u32; 3],
    max_compute_work_group_invocations: u32,
    max_compute_work_group_size: [u32; 3],
    sub_pixel_precision_bits: u32,
    sub_texel_precision_bits: u32,
    mipmap_precision_bits: u32,
    max_draw_indexed_index_value: u32,
    max_draw_indirect_count: u32,
    max_sampler_lod_bias: f32,
    max_sampler_anisotropy: f32,
    max_viewports: u32,
    max_viewport_dimensions: [u32; 2],
    viewport_bounds_range: [f32; 2],
    viewport_sub_pixel_bits: u32,
    min_memory_map_alignment: usize,
    min_texel_buffer_offset_alignment: VkDeviceSize,
    min_uniform_buffer_offset_alignment: VkDeviceSize,
    min_storage_buffer_offset_alignment: VkDeviceSize,
    min_texel_offset: i32,
    max_texel_offset: u32,
    min_texel_gather_offset: i32,
    max_texel_gather_offset: u32,
    min_interpolation_offset: f32,
    max_interpolation_offset: f32,
    sub_pixel_interpolation_offset_bits: u32,
    max_framebuffer_width: u32,
    max_framebuffer_height: u32,
    max_framebuffer_layers: u32,
    framebuffer_color_sample_counts: VkSampleCountFlags,
    framebuffer_depth_sample_counts: VkSampleCountFlags,
    framebuffer_stencil_sample_counts: VkSampleCountFlags,
    framebuffer_no_attachments_sample_counts: VkSampleCountFlags,
    max_color_attachments: u32,
    sampled_image_color_sample_counts: VkSampleCountFlags,
    sampled_image_integer_sample_counts: VkSampleCountFlags,
    sampled_image_depth_sample_counts: VkSampleCountFlags,
    sampled_image_stencil_sample_counts: VkSampleCountFlags,
    storage_image_sample_counts: VkSampleCountFlags,
    max_sample_mask_words: u32,
    timestamp_compute_and_graphics: VkBool32,
    timestamp_period: f32,
    max_clip_distances: u32,
    max_cull_distances: u32,
    max_combined_clip_and_cull_distances: u32,
    discrete_queue_priorities: u32,
    point_size_range: [f32; 2],
    line_width_range: [f32; 2],
    point_size_granularity: f32,
    line_width_granularity: f32,
    strict_lines: VkBool32,
    standard_sample_locations: VkBool32,
    optimal_buffer_copy_offset_alignment: VkDeviceSize,
    optimal_buffer_copy_row_pitch_alignment: VkDeviceSize,
    non_coherent_atom_size: VkDeviceSize,
}

impl VkPhysicalDeviceLimits {
    pub(crate) const fn null() -> Self {
        VkPhysicalDeviceLimits {
            max_image_dimension_1d: 0,
            max_image_dimension_2d: 0,
            max_image_dimension_3d: 0,
            max_image_dimension_cube: 0,
            max_image_array_layers: 0,
            max_texel_buffer_elements: 0,
            max_uniform_buffer_range: 0,
            max_storage_buffer_range: 0,
            max_push_constants_size: 0,
            max_memory_allocation_count: 0,
            max_sampler_allocation_count: 0,
            buffer_image_granularity: 0,
            sparse_address_space_size: 0,
            max_bound_descriptor_sets: 0,
            max_per_stage_descriptor_samplers: 0,
            max_per_stage_descriptor_uniform_buffers: 0,
            max_per_stage_descriptor_storage_buffers: 0,
            max_per_stage_descriptor_sampled_images: 0,
            max_per_stage_descriptor_storage_images: 0,
            max_per_stage_descriptor_input_attachments: 0,
            max_per_stage_resources: 0,
            max_descriptor_set_samplers: 0,
            max_descriptor_set_uniform_buffers: 0,
            max_descriptor_set_uniform_buffers_dynamic: 0,
            max_descriptor_set_storage_buffers: 0,
            max_descriptor_set_storage_buffers_dynamic: 0,
            max_descriptor_set_sampled_images: 0,
            max_descriptor_set_storage_images: 0,
            max_descriptor_set_input_attachments: 0,
            max_vertex_input_attributes: 0,
            max_vertex_input_bindings: 0,
            max_vertex_input_attribute_offset: 0,
            max_vertex_input_binding_stride: 0,
            max_vertex_output_components: 0,
            max_tessellation_generation_level: 0,
            max_tessellation_patch_size: 0,
            max_tessellation_control_per_vertex_input_components: 0,
            max_tessellation_control_per_vertex_output_components: 0,
            max_tessellation_control_per_patch_output_components: 0,
            max_tessellation_control_total_output_components: 0,
            max_tessellation_evaluation_input_components: 0,
            max_tessellation_evaluation_output_components: 0,
            max_geometry_shader_invocations: 0,
            max_geometry_input_components: 0,
            max_geometry_output_components: 0,
            max_geometry_output_vertices: 0,
            max_geometry_total_output_components: 0,
            max_fragment_input_components: 0,
            max_fragment_output_attachments: 0,
            max_fragment_dual_src_attachments: 0,
            max_fragment_combined_output_resources: 0,
            max_compute_shared_memory_size: 0,
            max_compute_work_group_count: [0; 3],
            max_compute_work_group_invocations: 0,
            max_compute_work_group_size: [0; 3],
            sub_pixel_precision_bits: 0,
            sub_texel_precision_bits: 0,
            mipmap_precision_bits: 0,
            max_draw_indexed_index_value: 0,
            max_draw_indirect_count: 0,
            max_sampler_lod_bias: 0.,
            max_sampler_anisotropy: 0.,
            max_viewports: 0,
            max_viewport_dimensions: [0; 2],
            viewport_bounds_range: [0.; 2],
            viewport_sub_pixel_bits: 0,
            min_memory_map_alignment: 0,
            min_texel_buffer_offset_alignment: 0,
            min_uniform_buffer_offset_alignment: 0,
            min_storage_buffer_offset_alignment: 0,
            min_texel_offset: 0,
            max_texel_offset: 0,
            min_texel_gather_offset: 0,
            max_texel_gather_offset: 0,
            min_interpolation_offset: 0.,
            max_interpolation_offset: 0.,
            sub_pixel_interpolation_offset_bits: 0,
            max_framebuffer_width: 0,
            max_framebuffer_height: 0,
            max_framebuffer_layers: 0,
            framebuffer_color_sample_counts: VkSampleCountFlags::new(&[]),
            framebuffer_depth_sample_counts: VkSampleCountFlags::new(&[]),
            framebuffer_stencil_sample_counts: VkSampleCountFlags::new(&[]),
            framebuffer_no_attachments_sample_counts: VkSampleCountFlags::new(&[]),
            max_color_attachments: 0,
            sampled_image_color_sample_counts: VkSampleCountFlags::new(&[]),
            sampled_image_integer_sample_counts: VkSampleCountFlags::new(&[]),
            sampled_image_depth_sample_counts: VkSampleCountFlags::new(&[]),
            sampled_image_stencil_sample_counts: VkSampleCountFlags::new(&[]),
            storage_image_sample_counts: VkSampleCountFlags::new(&[]),
            max_sample_mask_words: 0,
            timestamp_compute_and_graphics: 0,
            timestamp_period: 0.,
            max_clip_distances: 0,
            max_cull_distances: 0,
            max_combined_clip_and_cull_distances: 0,
            discrete_queue_priorities: 0,
            point_size_range: [0.; 2],
            line_width_range: [0.; 2],
            point_size_granularity: 0.,
            line_width_granularity: 0.,
            strict_lines: 0,
            standard_sample_locations: 0,
            optimal_buffer_copy_offset_alignment: 0,
            optimal_buffer_copy_row_pitch_alignment: 0,
            non_coherent_atom_size: 0,
        }
    }

    pub fn max_image_dimension_1d(&self) -> u32 {
        self.max_image_dimension_1d
    }

    pub fn max_image_dimension_2d(&self) -> u32 {
        self.max_image_dimension_2d
    }

    pub fn max_image_dimension_3d(&self) -> u32 {
        self.max_image_dimension_3d
    }

    pub fn max_image_dimension_cube(&self) -> u32 {
        self.max_image_dimension_cube
    }

    pub fn max_image_array_layers(&self) -> u32 {
        self.max_image_array_layers
    }

    pub fn max_texel_buffer_elements(&self) -> u32 {
        self.max_texel_buffer_elements
    }

    pub fn max_uniform_buffer_range(&self) -> u32 {
        self.max_uniform_buffer_range
    }

    pub fn max_storage_buffer_range(&self) -> u32 {
        self.max_storage_buffer_range
    }

    pub fn max_push_constants_size(&self) -> u32 {
        self.max_push_constants_size
    }

    pub fn max_memory_allocation_count(&self) -> u32 {
        self.max_memory_allocation_count
    }

    pub fn max_sampler_allocation_count(&self) -> u32 {
        self.max_sampler_allocation_count
    }

    pub fn buffer_image_granularity(&self) -> u64 {
        self.buffer_image_granularity
    }

    pub fn sparse_address_space_size(&self) -> u64 {
        self.sparse_address_space_size
    }

    pub fn max_bound_descriptor_sets(&self) -> u32 {
        self.max_bound_descriptor_sets
    }

    pub fn max_per_stage_descriptor_samplers(&self) -> u32 {
        self.max_per_stage_descriptor_samplers
    }

    pub fn max_per_stage_descriptor_uniform_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_uniform_buffers
    }

    pub fn max_per_stage_descriptor_storage_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_storage_buffers
    }

    pub fn max_per_stage_descriptor_sampled_images(&self) -> u32 {
        self.max_per_stage_descriptor_sampled_images
    }

    pub fn max_per_stage_descriptor_storage_images(&self) -> u32 {
        self.max_per_stage_descriptor_storage_images
    }

    pub fn max_per_stage_descriptor_input_attachments(&self) -> u32 {
        self.max_per_stage_descriptor_input_attachments
    }

    pub fn max_per_stage_resources(&self) -> u32 {
        self.max_per_stage_resources
    }

    pub fn max_descriptor_set_samplers(&self) -> u32 {
        self.max_descriptor_set_samplers
    }

    pub fn max_descriptor_set_uniform_buffers(&self) -> u32 {
        self.max_descriptor_set_uniform_buffers
    }

    pub fn max_descriptor_set_uniform_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_uniform_buffers_dynamic
    }

    pub fn max_descriptor_set_storage_buffers(&self) -> u32 {
        self.max_descriptor_set_storage_buffers
    }

    pub fn max_descriptor_set_storage_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_storage_buffers_dynamic
    }

    pub fn max_descriptor_set_sampled_images(&self) -> u32 {
        self.max_descriptor_set_sampled_images
    }

    pub fn max_descriptor_set_storage_images(&self) -> u32 {
        self.max_descriptor_set_storage_images
    }

    pub fn max_descriptor_set_input_attachments(&self) -> u32 {
        self.max_descriptor_set_input_attachments
    }

    pub fn max_vertex_input_attributes(&self) -> u32 {
        self.max_vertex_input_attributes
    }

    pub fn max_vertex_input_bindings(&self) -> u32 {
        self.max_vertex_input_bindings
    }

    pub fn max_vertex_input_attribute_offset(&self) -> u32 {
        self.max_vertex_input_attribute_offset
    }

    pub fn max_vertex_input_binding_stride(&self) -> u32 {
        self.max_vertex_input_binding_stride
    }

    pub fn max_vertex_output_components(&self) -> u32 {
        self.max_vertex_output_components
    }

    pub fn max_tessellation_generation_level(&self) -> u32 {
        self.max_tessellation_generation_level
    }

    pub fn max_tessellation_patch_size(&self) -> u32 {
        self.max_tessellation_patch_size
    }

    pub fn max_tessellation_control_per_vertex_input_components(&self) -> u32 {
        self.max_tessellation_control_per_vertex_input_components
    }

    pub fn max_tessellation_control_per_vertex_output_components(&self) -> u32 {
        self.max_tessellation_control_per_vertex_output_components
    }

    pub fn max_tessellation_control_per_patch_output_components(&self) -> u32 {
        self.max_tessellation_control_per_patch_output_components
    }

    pub fn max_tessellation_control_total_output_components(&self) -> u32 {
        self.max_tessellation_control_total_output_components
    }

    pub fn max_tessellation_evaluation_input_components(&self) -> u32 {
        self.max_tessellation_evaluation_input_components
    }

    pub fn max_tessellation_evaluation_output_components(&self) -> u32 {
        self.max_tessellation_evaluation_output_components
    }

    pub fn max_geometry_shader_invocations(&self) -> u32 {
        self.max_geometry_shader_invocations
    }

    pub fn max_geometry_input_components(&self) -> u32 {
        self.max_geometry_input_components
    }

    pub fn max_geometry_output_components(&self) -> u32 {
        self.max_geometry_output_components
    }

    pub fn max_geometry_output_vertices(&self) -> u32 {
        self.max_geometry_output_vertices
    }

    pub fn max_geometry_total_output_components(&self) -> u32 {
        self.max_geometry_total_output_components
    }

    pub fn max_fragment_input_components(&self) -> u32 {
        self.max_fragment_input_components
    }

    pub fn max_fragment_output_attachments(&self) -> u32 {
        self.max_fragment_output_attachments
    }

    pub fn max_fragment_dual_src_attachments(&self) -> u32 {
        self.max_fragment_dual_src_attachments
    }

    pub fn max_fragment_combined_output_resources(&self) -> u32 {
        self.max_fragment_combined_output_resources
    }

    pub fn max_compute_shared_memory_size(&self) -> u32 {
        self.max_compute_shared_memory_size
    }

    pub fn max_compute_work_group_count(&self) -> [u32; 3] {
        self.max_compute_work_group_count
    }

    pub fn max_compute_work_group_invocations(&self) -> u32 {
        self.max_compute_work_group_invocations
    }

    pub fn max_compute_work_group_size(&self) -> [u32; 3] {
        self.max_compute_work_group_size
    }

    pub fn sub_pixel_precision_bits(&self) -> u32 {
        self.sub_pixel_precision_bits
    }

    pub fn sub_texel_precision_bits(&self) -> u32 {
        self.sub_texel_precision_bits
    }

    pub fn mipmap_precision_bits(&self) -> u32 {
        self.mipmap_precision_bits
    }

    pub fn max_draw_indexed_index_value(&self) -> u32 {
        self.max_draw_indexed_index_value
    }

    pub fn max_draw_indirect_count(&self) -> u32 {
        self.max_draw_indirect_count
    }

    pub fn max_sampler_lod_bias(&self) -> f32 {
        self.max_sampler_lod_bias
    }

    pub fn max_sampler_anisotropy(&self) -> f32 {
        self.max_sampler_anisotropy
    }

    pub fn max_viewports(&self) -> u32 {
        self.max_viewports
    }

    pub fn max_viewport_dimensions(&self) -> [u32; 2] {
        self.max_viewport_dimensions
    }

    pub fn viewport_bounds_range(&self) -> [f32; 2] {
        self.viewport_bounds_range
    }

    pub fn viewport_sub_pixel_bits(&self) -> u32 {
        self.viewport_sub_pixel_bits
    }

    pub fn min_memory_map_alignment(&self) -> usize {
        self.min_memory_map_alignment
    }

    pub fn min_texel_buffer_offset_alignment(&self) -> u64 {
        self.min_texel_buffer_offset_alignment
    }

    pub fn min_uniform_buffer_offset_alignment(&self) -> u64 {
        self.min_uniform_buffer_offset_alignment
    }

    pub fn min_storage_buffer_offset_alignment(&self) -> u64 {
        self.min_storage_buffer_offset_alignment
    }

    pub fn min_texel_offset(&self) -> i32 {
        self.min_texel_offset
    }

    pub fn max_texel_offset(&self) -> u32 {
        self.max_texel_offset
    }

    pub fn min_texel_gather_offset(&self) -> i32 {
        self.min_texel_gather_offset
    }

    pub fn max_texel_gather_offset(&self) -> u32 {
        self.max_texel_gather_offset
    }

    pub fn min_interpolation_offset(&self) -> f32 {
        self.min_interpolation_offset
    }

    pub fn max_interpolation_offset(&self) -> f32 {
        self.max_interpolation_offset
    }

    pub fn sub_pixel_interpolation_offset_bits(&self) -> u32 {
        self.sub_pixel_interpolation_offset_bits
    }

    pub fn max_framebuffer_width(&self) -> u32 {
        self.max_framebuffer_width
    }

    pub fn max_framebuffer_height(&self) -> u32 {
        self.max_framebuffer_height
    }

    pub fn max_framebuffer_layers(&self) -> u32 {
        self.max_framebuffer_layers
    }

    pub fn framebuffer_color_sample_counts(&self) -> VkSampleCountFlags {
        self.framebuffer_color_sample_counts
    }

    pub fn framebuffer_depth_sample_counts(&self) -> VkSampleCountFlags {
        self.framebuffer_depth_sample_counts
    }

    pub fn framebuffer_stencil_sample_counts(&self) -> VkSampleCountFlags {
        self.framebuffer_stencil_sample_counts
    }

    pub fn framebuffer_no_attachments_sample_counts(&self) -> VkSampleCountFlags {
        self.framebuffer_no_attachments_sample_counts
    }

    pub fn max_color_attachments(&self) -> u32 {
        self.max_color_attachments
    }

    pub fn sampled_image_color_sample_counts(&self) -> VkSampleCountFlags {
        self.sampled_image_color_sample_counts
    }

    pub fn sampled_image_integer_sample_counts(&self) -> VkSampleCountFlags {
        self.sampled_image_integer_sample_counts
    }

    pub fn sampled_image_depth_sample_counts(&self) -> VkSampleCountFlags {
        self.sampled_image_depth_sample_counts
    }

    pub fn sampled_image_stencil_sample_counts(&self) -> VkSampleCountFlags {
        self.sampled_image_stencil_sample_counts
    }

    pub fn storage_image_sample_counts(&self) -> VkSampleCountFlags {
        self.storage_image_sample_counts
    }

    pub fn max_sample_mask_words(&self) -> u32 {
        self.max_sample_mask_words
    }

    pub fn timestamp_compute_and_graphics(&self) -> bool {
        self.timestamp_compute_and_graphics != 0
    }

    pub fn timestamp_period(&self) -> f32 {
        self.timestamp_period
    }

    pub fn max_clip_distances(&self) -> u32 {
        self.max_clip_distances
    }

    pub fn max_cull_distances(&self) -> u32 {
        self.max_cull_distances
    }

    pub fn max_combined_clip_and_cull_distances(&self) -> u32 {
        self.max_combined_clip_and_cull_distances
    }

    pub fn discrete_queue_priorities(&self) -> u32 {
        self.discrete_queue_priorities
    }

    pub fn point_size_range(&self) -> [f32; 2] {
        self.point_size_range
    }

    pub fn line_width_range(&self) -> [f32; 2] {
        self.line_width_range
    }

    pub fn point_size_granularity(&self) -> f32 {
        self.point_size_granularity
    }

    pub fn line_width_granularity(&self) -> f32 {
        self.line_width_granularity
    }

    pub fn strict_lines(&self) -> bool {
        self.strict_lines != 0
    }

    pub fn standard_sample_locations(&self) -> bool {
        self.standard_sample_locations != 0
    }

    pub fn optimal_buffer_copy_offset_alignment(&self) -> u64 {
        self.optimal_buffer_copy_offset_alignment
    }

    pub fn optimal_buffer_copy_row_pitch_alignment(&self) -> u64 {
        self.optimal_buffer_copy_row_pitch_alignment
    }

    pub fn non_coherent_atom_size(&self) -> u64 {
        self.non_coherent_atom_size
    }
}
