use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_3};

flags! {
    /// Bitmask of [`VkAccessFlags`]
    ///
    /// # Description
    /// [`VkAccessFlags`] is a bitmask type for setting a mask of zero or more [`VkAccessFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkAccessFlags;


    /// Bitmask specifying memory access types that will participate in a memory dependency
    ///
    /// # Description
    /// These values all have the same meaning as the equivalently named values for
    /// [`VkAccessFlags2`].
    ///
    /// Certain access types are only performed by a subset of pipeline stages. Any synchronization
    /// command that takes both stage masks and access masks uses both to define the access scopes
    /// - only the specified access types performed by the specified stages are included in the
    /// access scope. An application must not specify an access flag in a synchronization command
    /// if it does not include a pipeline stage in the corresponding stage mask that is able to
    /// perform accesses of that type.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkAccessFlag {
        /// [`VkAccessFlag::None`] specifies no accesses.
        ///
        /// Provided by [`VK_VERSION_1_3`]
        None = 0,

        /// [`VkAccessFlag::IndirectCommandRead`] specifies read access to indirect command data
        /// read as part of an indirect build, trace, drawing or dispatching command. Such access
        /// occurs in the [`VkPipelineStageFlag::DrawIndirect`] pipeline stage.
        IndirectCommandRead = 0x00000001,

        /// [`VkAccessFlag::IndexRead`] specifies read access to an index buffer as part of an
        /// indexed drawing command, bound by [`VkCmdBindIndexBuffer2`] and
        /// [`VkCmdBindIndexBuffer`]. Such access occurs in the
        /// [`VkPipelineStageFlag::VertexInput`] pipeline stage.
        IndexRead = 0x00000002,

        /// [`VkAccessFlag::VertexAttributeRead`] specifies read access to a vertex buffer as
        /// part of a drawing command, bound by [`VkCmdBindVertexBuffers`]. Such access occurs in
        /// the [`VkPipelineStageFlag::VertexInput`] pipeline stage.
        VertexAttributeRead = 0x00000004,

        /// [`VkAccessFlag::UniformRead`] specifies read access to a uniform buffer in any
        /// shader pipeline stage.
        UniformRead = 0x00000008,

        /// [`VkAccessFlag::InputAttachmentRead`] specifies read access to an input attachment
        /// within a render pass during subpass shading or fragment shading. Such access occurs in
        /// the [`VkPipelineStageFlag2::SubpassShaderHuawei`] or
        /// [`VkPipelineStageFlag::FragmentShader`] pipeline stage.
        InputAttachmentRead = 0x00000010,

        /// [`VkAccessFlag::ShaderRead`] specifies read access to a uniform texel buffer,
        /// sampled image, storage buffer, physical storage buffer, shader binding table, storage
        /// tensor, storage texel buffer, or storage image in any shader pipeline stage.
        ShaderRead = 0x00000020,

        /// [`VkAccessFlag::ShaderWrite`] specifies write access to a storage buffer, physical
        /// storage buffer, storage tensor, storage texel buffer, or storage image in any shader
        /// pipeline stage.
        ShaderWrite = 0x00000040,

        /// [`VkAccessFlag::ColorAttachmentRead`] specifies read access to a color attachment,
        /// such as via blending (other than advanced blend operations), logic operations or
        /// certain render pass load operations in the [`VkPipelineStageFlag::ColorAttachment`]
        /// pipeline stage or via fragment shader tile image reads in the
        /// [`VkPipelineStageFlag::FragmentShader`] pipeline stage.
        ColorAttachmentRead = 0x00000080,

        /// [`VkAccessFlag::ColorAttachmentWrite`] specifies write access to a color attachment
        /// during a render pass or via certain render pass load, store, and multisample resolve
        /// operations. This includes multisample resolve operations for depth/stencil resolve
        /// attachments. Such access occurs in the [`VkPipelineStageFlag::ColorAttachment`]
        /// pipeline stage.
        ColorAttachmentWrite = 0x00000100,

        /// [`VkAccessFlag::DepthStencilAttachmentRead`] specifies read access to a
        /// depth/stencil attachment, via depth or stencil operations or certain render pass load
        /// operations in the [`VkPipelineStageFlag::EarlyFragmentTests`] or
        /// [`VkPipelineStageFlag::LateFragmentTests`] pipeline stages or via fragment shader
        /// tile image reads in the [`VkPipelineStageFlag::FragmentShader`] pipeline stage.
        DepthStencilAttachmentRead = 0x00000200,

        /// [`VkAccessFlag::DepthStencilAttachmentWrite`] specifies write access to a
        /// depth/stencil attachment, via depth or stencil operations or certain render pass load
        /// and store operations. Such access occurs in the
        /// [`VkPipelineStageFlag::EarlyFragmentTests`] or
        /// [`VkPipelineStageFlag::LateFragmentTests`] pipeline stages.
        DepthStencilAttachmentWrite = 0x00000400,

        /// [`VkAccessFlag::TransferRead`] specifies read access to an image, tensor, or buffer
        /// in a copy operation. Such access occurs in the [`VkPipelineStageFlag2::AllTransfer`]
        /// pipeline stage.
        TransferRead = 0x00000800,

        /// [`VkAccessFlag::TransferWrite`] specifies write access to an image, tensor, or
        /// buffer in a clear or copy operation. Such access occurs in the
        /// [`VkPipelineStageFlag2::AllTransfer`] pipeline stage.
        TransferWrite = 0x00001000,

        /// [`VkAccessFlag::HostRead`] specifies read access by a host operation. Accesses of
        /// this type are not performed through a resource, but directly on memory. Such access
        /// occurs in the [`VkPipelineStageFlag::Host`] pipeline stage.
        HostRead = 0x00002000,

        /// [`VkAccessFlag::HostWrite`] specifies write access by a host operation. Accesses of
        /// this type are not performed through a resource, but directly on memory. Such access
        /// occurs in the [`VkPipelineStageFlag::Host`] pipeline stage.
        HostWrite = 0x00004000,

        /// [`VkAccessFlag::MemoryRead`] specifies all read accesses. It is always valid in any
        /// access mask, and is treated as equivalent to setting all READ access flags that are
        /// valid where it is used.
        MemoryRead = 0x00008000,

        /// [`VkAccessFlag::MemoryWrite`] specifies all write accesses. It is always valid in
        /// any access mask, and is treated as equivalent to setting all `WRITE` access flags that
        /// are valid where it is used.
        MemoryWrite = 0x00010000,

        /// [`VkAccessFlag::TransformFeedbackWriteExt`] specifies write access to a transform
        /// feedback buffer made when transform feedback is active. Such access occurs in the
        /// [`VkPipelineStageFlag::TransformFeedbackExt`] pipeline stage.
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackWriteExt = 0x02000000,

        /// [`VkAccessFlag::TransformFeedbackCounterReadExt`] specifies read access to a
        /// transform feedback counter buffer which is read when [`VkCmdBeginTransformFeedbackExt`]
        /// executes. Such access occurs in the [`VkPipelineStageFlag::TransformFeedbackExt`]
        /// pipeline stage.
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackCounterReadExt = 0x04000000,

        /// [`VkAccessFlag::TransformFeedbackCounterWriteExt`] specifies write access to a
        /// transform feedback counter buffer which is written when
        /// [`VkCmdEndTransformFeedbackExt`] executes. Such access occurs in the
        /// [`VkPipelineStageFlag::TransformFeedbackExt`] pipeline stage.
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackCounterWriteExt = 0x08000000,

        /// [`VkAccessFlag::ConditionalRenderingReadExt`] specifies read access to a predicate
        /// as part of conditional rendering. Such access occurs in the
        /// [`VkPipelineStageFlag::ConditionalRenderingExt`] pipeline stage.
        ///
        /// Provided by [`ext_conditional_rendering`]
        ConditionalRenderingReadExt = 0x00100000,

        /// [`VkAccessFlag::ColorAttachmentReadNoncoherentExt`] specifies read access to color
        /// attachments, including advanced blend operations. Such access occurs in the
        /// [`VkPipelineStageFlag::ColorAttachment`] pipeline stage.
        ///
        /// Provided by [`ext_blend_operation_advanced`]
        ColorAttachmentReadNoncoherentExt = 0x00080000,

        /// [`VkAccessFlag::AccelerationStructureReadKhr`] specifies read access to an
        /// acceleration structure as part of a trace, build, or copy command, or to an
        /// acceleration structure scratch buffer as part of a build command. Such access occurs in
        /// the [`VkPipelineStageFlag::RayTracingShaderKhr`] pipeline stage or
        /// [`VkPipelineStageFlag::AccelerationStructureBuildKhr`] pipeline stage.
        ///
        /// Provided by [`khr_acceleration_structure`]
        AccelerationStructureReadKhr = 0x00200000,

        /// [`VkAccessFlag::AccelerationStructureWriteKhr`] specifies write access to an
        /// acceleration structure or acceleration structure scratch buffer as part of a build or
        /// copy command. Such access occurs in the
        /// [`VkPipelineStageFlag::AccelerationStructureBuildKhr`] pipeline stage.
        ///
        /// Provided by [`khr_acceleration_structure`]
        AccelerationStructureWriteKhr = 0x00400000,

        /// [`VkAccessFlag::FragmentDensityMapReadExt`] specifies read access to a fragment
        /// density map attachment during dynamic fragment density map operations Such access
        /// occurs in the [`VkPipelineStageFlag::FragmentDensityProcessExt`] pipeline stage.
        ///
        /// Provided by [`ext_fragment_density_map`]
        FragmentDensityMapReadExt = 0x00800000,

        /// [`VkAccessFlag::FragmentShadingRateAttachmentReadKhr`] specifies read access to a
        /// fragment shading rate attachment during rasterization. Such access occurs in the
        /// [`VkPipelineStageFlag::FragmentShadingRateAttachmentKhr`] pipeline stage.
        ///
        /// Provided by [`ext_fragment_shading_rate`]
        FragmentShadingRateAttachmentReadKhr = 0x01000000,

        /// [`VkAccessFlag::CommandPreprocessReadExt`] specifies reads from buffer inputs to
        /// [`VkCmdPreprocessGeneratedCommandsExt`]. Such access occurs in the
        /// [`VkPipelineStageFlag::CommandPreprocessExt`] pipeline stage.
        ///
        /// Provided by [`ext_device_generated_commands`]
        CommandPreprocessReadExt = 0x00020000,

        /// [`VkAccessFlag::CommandPreprocessWriteExt`] specifies writes to the target command
        /// buffer preprocess outputs in [`VkCmdPreprocessGeneratedCommandsExt`]. Such access
        /// occurs in the [`VkPipelineStageFlag::CommandPreprocessExt`] pipeline stage.
        ///
        /// Provided by [`ext_device_generated_commands`]
        CommandPreprocessWriteExt = 0x00040000,
    }
}
