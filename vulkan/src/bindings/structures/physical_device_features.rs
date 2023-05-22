use crate::bindings::VkBool32;

#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
    robust_buffer_access: VkBool32,
    full_draw_index_uint32: VkBool32,
    image_cube_array: VkBool32,
    independent_blend: VkBool32,
    geometry_shader: VkBool32,
    tessellation_shader: VkBool32,
    sample_rate_shading: VkBool32,
    dual_src_blend: VkBool32,
    logic_op: VkBool32,
    multi_draw_indirect: VkBool32,
    draw_indirect_first_instance: VkBool32,
    depth_clamp: VkBool32,
    depth_bias_clamp: VkBool32,
    fill_mode_non_solid: VkBool32,
    depth_bounds: VkBool32,
    wide_lines: VkBool32,
    large_points: VkBool32,
    alpha_to_one: VkBool32,
    multi_viewport: VkBool32,
    sampler_anisotropy: VkBool32,
    texture_compression_etc2: VkBool32,
    texture_compression_astc_ldr: VkBool32,
    texture_compression_bc: VkBool32,
    occlusion_query_precise: VkBool32,
    pipeline_statistics_query: VkBool32,
    vertex_pipeline_stores_and_atomics: VkBool32,
    fragment_stores_and_atomics: VkBool32,
    shader_tessellation_and_geometry_point_size: VkBool32,
    shader_image_gather_extended: VkBool32,
    shader_storage_image_extended_formats: VkBool32,
    shader_storage_image_multisample: VkBool32,
    shader_storage_image_read_without_format: VkBool32,
    shader_storage_image_write_without_format: VkBool32,
    shader_uniform_buffer_array_dynamic_indexing: VkBool32,
    shader_sampled_image_array_dynamic_indexing: VkBool32,
    shader_storage_buffer_array_dynamic_indexing: VkBool32,
    shader_storage_image_array_dynamic_indexing: VkBool32,
    shader_clip_distance: VkBool32,
    shader_cull_distance: VkBool32,
    shader_float64: VkBool32,
    shader_int64: VkBool32,
    shader_int16: VkBool32,
    shader_resource_residency: VkBool32,
    shader_resource_min_lod: VkBool32,
    sparse_binding: VkBool32,
    sparse_residency_buffer: VkBool32,
    sparse_residency_image_2d: VkBool32,
    sparse_residency_image_3d: VkBool32,
    sparse_residency_2_samples: VkBool32,
    sparse_residency_4_samples: VkBool32,
    sparse_residency_8_samples: VkBool32,
    sparse_residency_16_samples: VkBool32,
    sparse_residency_aliased: VkBool32,
    variable_multisample_rate: VkBool32,
    inherited_queries: VkBool32,
}

impl VkPhysicalDeviceFeatures {
    pub(crate) const fn null() -> Self {
        VkPhysicalDeviceFeatures {
            robust_buffer_access: 0,
            full_draw_index_uint32: 0,
            image_cube_array: 0,
            independent_blend: 0,
            geometry_shader: 0,
            tessellation_shader: 0,
            sample_rate_shading: 0,
            dual_src_blend: 0,
            logic_op: 0,
            multi_draw_indirect: 0,
            draw_indirect_first_instance: 0,
            depth_clamp: 0,
            depth_bias_clamp: 0,
            fill_mode_non_solid: 0,
            depth_bounds: 0,
            wide_lines: 0,
            large_points: 0,
            alpha_to_one: 0,
            multi_viewport: 0,
            sampler_anisotropy: 0,
            texture_compression_etc2: 0,
            texture_compression_astc_ldr: 0,
            texture_compression_bc: 0,
            occlusion_query_precise: 0,
            pipeline_statistics_query: 0,
            vertex_pipeline_stores_and_atomics: 0,
            fragment_stores_and_atomics: 0,
            shader_tessellation_and_geometry_point_size: 0,
            shader_image_gather_extended: 0,
            shader_storage_image_extended_formats: 0,
            shader_storage_image_multisample: 0,
            shader_storage_image_read_without_format: 0,
            shader_storage_image_write_without_format: 0,
            shader_uniform_buffer_array_dynamic_indexing: 0,
            shader_sampled_image_array_dynamic_indexing: 0,
            shader_storage_buffer_array_dynamic_indexing: 0,
            shader_storage_image_array_dynamic_indexing: 0,
            shader_clip_distance: 0,
            shader_cull_distance: 0,
            shader_float64: 0,
            shader_int64: 0,
            shader_int16: 0,
            shader_resource_residency: 0,
            shader_resource_min_lod: 0,
            sparse_binding: 0,
            sparse_residency_buffer: 0,
            sparse_residency_image_2d: 0,
            sparse_residency_image_3d: 0,
            sparse_residency_2_samples: 0,
            sparse_residency_4_samples: 0,
            sparse_residency_8_samples: 0,
            sparse_residency_16_samples: 0,
            sparse_residency_aliased: 0,
            variable_multisample_rate: 0,
            inherited_queries: 0,
        }
    }

    pub fn robust_buffer_access(&self) -> bool {
        self.robust_buffer_access != 0
    }

    pub fn full_draw_index_uint32(&self) -> bool {
        self.full_draw_index_uint32 != 0
    }

    pub fn image_cube_array(&self) -> bool {
        self.image_cube_array != 0
    }

    pub fn independent_blend(&self) -> bool {
        self.independent_blend != 0
    }

    pub fn geometry_shader(&self) -> bool {
        self.geometry_shader != 0
    }

    pub fn tessellation_shader(&self) -> bool {
        self.tessellation_shader != 0
    }

    pub fn sample_rate_shading(&self) -> bool {
        self.sample_rate_shading != 0
    }

    pub fn dual_src_blend(&self) -> bool {
        self.dual_src_blend != 0
    }

    pub fn logic_op(&self) -> bool {
        self.logic_op != 0
    }

    pub fn multi_draw_indirect(&self) -> bool {
        self.multi_draw_indirect != 0
    }

    pub fn draw_indirect_first_instance(&self) -> bool {
        self.draw_indirect_first_instance != 0
    }

    pub fn depth_clamp(&self) -> bool {
        self.depth_clamp != 0
    }

    pub fn depth_bias_clamp(&self) -> bool {
        self.depth_bias_clamp != 0
    }

    pub fn fill_mode_non_solid(&self) -> bool {
        self.fill_mode_non_solid != 0
    }

    pub fn depth_bounds(&self) -> bool {
        self.depth_bounds != 0
    }

    pub fn wide_lines(&self) -> bool {
        self.wide_lines != 0
    }

    pub fn large_points(&self) -> bool {
        self.large_points != 0
    }

    pub fn alpha_to_one(&self) -> bool {
        self.alpha_to_one != 0
    }

    pub fn multi_viewport(&self) -> bool {
        self.multi_viewport != 0
    }

    pub fn sampler_anisotropy(&self) -> bool {
        self.sampler_anisotropy != 0
    }

    pub fn texture_compression_etc2(&self) -> bool {
        self.texture_compression_etc2 != 0
    }

    pub fn texture_compression_astc_ldr(&self) -> bool {
        self.texture_compression_astc_ldr != 0
    }

    pub fn texture_compression_bc(&self) -> bool {
        self.texture_compression_bc != 0
    }

    pub fn occlusion_query_precise(&self) -> bool {
        self.occlusion_query_precise != 0
    }

    pub fn pipeline_statistics_query(&self) -> bool {
        self.pipeline_statistics_query != 0
    }

    pub fn vertex_pipeline_stores_and_atomics(&self) -> bool {
        self.vertex_pipeline_stores_and_atomics != 0
    }

    pub fn fragment_stores_and_atomics(&self) -> bool {
        self.fragment_stores_and_atomics != 0
    }

    pub fn shader_tessellation_and_geometry_point_size(&self) -> bool {
        self.shader_tessellation_and_geometry_point_size != 0
    }

    pub fn shader_image_gather_extended(&self) -> bool {
        self.shader_image_gather_extended != 0
    }

    pub fn shader_storage_image_extended_formats(&self) -> bool {
        self.shader_storage_image_extended_formats != 0
    }

    pub fn shader_storage_image_multisample(&self) -> bool {
        self.shader_storage_image_multisample != 0
    }

    pub fn shader_storage_image_read_without_format(&self) -> bool {
        self.shader_storage_image_read_without_format != 0
    }

    pub fn shader_storage_image_write_without_format(&self) -> bool {
        self.shader_storage_image_write_without_format != 0
    }

    pub fn shader_uniform_buffer_array_dynamic_indexing(&self) -> bool {
        self.shader_uniform_buffer_array_dynamic_indexing != 0
    }

    pub fn shader_sampled_image_array_dynamic_indexing(&self) -> bool {
        self.shader_sampled_image_array_dynamic_indexing != 0
    }

    pub fn shader_storage_buffer_array_dynamic_indexing(&self) -> bool {
        self.shader_storage_buffer_array_dynamic_indexing != 0
    }

    pub fn shader_storage_image_array_dynamic_indexing(&self) -> bool {
        self.shader_storage_image_array_dynamic_indexing != 0
    }

    pub fn shader_clip_distance(&self) -> bool {
        self.shader_clip_distance != 0
    }

    pub fn shader_cull_distance(&self) -> bool {
        self.shader_cull_distance != 0
    }

    pub fn shader_float64(&self) -> bool {
        self.shader_float64 != 0
    }

    pub fn shader_int64(&self) -> bool {
        self.shader_int64 != 0
    }

    pub fn shader_int16(&self) -> bool {
        self.shader_int16 != 0
    }

    pub fn shader_resource_residency(&self) -> bool {
        self.shader_resource_residency != 0
    }

    pub fn shader_resource_min_lod(&self) -> bool {
        self.shader_resource_min_lod != 0
    }

    pub fn sparse_binding(&self) -> bool {
        self.sparse_binding != 0
    }

    pub fn sparse_residency_buffer(&self) -> bool {
        self.sparse_residency_buffer != 0
    }

    pub fn sparse_residency_image_2d(&self) -> bool {
        self.sparse_residency_image_2d != 0
    }

    pub fn sparse_residency_image_3d(&self) -> bool {
        self.sparse_residency_image_3d != 0
    }

    pub fn sparse_residency_2_samples(&self) -> bool {
        self.sparse_residency_2_samples != 0
    }

    pub fn sparse_residency_4_samples(&self) -> bool {
        self.sparse_residency_4_samples != 0
    }

    pub fn sparse_residency_8_samples(&self) -> bool {
        self.sparse_residency_8_samples != 0
    }

    pub fn sparse_residency_16_samples(&self) -> bool {
        self.sparse_residency_16_samples != 0
    }

    pub fn sparse_residency_aliased(&self) -> bool {
        self.sparse_residency_aliased != 0
    }

    pub fn variable_multisample_rate(&self) -> bool {
        self.variable_multisample_rate != 0
    }

    pub fn inherited_queries(&self) -> bool {
        self.inherited_queries != 0
    }
}
