use crate::flags64;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkAccessFlag};

flags64! {
    /// 64-bit mask of access flags
    ///
    /// # Description
    /// [`VkAccessFlags2`] is a bitmask type for setting a mask of zero or more [`VkAccessFlag2`].
    ///
    /// Provided by [`VK_VERSION_1_3`]
    pub struct VkAccessFlags2;


    /// Access flags for [`VkAccessFlags2`]
    ///
    /// # Description
    /// Certain access types are only performed by a subset of pipeline stages, as described in
    /// more detail for [`VkAccessFlag`].
    ///
    /// Provided by [`VK_VERSION_1_3`]
    pub enum VkAccessFlag2 {
        /// [`VkAccessFlag2::None`] specifies no accesses.
        None = 0x0,

        /// [`VkAccessFlag2::IndirectCommandRead`] specifies read access to command data read from
        /// indirect buffers as part of an indirect build, trace, drawing or dispatch command. Such
        /// access occurs in the [`VkPipelineStageFlag2::DrawIndirect`] pipeline stage. It also
        /// specifies read access to command data read from indirect buffers as part of a copy
        /// command with access occurring in the [`VkPipelineStageFlag2::CopyIndirectKhr`] pipeline
        /// stage.
        IndirectCommandRead = 0x00000001,

        /// [`VkAccessFlag2::IndexRead`] specifies read access to an index buffer as part of an
        /// indexed drawing command, bound by [`VkCmdBindIndexBuffer2`] and
        /// [`VkCmdBindIndexBuffer`]. Such access occurs in the
        /// [`VkPipelineStageFlag2::IndexInput`] pipeline stage.
        IndexRead = 0x00000002,

        /// [`VkAccessFlag2::VertexAttributeRead`] specifies read access to a vertex buffer as part
        /// of a drawing command, bound by [`VkCmdBindVertexBuffers`]. Such access occurs in the
        /// [`VkPipelineStageFlag2::VertexAttributeInput`] pipeline stage.
        VertexAttributeRead = 0x00000004,

        /// [`VkAccessFlag2::UniformRead`] specifies read access to a uniform buffer in any shader
        /// pipeline stage.
        UniformRead = 0x00000008,

        /// [`VkAccessFlag2::InputAttachmentRead`] specifies read access to an input attachment
        /// within a render pass during subpass shading or fragment shading. Such access occurs in
        /// the [`VkPipelineStageFlag2::SubpassShaderHuawei`] or
        /// [`VkPipelineStageFlag2::FragmentShader`] pipeline stage.
        InputAttachmentRead = 0x00000010,

        /// [`VkAccessFlag2::ShaderRead`] is equivalent to the logical OR of:
        ///  - [`VkAccessFlag2::ShaderSampledRead`]
        ///  - [`VkAccessFlag2::ShaderStorageRead`]
        ///  - [`VkAccessFlag2::ShaderBindingTableReadKhr`]
        ///  - [`VkAccessFlag2::ShaderTileAttachmentReadQcom`]
        ShaderRead = 0x00000020,

        /// [`VkAccessFlag2::ShaderWrite`] is equivalent to [`VkAccessFlag2::ShaderStorageWrite`].
        ShaderWrite = 0x00000040,

        /// [`VkAccessFlag2::ColorAttachmentRead`] specifies read access to a color attachment,
        /// such as via blending (other than advanced blend operations), logic operations or
        /// certain render pass load operations in the
        /// [`VkPipelineStageFlag2::ColorAttachmentOutput`] pipeline stage or via fragment shader
        /// tile image reads in the [`VkPipelineStageFlag2::FragmentShader`] pipeline stage.
        ColorAttachmentRead = 0x00000080,

        /// [`VkAccessFlag2::ColorAttachmentWrite`] specifies write access to a color attachment
        /// during a render pass or via certain render pass load, store, and multisample resolve
        /// operations. This includes multisample resolve operations for depth/stencil resolve
        /// attachments. Such access occurs in the [`VkPipelineStageFlag2::ColorAttachmentOutput`]
        /// pipeline stage.
        ColorAttachmentWrite = 0x00000100,

        /// [`VkAccessFlag2::DepthStencilAttachmentRead`] specifies read access to a depth/stencil
        /// attachment, via depth or stencil operations or certain render pass load operations in
        /// the [`VkPipelineStageFlag2::EarlyFragmentTests`] or
        /// [`VkPipelineStageFlag2::LateFragmentTests`] pipeline stages or via fragment shader tile
        /// image reads in the [`VkPipelineStageFlag2::FragmentShader`] pipeline stage.
        DepthStencilAttachmentRead = 0x00000200,

        /// [`VkAccessFlag2::DepthStencilAttachmentWrite`] specifies write access to a
        /// depth/stencil attachment, via depth or stencil operations or certain render pass load
        /// and store operations. Such access occurs in the
        /// [`VkPipelineStageFlag2::EarlyFragmentTests`] or
        /// [`VkPipelineStageFlag2::LateFragmentTests`] pipeline stages.
        DepthStencilAttachmentWrite = 0x00000400,

        /// [`VkAccessFlag2::TransferRead`] specifies read access to an image or buffer in a copy
        /// operation. Such access occurs in the [`VkPipelineStageFlag2::Copy`],
        /// [`VkPipelineStageFlag2::Blit`], or [`VkPipelineStageFlag2::Resolve`] pipeline stages.
        TransferRead = 0x00000800,

        /// [`VkAccessFlag2::TransferWrite`] specifies write access to an image or buffer in a
        /// clear or copy operation. Such access occurs in the [`VkPipelineStageFlag2::Copy`],
        /// [`VkPipelineStageFlag2::Blit`], [`VkPipelineStageFlag2::Clear`], or
        /// [`VkPipelineStageFlag2::Resolve`] pipeline stages.
        TransferWrite = 0x00001000,

        /// [`VkAccessFlag2::HostRead`] specifies read access by a host operation. Accesses of this
        /// type are not performed through a resource, but directly on memory. Such access occurs
        /// in the [`VkPipelineStageFlag2::Host`] pipeline stage.
        HostRead = 0x00002000,

        /// [`VkAccessFlag2::HostWrite`] specifies write access by a host operation. Accesses of
        /// this type are not performed through a resource, but directly on memory. Such access
        /// occurs in the [`VkPipelineStageFlag2::Host`] pipeline stage.
        HostWrite = 0x00004000,

        /// [`VkAccessFlag2::MemoryRead`] specifies all read accesses. It is always valid in any
        /// access mask, and is treated as equivalent to setting all `READ` access flags that are
        /// valid where it is used.
        MemoryRead = 0x00008000,

        /// [`VkAccessFlag2::MemoryWrite`] specifies all write accesses. It is always valid in any
        /// access mask, and is treated as equivalent to setting all `WRITE` access flags that are
        /// valid where it is used.
        MemoryWrite = 0x00010000,

        /// [`VkAccessFlag2::ShaderSampledRead`] specifies read access to a uniform texel buffer or
        /// sampled image in any shader pipeline stage.
        ShaderSampledRead = 0x100000000,

        /// [`VkAccessFlag2::ShaderStorageRead`] specifies read access to a storage buffer,
        /// physical storage buffer, storage texel buffer, or storage image in any shader pipeline
        /// stage.
        ShaderStorageRead = 0x200000000,

        /// [`VkAccessFlag2::ShaderStorageWrite`] specifies write access to a storage buffer,
        /// physical storage buffer, storage texel buffer, or storage image in any shader pipeline
        /// stage.
        ShaderStorageWrite = 0x400000000,

        /// [`VkAccessFlag2::VideoDecodeReadKhr`] specifies read access to an image or buffer
        /// resource in a video decode operation. Such access occurs in the
        /// [`VkPipelineStageFlag2::VideoDecodeKhr`] pipeline stage.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeReadKhr = 0x800000000,

        /// [`VkAccessFlag2::VideoDecodeWriteKhr`] specifies write access to an image or buffer
        /// resource in a video decode operation. Such access occurs in the
        /// [`VkPipelineStageFlag2::VideoDecodeKhr`] pipeline stage.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeWriteKhr = 0x1000000000,

        /// [`VkAccessFlag2::SamplerHeapReadExt`] specifies read access to a sampler heap in any
        /// shader pipeline stage.
        ///
        /// Provided by [`ext_descriptor_heap`]
        SamplerHeapReadExt = 0x200000000000000,

        /// [`VkAccessFlag2::ResourceHeapReadExt`] specifies read access to a resource heap in any
        /// shader pipeline stage.
        ///
        /// Provided by [`ext_descriptor_heap`]
        ResourceHeapReadExt = 0x400000000000000,

        /// [`VkAccessFlag2::VideoEncodeReadKhr`] specifies read access to an image or buffer
        /// resource in a video encode operation. Such access occurs in the
        /// [`VkPipelineStageFlag2::VideoEncodeKhr`] pipeline stage.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeReadKhr = 0x2000000000,

        /// [`VkAccessFlag2::VideoEncodeWriteKhr`] specifies write access to an image or buffer
        /// resource in a video encode operation. Such access occurs in the
        /// [`VkPipelineStageFlag2::VideoEncodeKhr`] pipeline stage.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeWriteKhr = 0x4000000000,

        /// [`VkAccessFlag2::ShaderTileAttachmentReadQcom`] specifies read access to a tile
        /// attachment. Such access occurs in the [`VkPipelineStageFlag2::FragmentShader`] or
        /// [`VkPipelineStageFlag2::ComputeShader`] pipeline stages.
        ///
        /// Provided by [`Qcom_tile_shading`]
        ShaderTileAttachmentReadQcom = 0x8000000000000,

        /// [`VkAccessFlag2::ShaderTileAttachmentWriteQcom`] specifies write access to a tile
        /// attachment. Such access occurs in the [`VkPipelineStageFlag2::FragmentShader`] or
        /// [`VkPipelineStageFlag2::ComputeShader`] pipeline stages.
        ///
        /// Provided by [`Qcom_tile_shading`]
        ShaderTileAttachmentWriteQcom = 0x10000000000000,

        /// [`VkAccessFlag2::TransformFeedbackWriteExt`] specifies write access to a transform
        /// feedback buffer made when transform feedback is active. Such access occurs in the
        /// [`VkPipelineStageFlag2::TransformFeedbackExt`] pipeline stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_transform_feedback`]
        TransformFeedbackWriteExt = 0x02000000,

        /// [`VkAccessFlag2::TransformFeedbackCounterReadExt`] specifies read access to a transform
        /// feedback counter buffer which is read when [`VkCmdBeginTransformFeedbackExt`] executes.
        /// Such access occurs in the [`VkPipelineStageFlag2::TransformFeedbackExt`] pipeline
        /// stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_transform_feedback`]
        TransformFeedbackCounterReadExt = 0x04000000,

        /// [`VkAccessFlag2::TransformFeedbackCounterWriteExt`] specifies write access to a
        /// transform feedback counter buffer which is written when
        /// [`VkCmdEndTransformFeedbackExt`] executes. Such access occurs in the
        /// [`VkPipelineStageFlag2::TransformFeedbackExt`] pipeline stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_transform_feedback`]
        TransformFeedbackCounterWriteExt = 0x08000000,

        /// [`VkAccessFlag2::ConditionalRenderingReadExt`] specifies read access to a predicate as
        /// part of conditional rendering. Such access occurs in the
        /// [`VkPipelineStageFlag2::ConditionalRenderingExt`] pipeline stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_conditional_rendering`]
        ConditionalRenderingReadExt = 0x00100000,

        /// [`VkAccessFlag2::CommandPreprocessReadExt`] specifies reads from buffer inputs to
        /// [`VkCmdPreprocessGeneratedCommandsExt`]. Such access occurs in the
        /// [`VkPipelineStageFlag2::CommandPreprocessExt`] pipeline stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_device_generated_commands`]
        CommandPreprocessReadExt = 0x00020000,

        /// [`VkAccessFlag2::CommandPreprocessWriteExt`] specifies writes to the target command
        /// buffer preprocess outputs. Such access occurs in the
        /// [`VkPipelineStageFlag2::CommandPreprocessExt`] pipeline stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_device_generated_commands`]
        CommandPreprocessWriteExt = 0x00040000,

        /// [`VkAccessFlag2::FragmentShadingRateAttachmentReadKhr`] specifies read access to a
        /// fragment shading rate attachment during rasterization. Such access occurs in the
        /// [`VkPipelineStageFlag2::FragmentShadingRateAttachmentKhr`] pipeline stage.
        ///
        /// Provided by [`khr_fragment_shading_rate`] with [`khr_synchronization2`]
        FragmentShadingRateAttachmentReadKhr = 0x00800000,

        /// [`VkAccessFlag2::AccelerationStructureReadKhr`] specifies read access to an
        /// acceleration structure as part of a trace, build, or copy command, or to an
        /// acceleration structure scratch buffer as part of a build command. Such access occurs in
        /// the [`VkPipelineStageFlag2::RayTracingShaderKhr`] pipeline stage or
        /// [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`] pipeline stage.
        ///
        /// Provided by [`khr_acceleration_structure`] with [`khr_synchronization2`]
        AccelerationStructureReadKhr = 0x00200000,

        /// [`VkAccessFlag2::AccelerationStructureWriteKhr`] specifies write access to an
        /// acceleration structure or acceleration structure scratch buffer as part of a build or
        /// copy command. Such access occurs in the
        /// [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`] pipeline stage.
        ///
        /// Provided by [`khr_acceleration_structure`] with [`khr_synchronization2`]
        AccelerationStructureWriteKhr = 0x00400000,

        /// [`VkAccessFlag2::FragmentDensityMapReadExt`] specifies read access to a fragment
        /// density map attachment during dynamic fragment density map operations. Such access
        /// occurs in the [`VkPipelineStageFlag2::FragmentDensityProcessExt`] pipeline stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_fragment_density_map`]
        FragmentDensityMapReadExt = 0x01000000,

        /// [`VkAccessFlag2::ColorAttachmentReadNoncoherentExt`] specifies read access to color
        /// attachments, including advanced blend operations. Such access occurs in the
        /// [`VkPipelineStageFlag2::ColorAttachmentOutput`] pipeline stage.
        ///
        /// Provided by [`khr_synchronization2`] with [`ext_blend_operation_advanced`]
        ColorAttachmentReadNoncoherentExt = 0x00080000,

        /// [`VkAccessFlag2::DescriptorBufferReadExt`] specifies read access to a descriptor buffer
        /// in any shader pipeline stage.
        ///
        /// Provided by [`ext_descriptor_buffer`]
        DescriptorBufferReadExt = 0x20000000000,

        /// [`VkAccessFlag2::InvocationMaskReadHuawei`] specifies read access to an invocation mask
        /// image in the [`VkPipelineStageFlag2::InvocationMaskHuawei`] pipeline stage.
        ///
        /// Provided by [`huawei_invocation_mask`]
        InvocationMaskReadHuawei = 0x8000000000,

        /// [`VkAccessFlag2::ShaderBindingTableReadKhr`] specifies read access to a shader binding
        /// table in any shader pipeline stage.
        ///
        /// Provided by [`khr_ray_tracing_maintenance1`] with ([`khr_synchronization2`] or
        /// [`VK_VERSION_1_3`]) and [`khr_ray_tracing_pipeline`]
        ShaderBindingTableReadKhr = 0x10000000000,

        /// [`VkAccessFlag2::MicromapReadExt`] specifies read access to a micromap object. Such
        /// access occurs in the [`VkPipelineStageFlag2::MicromapBuildExt`] and
        /// [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`] pipeline stages.
        ///
        /// Provided by [`ext_opacity_micromap`]
        MicromapReadExt = 0x100000000000,

        /// [`VkAccessFlag2::MicromapWriteExt`] specifies write access to a micromap object. Such
        /// access occurs in the [`VkPipelineStageFlag2::MicromapBuildExt`] pipeline stage.
        ///
        /// Provided by [`ext_opacity_micromap`]
        MicromapWriteExt = 0x200000000000,

        /// [`VkAccessFlag2::OpticalFlowReadNv`] specifies read access to an image or buffer
        /// resource as part of a optical flow operation. Such access occurs in the
        /// [`VkPipelineStageFlag2::OpticalFlowNv`] pipeline stage.
        ///
        /// Provided by [`Nv_optical_flow`]
        OpticalFlowReadNv = 0x40000000000,

        /// [`VkAccessFlag2::OpticalFlowWriteNv`] specifies write access to an image or buffer
        /// resource as part of a optical flow operation. Such access occurs in the
        /// [`VkPipelineStageFlag2::OpticalFlowNv`] pipeline stage.
        ///
        /// Provided by [`Nv_optical_flow`]
        OpticalFlowWriteNv = 0x80000000000,

        /// [`VkAccessFlag2::DataGraphReadArm`] specifies read access to resources in the
        /// [`VkPipelineStageFlag2::DataGraphArm`] pipeline stage.
        ///
        /// Provided by [`arm_data_graph`]
        DataGraphReadArm = 0x800000000000,

        /// [`VkAccessFlag2::DataGraphWriteArm`] specifies write access to resources in the
        /// [`VkPipelineStageFlag2::DataGraphArm`] pipeline stage.
        ///
        /// Provided by [`arm_data_graph`]
        DataGraphWriteArm = 0x1000000000000,

        /// [`VkAccessFlag2::MemoryDecompressionReadExt`] specifies read access to memory in
        /// decompression commands [`VkCmdDecompressMemoryExt`] and
        /// [`VkCmdDecompressMemoryIndirectCountExt`]. Such access occurs in
        /// [`VkPipelineStageFlag2::MemoryDecompressionExt`] pipeline stage.
        ///
        /// Provided by [`ext_memory_decompression`]
        MemoryDecompressionReadExt = 0x80000000000000,

        /// [`VkAccessFlag2::MemoryDecompressionWriteExt`] specifies write access to memory in
        /// decompression commands [`VkCmdDecompressMemoryExt`] and
        /// [`VkCmdDecompressMemoryIndirectCountExt`]. Such access occurs in
        /// [`VkPipelineStageFlag2::MemoryDecompressionExt`] pipeline stage.
        ///
        /// Provided by [`ext_memory_decompression`]
        MemoryDecompressionWriteExt = 0x100000000000000,
    }
}
