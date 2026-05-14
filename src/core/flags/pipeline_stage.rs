use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_3};

flags! {
    /// Bitmask of [`VkPipelineStageFlag`]
    ///
    /// # Description
    /// [`VkPipelineStageFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkPipelineStageFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkPipelineStageFlags;


    /// Bitmask specifying pipeline stages
    ///
    /// # Description
    /// These values all have the same meaning as the equivalently named values for
    /// [`VkPipelineStageFlags2`].
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkPipelineStageFlag {
        /// [`VkPipelineStageFlag::None`] specifies no stages of execution.
        ///
        /// Provided by [`VK_VERSION_1_3`]
        None = 0,

        /// [`VkPipelineStageFlag::TopOfPipe`] is equivalent to
        /// [`VkPipelineStageFlag::AllCommands`] with [`VkAccessFlags`] set to 0 when specified
        /// in the second synchronization scope, but specifies no stage of execution when specified
        /// in the first scope.
        TopOfPipe = 0x00000001,

        /// [`VkPipelineStageFlag::DrawIndirect`] specifies the stage of the pipeline where
        /// `VkDrawIndirect*`/`VkDispatchIndirect*`/`VkTraceRaysIndirect*` data structures are
        /// consumed. This stage also includes reading commands written by
        /// [`VkCmdExecuteGeneratedCommandsNv`]. This stage also includes reading commands written
        /// by [`VkCmdExecuteGeneratedCommandsExt`].
        DrawIndirect = 0x00000002,

        /// [`VkPipelineStageFlag::VertexInput`] specifies the stage of the pipeline where
        /// vertex and index buffers are consumed.
        VertexInput = 0x00000004,

        /// [`VkPipelineStageFlag::VertexShader`] specifies the vertex shader stage.
        VertexShader = 0x00000008,

        /// [`VkPipelineStageFlag::TessellationControlShader`] specifies the tessellation
        /// control shader stage.
        TessellationControlShader = 0x00000010,

        /// [`VkPipelineStageFlag::TessellationEvaluationShader`] specifies the tessellation
        /// evaluation shader stage.
        TessellationEvaluationShader = 0x00000020,

        /// [`VkPipelineStageFlag::GeometryShader`] specifies the geometry shader stage.
        GeometryShader = 0x00000040,

        /// [`VkPipelineStageFlag::FragmentShader`] specifies the fragment shader stage.
        FragmentShader = 0x00000080,

        /// [`VkPipelineStageFlag::EarlyFragmentTests`] specifies the stage of the pipeline
        /// where early fragment tests (depth and stencil tests before fragment shading) are
        /// performed. This stage also includes render pass load operations for framebuffer
        /// attachments with a depth/stencil format.
        EarlyFragmentTests = 0x00000100,

        /// [`VkPipelineStageFlag::LateFragmentTests`] specifies the stage of the pipeline where
        /// late fragment tests (depth and stencil tests after fragment shading) are performed.
        /// This stage also includes render pass store operations for framebuffer attachments with
        /// a depth/stencil format.
        LateFragmentTests = 0x00000200,

        /// [`VkPipelineStageFlag::ColorAttachmentOutput`] specifies the stage of the pipeline
        /// after blending where the final color values are output from the pipeline. This stage
        /// includes blending, logic operations, render pass load and store operations for color
        /// attachments, render pass multisample resolve operations, and [`VkCmdClearAttachments`].
        ColorAttachmentOutput = 0x00000400,

        /// [`VkPipelineStageFlag::ComputeShader`] specifies the execution of a compute shader.
        ComputeShader = 0x00000800,

        /// [`VkPipelineStageFlag::Transfer`] specifies the following commands:
        ///  - All copy commands, including [`VkCmdCopyQueryPoolResults`]
        ///  - [`VkCmdBlitImage2`] and [`VkCmdBlitImage`]
        ///  - [`VkCmdResolveImage2`] and [`VkCmdResolveImage`]
        ///  - All clear commands, with the exception of [`VkCmdClearAttachments`]
        Transfer = 0x00001000,

        /// [`VkPipelineStageFlag::BottomOfPipe`] is equivalent to
        /// [`VkPipelineStageFlag::AllCommands`] with [`VkAccessFlags`] set to 0 when specified
        /// in the first synchronization scope, but specifies no stage of execution when specified
        /// in the second scope.
        BottomOfPipe = 0x00002000,

        /// [`VkPipelineStageFlag::Host`] specifies a pseudo-stage indicating execution on the
        /// host of reads/writes of device memory. This stage is not invoked by any commands
        /// recorded in a command buffer.
        Host = 0x00004000,

        /// [`VkPipelineStageFlag::AllGraphics`] specifies the execution of all graphics
        /// pipeline stages, and is equivalent to the logical OR of:
        ///  - [`VkPipelineStageFlag::DrawIndirect`]
        ///  - [`VkPipelineStageFlag::TaskShaderExt`]
        ///  - [`VkPipelineStageFlag::MeshShaderExt`]
        ///  - [`VkPipelineStageFlag::VertexInput`]
        ///  - [`VkPipelineStageFlag::VertexShader`]
        ///  - [`VkPipelineStageFlag::TessellationControlShader`]
        ///  - [`VkPipelineStageFlag::TessellationEvaluationShader`]
        ///  - [`VkPipelineStageFlag::GeometryShader`]
        ///  - [`VkPipelineStageFlag::FragmentShader`]
        ///  - [`VkPipelineStageFlag::EarlyFragmentTests`]
        ///  - [`VkPipelineStageFlag::LateFragmentTests`]
        ///  - [`VkPipelineStageFlag::ColorAttachmentOutput`]
        ///  - [`VkPipelineStageFlag::ConditionalRenderingExt`]
        ///  - [`VkPipelineStageFlag::TransformFeedbackExt`]
        ///  - [`VkPipelineStageFlag::FragmentShadingRateAttachmentKhr`]
        ///  - [`VkPipelineStageFlag::FragmentDensityProcessExt`]
        AllGraphics = 0x00008000,

        /// [`VkPipelineStageFlag::AllCommands`] specifies all operations performed by all
        /// commands supported on the queue it is used with.
        AllCommands = 0x00010000,

        /// [`VkPipelineStageFlag::TransformFeedbackExt`] specifies the stage of the pipeline
        /// where vertex attribute output values are written to the transform feedback buffers.
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackExt = 0x01000000,

        /// [`VkPipelineStageFlag::ConditionalRenderingExt`] specifies the stage of the pipeline
        /// where the predicate of conditional rendering is consumed.
        ///
        /// Provided by [`ext_conditional_rendering`]
        ConditionalRenderingExt = 0x00040000,

        /// [`VkPipelineStageFlag::AccelerationStructureBuildKhr`] specifies the execution of
        /// [`VkCmdBuildAccelerationStructureNv`], [`VkCmdCopyAccelerationStructureNv`],
        /// [`VkCmdWriteAccelerationStructuresPropertiesNv`] ,
        /// [`VkCmdBuildAccelerationStructuresKhr`],
        /// [`VkCmdBuildAccelerationStructuresIndirectKhr`], [`VkCmdCopyAccelerationStructureKhr`],
        /// [`VkCmdCopyAccelerationStructureToMemoryKhr`],
        /// [`VkCmdCopyMemoryToAccelerationStructureKhr`], and
        /// [`VkCmdWriteAccelerationStructuresPropertiesKhr`].
        ///
        /// Provided by [`khr_acceleration_structure`]
        AccelerationStructureBuildKhr = 0x02000000,

        /// [`VkPipelineStageFlag::RayTracingShaderKhr`] specifies the execution of the ray
        /// tracing shader stages, via [`VkCmdTraceRaysNv`] , [`VkCmdTraceRaysKhr`], or
        /// [`VkCmdTraceRaysIndirectKhr`]
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RayTracingShaderKhr = 0x00200000,

        /// [`VkPipelineStageFlag::FragmentDensityProcessExt`] specifies the stage of the
        /// pipeline where the fragment density map is read to generate the fragment areas.
        ///
        /// Provided by [`ext_fragment_density_map`]
        FragmentDensityProcessExt = 0x00400000,

        /// [`VkPipelineStageFlag::FragmentShadingRateAttachmentKhr`] specifies the stage of the
        /// pipeline where the fragment shading rate attachment or shading rate image is read to
        /// determine the fragment shading rate for portions of a rasterized primitive.
        ///
        /// Provided by [`khr_fragment_shading_rate`]
        FragmentShadingRateAttachmentKhr = 0x00800000,

        /// [`VkPipelineStageFlag::TaskShaderExt`] specifies the task shader stage.
        ///
        /// Provided by [`ext_mesh_shader`]
        TaskShaderExt = 0x00080000,

        /// [`VkPipelineStageFlag::MeshShaderExt`] specifies the mesh shader stage.
        ///
        /// Provided by [`ext_mesh_shader`]
        MeshShaderExt = 0x00100000,

        /// [`VkPipelineStageFlag::CommandPreprocessExt`] specifies the stage of the pipeline
        /// where device-side preprocessing for generated commands via
        /// [`VkCmdPreprocessGeneratedCommandsExt`] is handled.
        ///
        /// Provided by [`ext_device_generated_commands`]
        CommandPreprocessExt = 0x00020000,
    }
}
