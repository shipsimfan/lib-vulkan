use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_3};

flags! {
    /// Bitmask of [`VkAccessFlags`]
    ///
    /// # Description
    /// [`VkAccessFlags`] is a bitmask type for setting a mask of zero or more [`VkAccessFlag`].
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

        /// [`VkAccessFlag::IndirectCommandReadBit`] specifies read access to indirect command data
        /// read as part of an indirect build, trace, drawing or dispatching command. Such access
        /// occurs in the [`VkPipelineStageFlag::DrawIndirectBit`] pipeline stage.
        IndirectCommandReadBit = 0x00000001,

        /// [`VkAccessFlag::IndexReadBit`] specifies read access to an index buffer as part of an
        /// indexed drawing command, bound by [`VkCmdBindIndexBuffer2`] and
        /// [`VkCmdBindIndexBuffer`]. Such access occurs in the
        /// [`VkPipelineStageFlag::VertexInputBit`] pipeline stage.
        IndexReadBit = 0x00000002,

        /// [`VkAccessFlag::VertexAttributeReadBit`] specifies read access to a vertex buffer as
        /// part of a drawing command, bound by [`VkCmdBindVertexBuffers`]. Such access occurs in
        /// the [`VkPipelineStageFlag::VertexInputBit`] pipeline stage.
        VertexAttributeReadBit = 0x00000004,

        /// [`VkAccessFlag::UniformReadBit`] specifies read access to a uniform buffer in any
        /// shader pipeline stage.
        UniformReadBit = 0x00000008,

        /// [`VkAccessFlag::InputAttachmentReadBit`] specifies read access to an input attachment
        /// within a render pass during subpass shading or fragment shading. Such access occurs in
        /// the [`VkPipelineStageFlag2::SubpassShaderBitHuawei`] or
        /// [`VkPipelineStageFlag::FragmentShaderBit`] pipeline stage.
        InputAttachmentReadBit = 0x00000010,

        /// [`VkAccessFlag::ShaderReadBit`] specifies read access to a uniform texel buffer,
        /// sampled image, storage buffer, physical storage buffer, shader binding table, storage
        /// tensor, storage texel buffer, or storage image in any shader pipeline stage.
        ShaderReadBit = 0x00000020,

        /// [`VkAccessFlag::ShaderWriteBit`] specifies write access to a storage buffer, physical
        /// storage buffer, storage tensor, storage texel buffer, or storage image in any shader
        /// pipeline stage.
        ShaderWriteBit = 0x00000040,

        /// [`VkAccessFlag::ColorAttachmentReadBit`] specifies read access to a color attachment,
        /// such as via blending (other than advanced blend operations), logic operations or
        /// certain render pass load operations in the [`VkPipelineStageFlag::ColorAttachmentBit`]
        /// pipeline stage or via fragment shader tile image reads in the
        /// [`VkPipelineStageFlag::FragmentShaderBit`] pipeline stage.
        ColorAttachmentReadBit = 0x00000080,

        /// [`VkAccessFlag::ColorAttachmentWriteBit`] specifies write access to a color attachment
        /// during a render pass or via certain render pass load, store, and multisample resolve
        /// operations. This includes multisample resolve operations for depth/stencil resolve
        /// attachments. Such access occurs in the [`VkPipelineStageFlag::ColorAttachmentBit`]
        /// pipeline stage.
        ColorAttachmentWriteBit = 0x00000100,

        /// [`VkAccessFlag::DepthStencilAttachmentReadBit`] specifies read access to a
        /// depth/stencil attachment, via depth or stencil operations or certain render pass load
        /// operations in the [`VkPipelineStageFlag::EarlyFragmentTestsBit`] or
        /// [`VkPipelineStageFlag::LateFragmentTestsBit`] pipeline stages or via fragment shader
        /// tile image reads in the [`VkPipelineStageFlag::FragmentShaderBit`] pipeline stage.
        DepthStencilAttachmentReadBit = 0x00000200,

        /// [`VkAccessFlag::DepthStencilAttachmentWriteBit`] specifies write access to a
        /// depth/stencil attachment, via depth or stencil operations or certain render pass load
        /// and store operations. Such access occurs in the
        /// [`VkPipelineStageFlag::EarlyFragmentTestsBit`] or
        /// [`VkPipelineStageFlag::LateFragmentTestsBit`] pipeline stages.
        DepthStencilAttachmentWriteBit = 0x00000400,

        /// [`VkAccessFlag::TransferReadBit`] specifies read access to an image, tensor, or buffer
        /// in a copy operation. Such access occurs in the [`VkPipelineStageFlag2::AllTransferBit`]
        /// pipeline stage.
        TransferReadBit = 0x00000800,

        /// [`VkAccessFlag::TransferWriteBit`] specifies write access to an image, tensor, or
        /// buffer in a clear or copy operation. Such access occurs in the
        /// [`VkPipelineStageFlag2::AllTransferBit`] pipeline stage.
        TransferWriteBit = 0x00001000,

        /// [`VkAccessFlag::HostReadBit`] specifies read access by a host operation. Accesses of
        /// this type are not performed through a resource, but directly on memory. Such access
        /// occurs in the [`VkPipelineStageFlag::HostBit`] pipeline stage.
        HostReadBit = 0x00002000,

        /// [`VkAccessFlag::HostWriteBit`] specifies write access by a host operation. Accesses of
        /// this type are not performed through a resource, but directly on memory. Such access
        /// occurs in the [`VkPipelineStageFlag::HostBit`] pipeline stage.
        HostWriteBit = 0x00004000,

        /// [`VkAccessFlag::MemoryReadBit`] specifies all read accesses. It is always valid in any
        /// access mask, and is treated as equivalent to setting all READ access flags that are
        /// valid where it is used.
        MemoryReadBit = 0x00008000,

        /// [`VkAccessFlag::MemoryWriteBit`] specifies all write accesses. It is always valid in
        /// any access mask, and is treated as equivalent to setting all `WRITE` access flags that
        /// are valid where it is used.
        MemoryWriteBit = 0x00010000,

        /// [`VkAccessFlag::TransformFeedbackWriteBitExt`] specifies write access to a transform
        /// feedback buffer made when transform feedback is active. Such access occurs in the
        /// [`VkPipelineStageFlag::TransformFeedbackBitExt`] pipeline stage.
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackWriteBitExt = 0x02000000,

        /// [`VkAccessFlag::TransformFeedbackCounterReadBitExt`] specifies read access to a
        /// transform feedback counter buffer which is read when [`VkCmdBeginTransformFeedbackExt`]
        /// executes. Such access occurs in the [`VkPipelineStageFlag::TransformFeedbackBitExt`]
        /// pipeline stage.
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackCounterReadBitExt = 0x04000000,

        /// [`VkAccessFlag::TransformFeedbackCounterWriteBitExt`] specifies write access to a
        /// transform feedback counter buffer which is written when
        /// [`VkCmdEndTransformFeedbackExt`] executes. Such access occurs in the
        /// [`VkPipelineStageFlag::TransformFeedbackBitExt`] pipeline stage.
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackCounterWriteBitExt = 0x08000000,

        /// [`VkAccessFlag::ConditionalRenderingReadBitExt`] specifies read access to a predicate
        /// as part of conditional rendering. Such access occurs in the
        /// [`VkPipelineStageFlag::ConditionalRenderingBitExt`] pipeline stage.
        ///
        /// Provided by [`ext_conditional_rendering`]
        ConditionalRenderingReadBitExt = 0x00100000,

        /// [`VkAccessFlag::ColorAttachmentReadNoncoherentBitExt`] specifies read access to color
        /// attachments, including advanced blend operations. Such access occurs in the
        /// [`VkPipelineStageFlag::ColorAttachmentBit`] pipeline stage.
        ///
        /// Provided by [`ext_blend_operation_advanced`]
        ColorAttachmentReadNoncoherentBitExt = 0x00080000,

        /// [`VkAccessFlag::AccelerationStructureReadBitKhr`] specifies read access to an
        /// acceleration structure as part of a trace, build, or copy command, or to an
        /// acceleration structure scratch buffer as part of a build command. Such access occurs in
        /// the [`VkPipelineStageFlag::RayTracingShaderBitKhr`] pipeline stage or
        /// [`VkPipelineStageFlag::AccelerationStructureBuildBitKhr`] pipeline stage.
        ///
        /// Provided by [`khr_acceleration_structure`]
        AccelerationStructureReadBitKhr = 0x00200000,

        /// [`VkAccessFlag::AccelerationStructureWriteBitKhr`] specifies write access to an
        /// acceleration structure or acceleration structure scratch buffer as part of a build or
        /// copy command. Such access occurs in the
        /// [`VkPipelineStageFlag::AccelerationStructureBuildBitKhr`] pipeline stage.
        ///
        /// Provided by [`khr_acceleration_structure`]
        AccelerationStructureWriteBitKhr = 0x00400000,

        /// [`VkAccessFlag::FragmentDensityMapReadBitExt`] specifies read access to a fragment
        /// density map attachment during dynamic fragment density map operations Such access
        /// occurs in the [`VkPipelineStageFlag::FragmentDensityProcessBitExt`] pipeline stage.
        ///
        /// Provided by [`ext_fragment_density_map`]
        FragmentDensityMapReadBitExt = 0x00800000,

        /// [`VkAccessFlag::FragmentShadingRateAttachmentReadBitKhr`] specifies read access to a fragment shading rate attachment during rasterization. Such access occurs in the [`VkPipelineStageFlag::FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR pipeline stage.
        ///
        /// Provided by [`ext_fragment_shading_rate`]
        FragmentShadingRateAttachmentReadBitKhr = 0x01000000,

        /// [`VkAccessFlag::COMMAND_PREPROCESS_READ_BIT_EXT specifies reads from buffer inputs to vkCmdPreprocessGeneratedCommandsEXT. Such access occurs in the [`VkPipelineStageFlag::COMMAND_PREPROCESS_BIT_EXT pipeline stage.
        ///
        /// Provided by [`ext_device_generated_commands`]
        CommandPreprocessReadBitExt = 0x00020000,

        /// [`VkAccessFlag::COMMAND_PREPROCESS_WRITE_BIT_EXT specifies writes to the target command buffer preprocess outputs in vkCmdPreprocessGeneratedCommandsEXT. Such access occurs in the [`VkPipelineStageFlag::COMMAND_PREPROCESS_BIT_EXT pipeline stage.
        ///
        /// Provided by [`ext_device_generated_commands`]
        CommandPreprocessWriteBitExt = 0x00040000,
    }
}
