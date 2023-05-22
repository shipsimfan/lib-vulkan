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
    pub const fn default() -> Self {
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

    pub const fn new(
        robust_buffer_access: bool,
        full_draw_index_uint32: bool,
        image_cube_array: bool,
        independent_blend: bool,
        geometry_shader: bool,
        tessellation_shader: bool,
        sample_rate_shading: bool,
        dual_src_blend: bool,
        logic_op: bool,
        multi_draw_indirect: bool,
        draw_indirect_first_instance: bool,
        depth_clamp: bool,
        depth_bias_clamp: bool,
        fill_mode_non_solid: bool,
        depth_bounds: bool,
        wide_lines: bool,
        large_points: bool,
        alpha_to_one: bool,
        multi_viewport: bool,
        sampler_anisotropy: bool,
        texture_compression_etc2: bool,
        texture_compression_astc_ldr: bool,
        texture_compression_bc: bool,
        occlusion_query_precise: bool,
        pipeline_statistics_query: bool,
        vertex_pipeline_stores_and_atomics: bool,
        fragment_stores_and_atomics: bool,
        shader_tessellation_and_geometry_point_size: bool,
        shader_image_gather_extended: bool,
        shader_storage_image_extended_formats: bool,
        shader_storage_image_multisample: bool,
        shader_storage_image_read_without_format: bool,
        shader_storage_image_write_without_format: bool,
        shader_uniform_buffer_array_dynamic_indexing: bool,
        shader_sampled_image_array_dynamic_indexing: bool,
        shader_storage_buffer_array_dynamic_indexing: bool,
        shader_storage_image_array_dynamic_indexing: bool,
        shader_clip_distance: bool,
        shader_cull_distance: bool,
        shader_float64: bool,
        shader_int64: bool,
        shader_int16: bool,
        shader_resource_residency: bool,
        shader_resource_min_lod: bool,
        sparse_binding: bool,
        sparse_residency_buffer: bool,
        sparse_residency_image_2d: bool,
        sparse_residency_image_3d: bool,
        sparse_residency_2_samples: bool,
        sparse_residency_4_samples: bool,
        sparse_residency_8_samples: bool,
        sparse_residency_16_samples: bool,
        sparse_residency_aliased: bool,
        variable_multisample_rate: bool,
        inherited_queries: VkBool32,
    ) -> Self {
        VkPhysicalDeviceFeatures {
            robust_buffer_access: robust_buffer_access as VkBool32,
            full_draw_index_uint32: full_draw_index_uint32 as VkBool32,
            image_cube_array: image_cube_array as VkBool32,
            independent_blend: independent_blend as VkBool32,
            geometry_shader: geometry_shader as VkBool32,
            tessellation_shader: tessellation_shader as VkBool32,
            sample_rate_shading: sample_rate_shading as VkBool32,
            dual_src_blend: dual_src_blend as VkBool32,
            logic_op: logic_op as VkBool32,
            multi_draw_indirect: multi_draw_indirect as VkBool32,
            draw_indirect_first_instance: draw_indirect_first_instance as VkBool32,
            depth_clamp: depth_clamp as VkBool32,
            depth_bias_clamp: depth_bias_clamp as VkBool32,
            fill_mode_non_solid: fill_mode_non_solid as VkBool32,
            depth_bounds: depth_bounds as VkBool32,
            wide_lines: wide_lines as VkBool32,
            large_points: large_points as VkBool32,
            alpha_to_one: alpha_to_one as VkBool32,
            multi_viewport: multi_viewport as VkBool32,
            sampler_anisotropy: sampler_anisotropy as VkBool32,
            texture_compression_etc2: texture_compression_etc2 as VkBool32,
            texture_compression_astc_ldr: texture_compression_astc_ldr as VkBool32,
            texture_compression_bc: texture_compression_bc as VkBool32,
            occlusion_query_precise: occlusion_query_precise as VkBool32,
            pipeline_statistics_query: pipeline_statistics_query as VkBool32,
            vertex_pipeline_stores_and_atomics: vertex_pipeline_stores_and_atomics as VkBool32,
            fragment_stores_and_atomics: fragment_stores_and_atomics as VkBool32,
            shader_tessellation_and_geometry_point_size: shader_tessellation_and_geometry_point_size
                as VkBool32,
            shader_image_gather_extended: shader_image_gather_extended as VkBool32,
            shader_storage_image_extended_formats: shader_storage_image_extended_formats
                as VkBool32,
            shader_storage_image_multisample: shader_storage_image_multisample as VkBool32,
            shader_storage_image_read_without_format: shader_storage_image_read_without_format
                as VkBool32,
            shader_storage_image_write_without_format: shader_storage_image_write_without_format
                as VkBool32,
            shader_uniform_buffer_array_dynamic_indexing:
                shader_uniform_buffer_array_dynamic_indexing as VkBool32,
            shader_sampled_image_array_dynamic_indexing: shader_sampled_image_array_dynamic_indexing
                as VkBool32,
            shader_storage_buffer_array_dynamic_indexing:
                shader_storage_buffer_array_dynamic_indexing as VkBool32,
            shader_storage_image_array_dynamic_indexing: shader_storage_image_array_dynamic_indexing
                as VkBool32,
            shader_clip_distance: shader_clip_distance as VkBool32,
            shader_cull_distance: shader_cull_distance as VkBool32,
            shader_float64: shader_float64 as VkBool32,
            shader_int64: shader_int64 as VkBool32,
            shader_int16: shader_int16 as VkBool32,
            shader_resource_residency: shader_resource_residency as VkBool32,
            shader_resource_min_lod: shader_resource_min_lod as VkBool32,
            sparse_binding: sparse_binding as VkBool32,
            sparse_residency_buffer: sparse_residency_buffer as VkBool32,
            sparse_residency_image_2d: sparse_residency_image_2d as VkBool32,
            sparse_residency_image_3d: sparse_residency_image_3d as VkBool32,
            sparse_residency_2_samples: sparse_residency_2_samples as VkBool32,
            sparse_residency_4_samples: sparse_residency_4_samples as VkBool32,
            sparse_residency_8_samples: sparse_residency_8_samples as VkBool32,
            sparse_residency_16_samples: sparse_residency_16_samples as VkBool32,
            sparse_residency_aliased: sparse_residency_aliased as VkBool32,
            variable_multisample_rate: variable_multisample_rate as VkBool32,
            inherited_queries: inherited_queries as VkBool32,
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

    pub fn set_robust_buffer_access(&mut self, robust_buffer_access: bool) {
        self.robust_buffer_access = robust_buffer_access as VkBool32;
    }

    pub fn set_full_draw_index_uint32(&mut self, full_draw_index_uint32: bool) {
        self.full_draw_index_uint32 = full_draw_index_uint32 as VkBool32;
    }

    pub fn set_image_cube_array(&mut self, image_cube_array: bool) {
        self.image_cube_array = image_cube_array as VkBool32;
    }

    pub fn set_independent_blend(&mut self, independent_blend: bool) {
        self.independent_blend = independent_blend as VkBool32;
    }

    pub fn set_geometry_shader(&mut self, geometry_shader: bool) {
        self.geometry_shader = geometry_shader as VkBool32;
    }

    pub fn set_tessellation_shader(&mut self, tessellation_shader: bool) {
        self.tessellation_shader = tessellation_shader as VkBool32;
    }

    pub fn set_sample_rate_shading(&mut self, sample_rate_shading: bool) {
        self.sample_rate_shading = sample_rate_shading as VkBool32;
    }

    pub fn set_dual_src_blend(&mut self, dual_src_blend: bool) {
        self.dual_src_blend = dual_src_blend as VkBool32;
    }

    pub fn set_logic_op(&mut self, logic_op: bool) {
        self.logic_op = logic_op as VkBool32;
    }

    pub fn set_multi_draw_indirect(&mut self, multi_draw_indirect: bool) {
        self.multi_draw_indirect = multi_draw_indirect as VkBool32;
    }

    pub fn set_draw_indirect_first_instance(&mut self, draw_indirect_first_instance: bool) {
        self.draw_indirect_first_instance = draw_indirect_first_instance as VkBool32;
    }

    pub fn set_depth_clamp(&mut self, depth_clamp: bool) {
        self.depth_clamp = depth_clamp as VkBool32;
    }

    pub fn set_depth_bias_clamp(&mut self, depth_bias_clamp: bool) {
        self.depth_bias_clamp = depth_bias_clamp as VkBool32;
    }

    pub fn set_fill_mode_non_solid(&mut self, fill_mode_non_solid: bool) {
        self.fill_mode_non_solid = fill_mode_non_solid as VkBool32;
    }

    pub fn set_depth_bounds(&mut self, depth_bounds: bool) {
        self.depth_bounds = depth_bounds as VkBool32;
    }

    pub fn set_wide_lines(&mut self, wide_lines: bool) {
        self.wide_lines = wide_lines as VkBool32;
    }

    pub fn set_large_points(&mut self, large_points: bool) {
        self.large_points = large_points as VkBool32;
    }

    pub fn set_alpha_to_one(&mut self, alpha_to_one: bool) {
        self.alpha_to_one = alpha_to_one as VkBool32;
    }

    pub fn set_multi_viewport(&mut self, multi_viewport: bool) {
        self.multi_viewport = multi_viewport as VkBool32;
    }

    pub fn set_sampler_anisotropy(&mut self, sampler_anisotropy: bool) {
        self.sampler_anisotropy = sampler_anisotropy as VkBool32;
    }

    pub fn set_texture_compression_etc2(&mut self, texture_compression_etc2: bool) {
        self.texture_compression_etc2 = texture_compression_etc2 as VkBool32;
    }

    pub fn set_texture_compression_astc_ldr(&mut self, texture_compression_astc_ldr: bool) {
        self.texture_compression_astc_ldr = texture_compression_astc_ldr as VkBool32;
    }

    pub fn set_texture_compression_bc(&mut self, texture_compression_bc: bool) {
        self.texture_compression_bc = texture_compression_bc as VkBool32;
    }

    pub fn set_occlusion_query_precise(&mut self, occlusion_query_precise: bool) {
        self.occlusion_query_precise = occlusion_query_precise as VkBool32;
    }

    pub fn set_pipeline_statistics_query(&mut self, pipeline_statistics_query: bool) {
        self.pipeline_statistics_query = pipeline_statistics_query as VkBool32;
    }

    pub fn set_vertex_pipeline_stores_and_atomics(
        &mut self,
        vertex_pipeline_stores_and_atomics: bool,
    ) {
        self.vertex_pipeline_stores_and_atomics = vertex_pipeline_stores_and_atomics as VkBool32;
    }

    pub fn set_fragment_stores_and_atomics(&mut self, fragment_stores_and_atomics: bool) {
        self.fragment_stores_and_atomics = fragment_stores_and_atomics as VkBool32;
    }

    pub fn set_shader_tessellation_and_geometry_point_size(
        &mut self,
        shader_tessellation_and_geometry_point_size: bool,
    ) {
        self.shader_tessellation_and_geometry_point_size =
            shader_tessellation_and_geometry_point_size as VkBool32;
    }

    pub fn set_shader_image_gather_extended(&mut self, shader_image_gather_extended: bool) {
        self.shader_image_gather_extended = shader_image_gather_extended as VkBool32;
    }

    pub fn set_shader_storage_image_extended_formats(
        &mut self,
        shader_storage_image_extended_formats: bool,
    ) {
        self.shader_storage_image_extended_formats =
            shader_storage_image_extended_formats as VkBool32;
    }

    pub fn set_shader_storage_image_multisample(&mut self, shader_storage_image_multisample: bool) {
        self.shader_storage_image_multisample = shader_storage_image_multisample as VkBool32;
    }

    pub fn set_shader_storage_image_read_without_format(
        &mut self,
        shader_storage_image_read_without_format: bool,
    ) {
        self.shader_storage_image_read_without_format =
            shader_storage_image_read_without_format as VkBool32;
    }

    pub fn set_shader_storage_image_write_without_format(
        &mut self,
        shader_storage_image_write_without_format: bool,
    ) {
        self.shader_storage_image_write_without_format =
            shader_storage_image_write_without_format as VkBool32;
    }

    pub fn set_shader_uniform_buffer_array_dynamic_indexing(
        &mut self,
        shader_uniform_buffer_array_dynamic_indexing: bool,
    ) {
        self.shader_uniform_buffer_array_dynamic_indexing =
            shader_uniform_buffer_array_dynamic_indexing as VkBool32;
    }

    pub fn set_shader_sampled_image_array_dynamic_indexing(
        &mut self,
        shader_sampled_image_array_dynamic_indexing: bool,
    ) {
        self.shader_sampled_image_array_dynamic_indexing =
            shader_sampled_image_array_dynamic_indexing as VkBool32;
    }

    pub fn set_shader_storage_buffer_array_dynamic_indexing(
        &mut self,
        shader_storage_buffer_array_dynamic_indexing: bool,
    ) {
        self.shader_storage_buffer_array_dynamic_indexing =
            shader_storage_buffer_array_dynamic_indexing as VkBool32;
    }

    pub fn set_shader_storage_image_array_dynamic_indexing(
        &mut self,
        shader_storage_image_array_dynamic_indexing: bool,
    ) {
        self.shader_storage_image_array_dynamic_indexing =
            shader_storage_image_array_dynamic_indexing as VkBool32;
    }

    pub fn set_shader_clip_distance(&mut self, shader_clip_distance: bool) {
        self.shader_clip_distance = shader_clip_distance as VkBool32;
    }

    pub fn set_shader_cull_distance(&mut self, shader_cull_distance: bool) {
        self.shader_cull_distance = shader_cull_distance as VkBool32;
    }

    pub fn set_shader_float64(&mut self, shader_float64: bool) {
        self.shader_float64 = shader_float64 as VkBool32;
    }

    pub fn set_shader_int64(&mut self, shader_int64: bool) {
        self.shader_int64 = shader_int64 as VkBool32;
    }

    pub fn set_shader_int16(&mut self, shader_int16: bool) {
        self.shader_int16 = shader_int16 as VkBool32;
    }

    pub fn set_shader_resource_residency(&mut self, shader_resource_residency: bool) {
        self.shader_resource_residency = shader_resource_residency as VkBool32;
    }

    pub fn set_shader_resource_min_lod(&mut self, shader_resource_min_lod: bool) {
        self.shader_resource_min_lod = shader_resource_min_lod as VkBool32;
    }

    pub fn set_sparse_binding(&mut self, sparse_binding: bool) {
        self.sparse_binding = sparse_binding as VkBool32;
    }

    pub fn set_sparse_residency_buffer(&mut self, sparse_residency_buffer: bool) {
        self.sparse_residency_buffer = sparse_residency_buffer as VkBool32;
    }

    pub fn set_sparse_residency_image_2d(&mut self, sparse_residency_image_2d: bool) {
        self.sparse_residency_image_2d = sparse_residency_image_2d as VkBool32;
    }

    pub fn set_sparse_residency_image_3d(&mut self, sparse_residency_image_3d: bool) {
        self.sparse_residency_image_3d = sparse_residency_image_3d as VkBool32;
    }

    pub fn set_sparse_residency_2_samples(&mut self, sparse_residency_2_samples: bool) {
        self.sparse_residency_2_samples = sparse_residency_2_samples as VkBool32;
    }

    pub fn set_sparse_residency_4_samples(&mut self, sparse_residency_4_samples: bool) {
        self.sparse_residency_4_samples = sparse_residency_4_samples as VkBool32;
    }

    pub fn set_sparse_residency_8_samples(&mut self, sparse_residency_8_samples: bool) {
        self.sparse_residency_8_samples = sparse_residency_8_samples as VkBool32;
    }

    pub fn set_sparse_residency_16_samples(&mut self, sparse_residency_16_samples: bool) {
        self.sparse_residency_16_samples = sparse_residency_16_samples as VkBool32;
    }

    pub fn set_sparse_residency_aliased(&mut self, sparse_residency_aliased: bool) {
        self.sparse_residency_aliased = sparse_residency_aliased as VkBool32;
    }

    pub fn set_variable_multisample_rate(&mut self, variable_multisample_rate: bool) {
        self.variable_multisample_rate = variable_multisample_rate as VkBool32;
    }

    pub fn set_inherited_queries(&mut self, inherited_queries: bool) {
        self.inherited_queries = inherited_queries as VkBool32;
    }
}
