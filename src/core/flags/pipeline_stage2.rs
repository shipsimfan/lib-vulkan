use crate::flags64;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkAccessFlag, VkAccessFlags2};

flags64! {
    /// 64-bit mask of pipeline stage flags
    ///
    /// # Description
    /// [`VkPipelineStageFlags2`] is a bitmask type for setting a mask of zero or more
    /// [`VkPipelineStageFlag2`].
    ///
    /// Provided by [`VK_VERSION_1_3`]
    pub struct VkPipelineStageFlags2;

    /// Pipeline stage flags for [`VkPipelineStageFlags2`]
    ///
    /// Provided by [`VK_VERSION_1_3`]
    pub enum VkPipelineStageFlag2 {
        /// [`VkPipelineStage2::None`] specifies no stages of execution.
        None = 0x0,

        /// [`VkPipelineStage2::TopOfPipe`] is equivalent to [`VkPipelineStage2::AllCommands`] with
        /// [`VkAccessFlags2`] set to 0 when specified in the second synchronization scope, but
        /// equivalent to [`VkPipelineStage2::None`] in the first scope.
        TopOfPipe = 0x00000001,

        /// [`VkPipelineStage2::DrawIndirect`] specifies the stage of the pipeline where indirect
        /// command parameters are consumed. This stage also includes reading commands written by
        /// [`VkCmdPreprocessGeneratedCommandsNv`]. This stage also includes reading commands
        /// written by [`VkCmdPreprocessGeneratedCommandsExt`].
        DrawIndirect = 0x00000002,

        /// [`VkPipelineStage2::VertexInput`] is equivalent to the logical OR of:
        ///  - [`VkPipelineStage2::IndexInput`]
        ///  - [`VkPipelineStage2::VertexAttributeInput`]
        VertexInput = 0x00000004,

        /// [`VkPipelineStage2::VertexShader`] specifies the vertex shader stage.
        VertexShader = 0x00000008,

        /// [`VkPipelineStage2::TessellationControlShader`] specifies the tessellation control
        /// shader stage.
        TessellationControlShader = 0x00000010,

        /// [`VkPipelineStage2::TessellationEvaluationShader`] specifies the tessellation
        /// evaluation shader stage.
        TessellationEvaluationShader = 0x00000020,

        /// [`VkPipelineStage2::GeometryShader`] specifies the geometry shader stage.
        GeometryShader = 0x00000040,

        /// [`VkPipelineStage2::FragmentShader`] specifies the fragment shader stage.
        FragmentShader = 0x00000080,

        /// [`VkPipelineStage2::EarlyFragmentTests`] specifies the stage of the pipeline where
        /// early fragment tests (depth and stencil tests before fragment shading) are performed.
        /// This stage also includes render pass load operations for framebuffer attachments with a
        /// depth/stencil format.
        EarlyFragmentTests = 0x00000100,

        /// [`VkPipelineStage2::LateFragmentTests`] specifies the stage of the pipeline where late
        /// fragment tests (depth and stencil tests after fragment shading) are performed. This
        /// stage also includes render pass store operations for framebuffer attachments with a
        /// depth/stencil format.
        LateFragmentTests = 0x00000200,

        /// [`VkPipelineStage2::ColorAttachmentOutput`] specifies the stage of the pipeline where
        /// final color values are output from the pipeline. This stage includes blending, logic
        /// operations, render pass load and store operations for color attachments, render pass
        /// multisample resolve operations, and [`VkCmdClearAttachments`].
        ColorAttachmentOutput = 0x00000400,

        /// [`VkPipelineStage2::ComputeShader`] specifies the compute shader stage.
        ComputeShader = 0x00000800,

        /// [`VkPipelineStage2::AllTransfer`] is equivalent to specifying all of:
        ///  - [`VkPipelineStage2::Copy`]
        ///  - [`VkPipelineStage2::Blit`]
        ///  - [`VkPipelineStage2::Resolve`]
        ///  - [`VkPipelineStage2::Clear`]
        ///  - [`VkPipelineStage2::AccelerationStructureCopyKhr`]
        AllTransfer = 0x00001000,

        /// [`VkPipelineStage2::BottomOfPipe`] is equivalent to [`VkPipelineStage2::AllCommands`]
        /// with [`VkAccessFlags2`] set to 0 when specified in the first synchronization scope, but
        /// equivalent to [`VkPipelineStage2::None`] in the second scope.
        BottomOfPipe = 0x00002000,

        /// [`VkPipelineStage2::Host`] specifies a pseudo-stage indicating execution on the host of
        /// reads/writes of device memory. This stage is not invoked by any commands recorded in a
        /// command buffer.
        Host = 0x00004000,

        /// [`VkPipelineStage2::AllGraphics`] specifies the execution of all graphics pipeline
        /// stages, and is equivalent to the logical OR of:
        ///  - [`VkPipelineStage2::DrawIndirect`]
        ///  - [`VkPipelineStage2::CopyIndirect_Khr`]
        ///  - [`VkPipelineStage2::TaskShaderExt`]
        ///  - [`VkPipelineStage2::MeshShaderExt`]
        ///  - [`VkPipelineStage2::VertexInput`]
        ///  - [`VkPipelineStage2::VertexShader`]
        ///  - [`VkPipelineStage2::TessellationControlShader`]
        ///  - [`VkPipelineStage2::TessellationEvaluationShader`]
        ///  - [`VkPipelineStage2::GeometryShader`]
        ///  - [`VkPipelineStage2::FragmentShader`]
        ///  - [`VkPipelineStage2::EarlyFragmentTests`]
        ///  - [`VkPipelineStage2::LateFragmentTests`]
        ///  - [`VkPipelineStage2::ColorAttachmentOutput`]
        ///  - [`VkPipelineStage2::ConditionalRenderingExt`]
        ///  - [`VkPipelineStage2::TransformFeedbackExt`]
        ///  - [`VkPipelineStage2::FragmentShadingRateAttachmentKhr`]
        ///  - [`VkPipelineStage2::FragmentDensityProcessExt`]
        ///  - [`VkPipelineStage2::SubpassShaderHuawei`]
        ///  - [`VkPipelineStage2::InvocationMaskHuawei`]
        ///  - [`VkPipelineStage2::ClusterCullingShaderHuawei`]
        AllGraphics = 0x00008000,

        /// [`VkPipelineStage2::AllCommands`] specifies all operations performed by all commands
        /// supported on the queue it is used with.
        AllCommands = 0x00010000,

        /// [`VkPipelineStage2::Copy`] specifies the execution of all copy commands, including
        /// [`VkCmdCopyQueryPoolResults`].
        Copy = 0x100000000,

        /// [`VkPipelineStage2::Resolve`] specifies the execution of [`VkCmdResolveImage`].
        Resolve = 0x200000000,

        /// [`VkPipelineStage2::Blit`] specifies the execution of [`VkCmdBlitImage`].
        Blit = 0x400000000,

        /// [`VkPipelineStage2::Clear`] specifies the execution of clear commands, with the
        /// exception of [`VkCmdClearAttachments`].
        Clear = 0x800000000,

        /// [`VkPipelineStage2::IndexInput`] specifies the stage of the pipeline where index
        /// buffers are consumed.
        IndexInput = 0x1000000000,

        /// [`VkPipelineStage2::VertexAttributeInput`] specifies the stage of the pipeline where
        /// vertex buffers are consumed.
        VertexAttributeInput = 0x2000000000,

        /// [`VkPipelineStage2::PreRasterizationShaders`] is equivalent to specifying all supported
        /// pre-rasterization shader stages:
        ///  - [`VkPipelineStage2::VertexShader`]
        ///  - [`VkPipelineStage2::TessellationControlShader`]
        ///  - [`VkPipelineStage2::TessellationEvaluationShader`]
        ///  - [`VkPipelineStage2::GeometryShader`]
        ///  - [`VkPipelineStage2::TaskShaderExt`]
        ///  - [`VkPipelineStage2::MeshShaderExt`]
        ///  - [`VkPipelineStage2::ClusterCullingShaderHuawei`]
        PreRasterizationShaders = 0x4000000000,

        /// [`VkPipelineStage2::VideoDecodeKhr`] specifies the execution of video decode
        /// operations.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeKhr = 0x04000000,

        /// [`VkPipelineStage2::VideoEncodeKhr`] specifies the execution of video encode
        /// operations.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeKhr = 0x08000000,

        /// [`VkPipelineStage2::TransformFeedbackExt`] specifies the stage of the pipeline where
        /// vertex attribute output values are written to the transform feedback buffers.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_transform_feedback`]
        TransformFeedbackExt = 0x01000000,

        /// [`VkPipelineStage2::ConditionalRenderingExt`] specifies the stage of the pipeline where
        /// the predicate of conditional rendering is consumed.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_conditional_rendering`]
        ConditionalRenderingExt = 0x00040000,

        /// [`VkPipelineStage2::CommandPreprocessExt`] specifies the stage of the pipeline where
        /// device-side generation of commands via [`VkCmdPreprocessGeneratedCommandsExt`] is
        /// handled.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_device_generated_commands`]
        CommandPreprocessExt = 0x00020000,

        /// [`VkPipelineStage2::FragmentShadingRateAttachmentKhr`] specifies the stage of the
        /// pipeline where the fragment shading rate attachment or shading rate image is read to
        /// determine the fragment shading rate for portions of a rasterized primitive.
        ///
        /// Provided by [`khr_fragment_shading_rate`] with [`khr_synchronization2`]
        FragmentShadingRateAttachmentKhr = 0x00400000,

        /// [`VkPipelineStage2::AccelerationStructureBuildKhr`] specifies the execution of
        /// acceleration structure commands or acceleration structure copy commands.
        ///
        /// Provided by [`khr_acceleration_structure`] with [`khr_synchronization2`]
        AccelerationStructureBuildKhr = 0x02000000,

        /// [`VkPipelineStage2::RayTracingShaderKhr`] specifies the execution of the ray tracing
        /// shader stages.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`] with [`khr_synchronization2`]
        RayTracingShaderKhr = 0x00200000,

        /// [`VkPipelineStage2::FragmentDensityProcessExt`] specifies the stage of the pipeline
        /// where the fragment density map is read to generate the fragment areas.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_fragment_density_map`]
        FragmentDensityProcessExt = 0x00800000,

        /// [`VkPipelineStage2::TaskShaderExt`] specifies the task shader stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_mesh_shader`]
        TaskShaderExt = 0x00080000,

        /// [`VkPipelineStage2::MeshShaderExt`] specifies the mesh shader stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_mesh_shader`]
        MeshShaderExt = 0x00100000,

        /// [`VkPipelineStage2::SubpassShaderHuawei`] specifies the subpass shading shader stage.
        ///
        /// Provided by [`huawei_subpass_shading`]
        SubpassShaderHuawei = 0x8000000000,

        /// [`VkPipelineStage2::InvocationMaskHuawei`] specifies the stage of the pipeline where
        /// the invocation mask image is read by the implementation to optimize the ray dispatch.
        ///
        /// Provided by [`huawei_invocation_mask`]
        InvocationMaskHuawei = 0x10000000000,

        /// [`VkPipelineStage2::AccelerationStructureCopyKhr`] specifies the execution of
        /// acceleration structure copy commands.
        ///
        /// Provided by [`khr_ray_tracing_maintenance1`] with [`khr_synchronization2`] or
        /// [`VK_VERSION_1_3`]
        AccelerationStructureCopyKhr = 0x10000000,

        /// [`VkPipelineStage2::MicromapBuildExt`] specifies the execution of micromap commands for
        /// [`VkMicromapExt`] objects.
        ///
        /// Provided by [`ext_opacity_micromap`]
        MicromapBuildExt = 0x40000000,

        /// [`VkPipelineStage2::ClusterCullingShaderHuawei`] specifies the cluster culling shader
        /// stage.
        ///
        /// Provided by [`huawei_cluster_culling_shader`]
        ClusterCullingShaderHuawei = 0x20000000000,

        /// [`VkPipelineStage2::OpticalFlowNv`] specifies the stage of the pipeline where optical
        /// flow operation are performed.
        ///
        /// Provided by [`nv_optical_flow`]
        OpticalFlowNv = 0x20000000,

        /// [`VkPipelineStage2::ConvertCooperativeVectorMatrixNv`] specifies the execution of
        /// [`VkCmdConvertCooperativeVectorMatrixNv`].
        ///
        /// Provided by [`nv_cooperative_vector`]
        ConvertCooperativeVectorMatrixNv = 0x100000000000,

        /// Provided by [`arm_data_graph`]
        DataGraphArm = 0x40000000000,

        /// [`VkPipelineStage2::CopyIndirectKhr`] specifies the stage of the pipeline where
        /// indirect copy commands (`VkCmdCopyMemoryIndirect*` and
        /// `VkCmdCopyMemoryToImageIndirect*`) parameters are consumed.
        ///
        /// Provided by [`khr_copy_memory_indirect`]
        CopyIndirectKhr = 0x400000000000,

        /// Provided by [`ext_memory_decompression`]
        MemoryDecompressionExt = 0x200000000000,
    }
}
