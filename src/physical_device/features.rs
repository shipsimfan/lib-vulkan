use crate::VkPhysicalDeviceFeatures;

pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: bool,
    pub full_draw_index_uint_32: bool,
    pub image_cube_array: bool,
    pub independent_blend: bool,
    pub geometry_shader: bool,
    pub tessellation_shader: bool,
    pub sample_rate_shading: bool,
    pub dual_src_blend: bool,
    pub logic_op: bool,
    pub multi_draw_indirect: bool,
    pub draw_indirect_first_instance: bool,
    pub depth_clamp: bool,
    pub depth_bias_clamp: bool,
    pub fill_mode_non_solid: bool,
    pub depth_bounds: bool,
    pub wide_lines: bool,
    pub large_points: bool,
    pub alpha_to_one: bool,
    pub multi_viewport: bool,
    pub sampler_anisotropy: bool,
    pub texture_compression_etc_2: bool,
    pub texture_compression_astc_ldr: bool,
    pub texture_compression_bc: bool,
    pub occlusion_query_precise: bool,
    pub pipeline_statistics_query: bool,
    pub vertex_pipeline_stores_and_atomics: bool,
    pub fragment_stores_and_atomics: bool,
    pub shader_tessellation_and_geometry_point_size: bool,
    pub shader_image_gather_extended: bool,
    pub shader_storage_image_extended_formats: bool,
    pub shader_storage_image_multisample: bool,
    pub shader_storage_image_read_without_format: bool,
    pub shader_storage_image_write_without_format: bool,
    pub shader_uniform_buffer_array_dynamic_indexing: bool,
    pub shader_sampled_image_array_dynamic_indexing: bool,
    pub shader_storage_buffer_array_dynamic_indexing: bool,
    pub shader_storage_image_array_dynamic_indexing: bool,
    pub shader_clip_distance: bool,
    pub shader_cull_distance: bool,
    pub shader_float_64: bool,
    pub shader_int_64: bool,
    pub shader_int_16: bool,
    pub shader_resource_residency: bool,
    pub shader_resource_min_lod: bool,
    pub sparse_binding: bool,
    pub sparse_residency_buffer: bool,
    pub sparse_residency_image_2d: bool,
    pub sparse_residency_image_3d: bool,
    pub sparse_residency_2_samples: bool,
    pub sparse_residency_4_samples: bool,
    pub sparse_residency_8_samples: bool,
    pub sparse_residency_16_samples: bool,
    pub sparse_residency_aliased: bool,
    pub variable_multisample_rate: bool,
    pub inherited_queries: bool,
}

impl From<VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
    fn from(features: VkPhysicalDeviceFeatures) -> Self {
        PhysicalDeviceFeatures {
            robust_buffer_access: features.robust_buffer_access != 0,
            full_draw_index_uint_32: features.full_draw_index_uint_32 != 0,
            image_cube_array: features.image_cube_array != 0,
            independent_blend: features.independent_blend != 0,
            geometry_shader: features.geometry_shader != 0,
            tessellation_shader: features.tessellation_shader != 0,
            sample_rate_shading: features.sample_rate_shading != 0,
            dual_src_blend: features.dual_src_blend != 0,
            logic_op: features.logic_op != 0,
            multi_draw_indirect: features.multi_draw_indirect != 0,
            draw_indirect_first_instance: features.draw_indirect_first_instance != 0,
            depth_clamp: features.depth_clamp != 0,
            depth_bias_clamp: features.depth_bias_clamp != 0,
            fill_mode_non_solid: features.fill_mode_non_solid != 0,
            depth_bounds: features.depth_bounds != 0,
            wide_lines: features.wide_lines != 0,
            large_points: features.large_points != 0,
            alpha_to_one: features.alpha_to_one != 0,
            multi_viewport: features.multi_viewport != 0,
            sampler_anisotropy: features.sampler_anisotropy != 0,
            texture_compression_etc_2: features.texture_compression_etc_2 != 0,
            texture_compression_astc_ldr: features.texture_compression_astc_ldr != 0,
            texture_compression_bc: features.texture_compression_bc != 0,
            occlusion_query_precise: features.occlusion_query_precise != 0,
            pipeline_statistics_query: features.pipeline_statistics_query != 0,
            vertex_pipeline_stores_and_atomics: features.vertex_pipeline_stores_and_atomics != 0,
            fragment_stores_and_atomics: features.fragment_stores_and_atomics != 0,
            shader_tessellation_and_geometry_point_size: features
                .shader_tessellation_and_geometry_point_size
                != 0,
            shader_image_gather_extended: features.shader_image_gather_extended != 0,
            shader_storage_image_extended_formats: features.shader_storage_image_extended_formats
                != 0,
            shader_storage_image_multisample: features.shader_storage_image_multisample != 0,
            shader_storage_image_read_without_format: features
                .shader_storage_image_read_without_format
                != 0,
            shader_storage_image_write_without_format: features
                .shader_storage_image_write_without_format
                != 0,
            shader_uniform_buffer_array_dynamic_indexing: features
                .shader_uniform_buffer_array_dynamic_indexing
                != 0,
            shader_sampled_image_array_dynamic_indexing: features
                .shader_sampled_image_array_dynamic_indexing
                != 0,
            shader_storage_buffer_array_dynamic_indexing: features
                .shader_storage_buffer_array_dynamic_indexing
                != 0,
            shader_storage_image_array_dynamic_indexing: features
                .shader_storage_image_array_dynamic_indexing
                != 0,
            shader_clip_distance: features.shader_clip_distance != 0,
            shader_cull_distance: features.shader_cull_distance != 0,
            shader_float_64: features.shader_float_64 != 0,
            shader_int_64: features.shader_int_64 != 0,
            shader_int_16: features.shader_int_16 != 0,
            shader_resource_residency: features.shader_resource_residency != 0,
            shader_resource_min_lod: features.shader_resource_min_lod != 0,
            sparse_binding: features.sparse_binding != 0,
            sparse_residency_buffer: features.sparse_residency_buffer != 0,
            sparse_residency_image_2d: features.sparse_residency_image_2d != 0,
            sparse_residency_image_3d: features.sparse_residency_image_3d != 0,
            sparse_residency_2_samples: features.sparse_residency_2_samples != 0,
            sparse_residency_4_samples: features.sparse_residency_4_samples != 0,
            sparse_residency_8_samples: features.sparse_residency_8_samples != 0,
            sparse_residency_16_samples: features.sparse_residency_16_samples != 0,
            sparse_residency_aliased: features.sparse_residency_aliased != 0,
            variable_multisample_rate: features.variable_multisample_rate != 0,
            inherited_queries: features.inherited_queries != 0,
        }
    }
}

impl Into<VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
    fn into(self) -> VkPhysicalDeviceFeatures {
        VkPhysicalDeviceFeatures {
            robust_buffer_access: self.robust_buffer_access as _,
            full_draw_index_uint_32: self.full_draw_index_uint_32 as _,
            image_cube_array: self.image_cube_array as _,
            independent_blend: self.independent_blend as _,
            geometry_shader: self.geometry_shader as _,
            tessellation_shader: self.tessellation_shader as _,
            sample_rate_shading: self.sample_rate_shading as _,
            dual_src_blend: self.dual_src_blend as _,
            logic_op: self.logic_op as _,
            multi_draw_indirect: self.multi_draw_indirect as _,
            draw_indirect_first_instance: self.draw_indirect_first_instance as _,
            depth_clamp: self.depth_clamp as _,
            depth_bias_clamp: self.depth_bias_clamp as _,
            fill_mode_non_solid: self.fill_mode_non_solid as _,
            depth_bounds: self.depth_bounds as _,
            wide_lines: self.wide_lines as _,
            large_points: self.large_points as _,
            alpha_to_one: self.alpha_to_one as _,
            multi_viewport: self.multi_viewport as _,
            sampler_anisotropy: self.sampler_anisotropy as _,
            texture_compression_etc_2: self.texture_compression_etc_2 as _,
            texture_compression_astc_ldr: self.texture_compression_astc_ldr as _,
            texture_compression_bc: self.texture_compression_bc as _,
            occlusion_query_precise: self.occlusion_query_precise as _,
            pipeline_statistics_query: self.pipeline_statistics_query as _,
            vertex_pipeline_stores_and_atomics: self.vertex_pipeline_stores_and_atomics as _,
            fragment_stores_and_atomics: self.fragment_stores_and_atomics as _,
            shader_tessellation_and_geometry_point_size: self
                .shader_tessellation_and_geometry_point_size
                as _,
            shader_image_gather_extended: self.shader_image_gather_extended as _,
            shader_storage_image_extended_formats: self.shader_storage_image_extended_formats as _,
            shader_storage_image_multisample: self.shader_storage_image_multisample as _,
            shader_storage_image_read_without_format: self.shader_storage_image_read_without_format
                as _,
            shader_storage_image_write_without_format: self
                .shader_storage_image_write_without_format
                as _,
            shader_uniform_buffer_array_dynamic_indexing: self
                .shader_uniform_buffer_array_dynamic_indexing
                as _,
            shader_sampled_image_array_dynamic_indexing: self
                .shader_sampled_image_array_dynamic_indexing
                as _,
            shader_storage_buffer_array_dynamic_indexing: self
                .shader_storage_buffer_array_dynamic_indexing
                as _,
            shader_storage_image_array_dynamic_indexing: self
                .shader_storage_image_array_dynamic_indexing
                as _,
            shader_clip_distance: self.shader_clip_distance as _,
            shader_cull_distance: self.shader_cull_distance as _,
            shader_float_64: self.shader_float_64 as _,
            shader_int_64: self.shader_int_64 as _,
            shader_int_16: self.shader_int_16 as _,
            shader_resource_residency: self.shader_resource_residency as _,
            shader_resource_min_lod: self.shader_resource_min_lod as _,
            sparse_binding: self.sparse_binding as _,
            sparse_residency_buffer: self.sparse_residency_buffer as _,
            sparse_residency_image_2d: self.sparse_residency_image_2d as _,
            sparse_residency_image_3d: self.sparse_residency_image_3d as _,
            sparse_residency_2_samples: self.sparse_residency_2_samples as _,
            sparse_residency_4_samples: self.sparse_residency_4_samples as _,
            sparse_residency_8_samples: self.sparse_residency_8_samples as _,
            sparse_residency_16_samples: self.sparse_residency_16_samples as _,
            sparse_residency_aliased: self.sparse_residency_aliased as _,
            variable_multisample_rate: self.variable_multisample_rate as _,
            inherited_queries: self.inherited_queries as _,
        }
    }
}
