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

        /// [`VkPipelineStageFlag::TopOfPipeBit`] is equivalent to
        /// [`VkPipelineStageFlag::AllCommandsBit`] with [`VkAccessFlags`] set to 0 when specified
        /// in the second synchronization scope, but specifies no stage of execution when specified
        /// in the first scope.
        TopOfPipeBit = 0x00000001,

        /// [`VkPipelineStageFlag::DrawIndirectBit`] specifies the stage of the pipeline where
        /// `VkDrawIndirect*`/`VkDispatchIndirect*`/`VkTraceRaysIndirect*` data structures are
        /// consumed. This stage also includes reading commands written by
        /// [`VkCmdExecuteGeneratedCommandsNv`]. This stage also includes reading commands written
        /// by [`VkCmdExecuteGeneratedCommandsExt`].
        DrawIndirectBit = 0x00000002,

        /// [`VkPipelineStageFlag::VertexInputBit`] specifies the stage of the pipeline where
        /// vertex and index buffers are consumed.
        VertexInputBit = 0x00000004,

        /// [`VkPipelineStageFlag::VertexShaderBit`] specifies the vertex shader stage.
        VertexShaderBit = 0x00000008,

        /// [`VkPipelineStageFlag::TessellationControlShaderBit`] specifies the tessellation
        /// control shader stage.
        TessellationControlShaderBit = 0x00000010,

        /// [`VkPipelineStageFlag::TessellationEvaluationShaderBit`] specifies the tessellation
        /// evaluation shader stage.
        TessellationEvaluationShaderBit = 0x00000020,

        /// [`VkPipelineStageFlag::GeometryShaderBit`] specifies the geometry shader stage.
        GeometryShaderBit = 0x00000040,

        /// [`VkPipelineStageFlag::FragmentShaderBit`] specifies the fragment shader stage.
        FragmentShaderBit = 0x00000080,

        /// [`VkPipelineStageFlag::EarlyFragmentTestsBit`] specifies the stage of the pipeline
        /// where early fragment tests (depth and stencil tests before fragment shading) are
        /// performed. This stage also includes render pass load operations for framebuffer
        /// attachments with a depth/stencil format.
        EarlyFragmentTestsBit = 0x00000100,

        /// [`VkPipelineStageFlag::LateFragmentTestsBit`] specifies the stage of the pipeline where
        /// late fragment tests (depth and stencil tests after fragment shading) are performed.
        /// This stage also includes render pass store operations for framebuffer attachments with
        /// a depth/stencil format.
        LateFragmentTestsBit = 0x00000200,

        /// [`VkPipelineStageFlag::ColorAttachmentOutputBit`] specifies the stage of the pipeline
        /// after blending where the final color values are output from the pipeline. This stage
        /// includes blending, logic operations, render pass load and store operations for color
        /// attachments, render pass multisample resolve operations, and [`VkCmdClearAttachments`].
        ColorAttachmentOutputBit = 0x00000400,

        /// [`VkPipelineStageFlag::ComputeShaderBit`] specifies the execution of a compute shader.
        ComputeShaderBit = 0x00000800,

        /// [`VkPipelineStageFlag::TransferBit`] specifies the following commands:
        ///  - All copy commands, including [`VkCmdCopyQueryPoolResults`]
        ///  - [`VkCmdBlitImage2`] and [`VkCmdBlitImage`]
        ///  - [`VkCmdResolveImage2`] and [`VkCmdResolveImage`]
        ///  - All clear commands, with the exception of [`VkCmdClearAttachments`]
        TransferBit = 0x00001000,

        /// [`VkPipelineStageFlag::BottomOfPipeBit`] is equivalent to
        /// [`VkPipelineStageFlag::AllCommandsBit`] with [`VkAccessFlags`] set to 0 when specified
        /// in the first synchronization scope, but specifies no stage of execution when specified
        /// in the second scope.
        BottomOfPipeBit = 0x00002000,

        /// [`VkPipelineStageFlag::HostBit`] specifies a pseudo-stage indicating execution on the
        /// host of reads/writes of device memory. This stage is not invoked by any commands
        /// recorded in a command buffer.
        HostBit = 0x00004000,

        /// [`VkPipelineStageFlag::AllGraphicsBit`] specifies the execution of all graphics
        /// pipeline stages, and is equivalent to the logical OR of:
        ///  - [`VkPipelineStageFlag::DrawIndirectBit`]
        ///  - [`VkPipelineStageFlag::TaskShaderBitExt`]
        ///  - [`VkPipelineStageFlag::MeshShaderBitExt`]
        ///  - [`VkPipelineStageFlag::VertexInputBit`]
        ///  - [`VkPipelineStageFlag::VertexShaderBit`]
        ///  - [`VkPipelineStageFlag::TessellationControlShaderBit`]
        ///  - [`VkPipelineStageFlag::TessellationEvaluationShaderBit`]
        ///  - [`VkPipelineStageFlag::GeometryShaderBit`]
        ///  - [`VkPipelineStageFlag::FragmentShaderBit`]
        ///  - [`VkPipelineStageFlag::EarlyFragmentTestsBit`]
        ///  - [`VkPipelineStageFlag::LateFragmentTestsBit`]
        ///  - [`VkPipelineStageFlag::ColorAttachmentOutputBit`]
        ///  - [`VkPipelineStageFlag::ConditionalRenderingBitExt`]
        ///  - [`VkPipelineStageFlag::TransformFeedbackBitExt`]
        ///  - [`VkPipelineStageFlag::FragmentShadingRateAttachmentBitKhr`]
        ///  - [`VkPipelineStageFlag::FragmentDensityProcessBitExt`]
        AllGraphicsBit = 0x00008000,

        /// [`VkPipelineStageFlag::AllCommandsBit`] specifies all operations performed by all
        /// commands supported on the queue it is used with.
        AllCommandsBit = 0x00010000,

        /// [`VkPipelineStageFlag::TransformFeedbackBitExt`] specifies the stage of the pipeline
        /// where vertex attribute output values are written to the transform feedback buffers.
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackBitExt = 0x01000000,

        /// [`VkPipelineStageFlag::ConditionalRenderingBitExt`] specifies the stage of the pipeline
        /// where the predicate of conditional rendering is consumed.
        ///
        /// Provided by [`ext_conditional_rendering`]
        ConditionalRenderingBitExt = 0x00040000,

        /// [`VkPipelineStageFlag::AccelerationStructureBuildBitKhr`] specifies the execution of
        /// [`VkCmdBuildAccelerationStructureNv`], [`VkCmdCopyAccelerationStructureNv`],
        /// [`VkCmdWriteAccelerationStructuresPropertiesNv`] ,
        /// [`VkCmdBuildAccelerationStructuresKhr`],
        /// [`VkCmdBuildAccelerationStructuresIndirectKhr`], [`VkCmdCopyAccelerationStructureKhr`],
        /// [`VkCmdCopyAccelerationStructureToMemoryKhr`],
        /// [`VkCmdCopyMemoryToAccelerationStructureKhr`], and
        /// [`VkCmdWriteAccelerationStructuresPropertiesKhr`].
        ///
        /// Provided by [`khr_acceleration_structure`]
        AccelerationStructureBuildBitKhr = 0x02000000,

        /// [`VkPipelineStageFlag::RayTracingShaderBitKhr`] specifies the execution of the ray
        /// tracing shader stages, via [`VkCmdTraceRaysNv`] , [`VkCmdTraceRaysKhr`], or
        /// [`VkCmdTraceRaysIndirectKhr`]
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        RayTracingShaderBitKhr = 0x00200000,

        /// [`VkPipelineStageFlag::FragmentDensityProcessBitExt`] specifies the stage of the
        /// pipeline where the fragment density map is read to generate the fragment areas.
        ///
        /// Provided by [`ext_fragment_density_map`]
        FragmentDensityProcessBitExt = 0x00400000,

        /// [`VkPipelineStageFlag::FragmentShadingRateAttachmentBitKhr`] specifies the stage of the
        /// pipeline where the fragment shading rate attachment or shading rate image is read to
        /// determine the fragment shading rate for portions of a rasterized primitive.
        ///
        /// Provided by [`khr_fragment_shading_rate`]
        FragmentShadingRateAttachmentBitKhr = 0x00800000,

        /// [`VkPipelineStageFlag::TaskShaderBitExt`] specifies the task shader stage.
        ///
        /// Provided by [`ext_mesh_shader`]
        TaskShaderBitExt = 0x00080000,

        /// [`VkPipelineStageFlag::MeshShaderBitExt`] specifies the mesh shader stage.
        ///
        /// Provided by [`ext_mesh_shader`]
        MeshShaderBitExt = 0x00100000,

        /// [`VkPipelineStageFlag::CommandPreprocessBitExt`] specifies the stage of the pipeline
        /// where device-side preprocessing for generated commands via
        /// [`VkCmdPreprocessGeneratedCommandsExt`] is handled.
        ///
        /// Provided by [`ext_device_generated_commands`]
        CommandPreprocessBitExt = 0x00020000,
    }
}
