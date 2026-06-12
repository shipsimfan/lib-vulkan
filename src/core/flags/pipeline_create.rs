use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_3, VK_VERSION_1_4, VkPipeline,
    VkResult,
};
#[allow(unused_imports)]
use std::ptr::null;

flags! {
    /// Bitmask of [`VkPipelineCreateFlag`]
    ///
    /// # Description
    /// [`VkPipelineCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkPipelineCreateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkPipelineCreateFlags;


    /// Bitmask controlling how a pipeline is created
    ///
    /// # Description
    /// It is valid to set both [`VkPipelineCreateFlag::AllowDerivatives`] and
    /// [`VkPipelineCreateFlag::Derivative`]. This allows a pipeline to be both a parent and
    /// possibly a child in a pipeline hierarchy.
    ///
    /// When an implementation is looking up a pipeline in a pipeline cache, if that pipeline is
    /// being created using linked libraries, implementations should always return an equivalent
    /// pipeline created with [`VkPipelineCreateFlag::LinkTimeOptimizationExt`] if available,
    /// whether or not that bit was specified.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkPipelineCreateFlag {
        /// [`VkPipelineCreateFlag::DisableOptimization`] specifies that the created pipeline will
        /// not be optimized. Using this flag may reduce the time taken to create the pipeline.
        DisableOptimization = 0x00000001,

        /// [`VkPipelineCreateFlag::AllowDerivatives`] specifies that the pipeline to be created is
        /// allowed to be the parent of a pipeline that will be created in a subsequent pipeline
        /// creation call.
        AllowDerivatives = 0x00000002,

        /// [`VkPipelineCreateFlag::Derivative`] specifies that the pipeline to be created will be
        /// a child of a previously created parent pipeline.
        Derivative = 0x00000004,

        /// [`VkPipelineCreateFlag::ViewIndexFromDeviceIndex`] specifies that any shader input
        /// variables decorated as `ViewIndex` will be assigned values as if they were decorated as
        /// `DeviceIndex`.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        ViewIndexFromDeviceIndex = 0x00000008,

        /// [`VkPipelineCreateFlag::DispatchBase`] specifies that a compute pipeline can be used
        /// with [`VkCmdDispatchBase`] with a non-zero base workgroup.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        DispatchBase = 0x00000010,

        /// [`VkPipelineCreateFlag::FailOnPipelineCompileRequired`] specifies that pipeline
        /// creation will fail if a compile is required for creation of a valid [`VkPipeline`]
        /// object; [`VkResult::VkPipelineCompileRequired`] will be returned by pipeline creation,
        /// and the [`VkPipeline`] will be [`VK_NULL_HANDLE`].
        ///
        /// Provided by [`VK_VERSION_1_3`]
        FailOnPipelineCompileRequired = 0x00000100,

        /// When creating multiple pipelines, [`VkPipelineCreateFlag::EarlyReturnOnFailure`]
        /// specifies that control will be returned to the application if any individual pipeline
        /// returns a result which is not [`VkResult::VkSuccess`] rather than continuing to create
        /// additional pipelines.
        ///
        /// Provided by [`VK_VERSION_1_3`]
        EarlyReturnOnFailure = 0x00000200,

        /// [`VkPipelineCreateFlag::NoProtectedAccess`] specifies that the pipeline must not be
        /// bound to a protected command buffer.
        ///
        /// Provided by [`VK_VERSION_1_4`]
        NoProtectedAccess = 0x08000000,

        /// [`VkPipelineCreateFlag::ProtectedAccessOnly`] specifies that the pipeline must not be
        /// bound to an unprotected command buffer.
        ///
        /// Provided by [`VK_VERSION_1_4`]
        ProtectedAccessOnly = 0x40000000,

        /// [`VkPipelineCreateFlag::RayTracingNoNullAnyHitShadersKhr`] specifies that an any-hit
        /// shader will always be present when an any-hit shader would be executed. A [`null`]
        /// any-hit shader is an any-hit shader which is effectively [`VK_SHADER_UNUSED_KHR`], such
        /// as from a shader group consisting entirely of zeros.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RayTracingNoNullAnyHitShadersKhr = 0x00004000,

        /// [`VkPipelineCreateFlag::RayTracingNoNullClosestHitShadersKhr`] specifies that a closest
        /// hit shader will always be present when a closest hit shader would be executed. A
        /// [`null`] closest hit shader is a closest hit shader which is effectively
        /// [`VK_SHADER_UNUSED_KHR`], such as from a shader group consisting entirely of zeros.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RayTracingNoNullClosestHitShadersKhr = 0x00008000,

        /// [`VkPipelineCreateFlag::RayTracingNoNullMissShadersKhr`] specifies that a miss shader
        /// will always be present when a miss shader would be executed. A [`null`] miss shader is
        /// a miss shader which is effectively [`VK_SHADER_UNUSED_KHR`], such as from a shader
        /// group consisting entirely of zeros.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RayTracingNoNullMissShadersKhr = 0x00010000,

        /// [`VkPipelineCreateFlag::RayTracingNoNullIntersectionShadersKhr`] specifies that an
        /// intersection shader will always be present when an intersection shader would be
        /// executed. A [`null`] intersection shader is an intersection shader which is effectively
        /// [`VK_SHADER_UNUSED_KHR`], such as from a shader group consisting entirely of zeros.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RayTracingNoNullIntersectionShadersKhr = 0x00020000,

        /// [`VkPipelineCreateFlag::RayTracingSkipTrianglesKhr`] specifies that sphere, LSS and
        /// triangle primitives will be skipped during traversal using pipeline trace ray
        /// instructions.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RayTracingSkipTrianglesKhr = 0x00001000,

        /// [`VkPipelineCreateFlag::RayTracingSkipAabbsKhr`] specifies that AABB primitives will be
        /// skipped during traversal using pipeline trace ray instructions.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RayTracingSkipAabbsKhr = 0x00002000,

        /// [`VkPipelineCreateFlag::RayTracingShaderGroupHandleCaptureReplayKhr`] specifies that
        /// the shader group handles can be saved and reused on a subsequent run (e.g. for trace
        /// capture and replay).
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RayTracingShaderGroupHandleCaptureReplayKhr = 0x00080000,

        /// [`VkPipelineCreateFlag::DeferCompileNv`] specifies that a pipeline is created with all
        /// shaders in the deferred state. Before using the pipeline the application must call
        /// [`VkCompileDeferredNv`] exactly once on each shader in the pipeline before using the
        /// pipeline.
        ///
        /// Provided by [`nv_ray_tracing`]
        DeferCompileNv = 0x00000020,

        /// [`VkPipelineCreateFlag::RenderingFragmentDensityMapAttachmentExt`] specifies that the
        /// pipeline will be used with a fragment density map attachment and dynamic rendering.
        ///
        /// Provided by [`ext_fragment_density_map`] with [`VK_VERSION_1_3`] or [`khr_dynamic_rendering`]
        RenderingFragmentDensityMapAttachmentExt = 0x00400000,

        /// [`VkPipelineCreateFlag::RenderingFragmentShadingRateAttachmentKhr`] specifies that the
        /// pipeline will be used with a fragment shading rate attachment and dynamic rendering.
        ///
        /// Provided by [`khr_fragment_shading_rate`] with [`VK_VERSION_1_3`] or [`khr_dynamic_rendering`]
        RenderingFragmentShadingRateAttachmentKhr = 0x00200000,

        /// [`VkPipelineCreateFlag::CaptureStatisticsKhr`] specifies that the shader compiler
        /// should capture statistics for the pipeline executables produced by the compile process
        /// which can later be retrieved by calling [`VkGetPipelineExecutableStatisticsKhr`].
        /// Enabling this flag must not affect the final compiled pipeline but may disable pipeline
        /// caching or otherwise affect pipeline creation time.
        ///
        /// Provided by [`khr_pipeline_executable_properties`]
        CaptureStatisticsKhr = 0x00000040,

        /// [`VkPipelineCreateFlag::CaptureInternalRepresentationsKhr`] specifies that the shader
        /// compiler should capture the internal representations of pipeline executables produced
        /// by the compile process which can later be retrieved by calling
        /// [`VkGetPipelineExecutableInternalRepresentationsKhr`]. Enabling this flag must not
        /// affect the final compiled pipeline but may disable pipeline caching or otherwise affect
        /// pipeline creation time. When capturing IR from pipelines created with pipeline
        /// libraries, there is no guarantee that IR from libraries can be retrieved from the
        /// linked pipeline. Applications should retrieve IR from each library, and any linked
        /// pipelines, separately.
        ///
        /// Provided by [`khr_pipeline_executable_properties`]
        CaptureInternalRepresentationsKhr = 0x00000080,

        /// [`VkPipelineCreateFlag::IndirectBindableNv`] specifies that the pipeline can be used in
        /// combination with Device-Generated Commands.
        ///
        /// Provided by [`nv_device_generated_commands`]
        IndirectBindableNv = 0x00040000,

        /// [`VkPipelineCreateFlag::LibraryKhr`] specifies that the pipeline cannot be used
        /// directly, and instead defines a pipeline library that can be combined with other
        /// pipelines using the [`VkPipelineLibraryCreateInfoKhr`] structure. This is available in
        /// ray tracing and graphics pipelines.
        ///
        /// Provided by [`khr_pipeline_library`]
        LibraryKhr = 0x00000800,

        /// [`VkPipelineCreateFlag::DescriptorBufferExt`] specifies that a pipeline will be used
        /// with descriptor buffers, rather than descriptor sets.
        ///
        /// Provided by [`ext_descriptor_buffer`]
        DescriptorBufferExt = 0x20000000,

        /// [`VkPipelineCreateFlag::RetainLinkTimeOptimizationInfoExt`] specifies that pipeline
        /// libraries should retain any information necessary to later perform an optimal link with
        /// [`VkPipelineCreateFlag::LinkTimeOptimizationExt`].
        ///
        /// Provided by [`ext_graphics_pipeline_library`]
        RetainLinkTimeOptimizationInfoExt = 0x00800000,

        /// [`VkPipelineCreateFlag::LinkTimeOptimizationExt`] specifies that pipeline libraries
        /// being linked into this library should have link time optimizations applied. If this bit
        /// is omitted, implementations should instead perform linking as rapidly as possible.
        ///
        /// Provided by [`ext_graphics_pipeline_library`]
        LinkTimeOptimizationExt = 0x00000400,

        /// [`VkPipelineCreateFlag::RayTracingAllowMotionNv`] specifies that the pipeline is
        /// allowed to use [`OpTraceRayMotionNv`].
        ///
        /// Provided by [`nv_ray_tracing_motion_blur`]
        RayTracingAllowMotionNv = 0x00100000,

        /// [`VkPipelineCreateFlag::ColorAttachmentFeedbackLoopExt`] specifies that the pipeline
        /// may be used with an attachment feedback loop including color attachments. It is ignored
        /// if [`VkDynamicState::AttachmentFeedbackLoopEnableExt`] is set in `dynamic_states`.
        ///
        /// Provided by [`ext_attachment_feedback_loop_layout`]
        ColorAttachmentFeedbackLoopExt = 0x02000000,

        /// [`VkPipelineCreateFlag::DepthStencilAttachmentFeedbackLoopExt`] specifies that the
        /// pipeline may be used with an attachment feedback loop including depth-stencil
        /// attachments. It is ignored if [`VkDynamicState::AttachmentFeedbackLoopEnableExt`] is
        /// set in `dynamic_states`.
        ///
        /// Provided by [`ext_attachment_feedback_loop_layout`]
        DepthStencilAttachmentFeedbackLoopExt = 0x04000000,

        /// [`VkPipelineCreateFlag::RayTracingDisplacementMicromapNv`] specifies that the ray
        /// tracing pipeline can be used with acceleration structures which reference a
        /// displacement micromap array.
        ///
        /// Provided by [`nv_displacement_micromap`]
        RayTracingDisplacementMicromapNv = 0x10000000,

        /// [`VkPipelineCreateFlag::RayTracingOpacityMicromapKhr`] specifies that the ray tracing
        /// pipeline can be used with acceleration structures which reference an opacity micromap
        /// array. This flag has no effect on using opacity micromaps with ray queries.
        ///
        /// Provided by [`khr_opacity_micromap`]
        RayTracingOpacityMicromapKhr = 0x01000000,
    }
}
