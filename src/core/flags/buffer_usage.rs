use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VK_VERSION_1_2, VkBufferView, VkDescriptorSet, VkDeviceMemory,
    VkMemoryHeapFlag, VkPipelineStageFlag,
};

flags! {
    /// Bitmask of [`VkBufferUsageFlag`]s
    ///
    /// # Description
    /// [`VkBufferUsageFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkBufferUsageFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkBufferUsageFlags;

    /// Bitmask specifying allowed usage of a buffer
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkBufferUsageFlag {
        /// [`VkBufferUsageFlag::TransferSrc`] specifies that the buffer can be used as the source
        /// of a transfer command (see the definition of [`VkPipelineStageFlag::Transfer`]).
        TransferSrc = 0x00000001,

        /// [`VkBufferUsageFlag::TransferDst`] specifies that the buffer can be used as the
        /// destination of a transfer command (see the definition of
        /// [`VkPipelineStageFlag::Transfer`]).
        TransferDst = 0x00000002,

        /// [`VkBufferUsageFlag::UniformTexelBuffer`] specifies that the buffer can be used to
        /// create a [`VkBufferView`] suitable for occupying a [`VkDescriptorSet`] slot of type
        /// [`VkDescriptorType::UniformTexelBuffer`].
        UniformTexelBuffer = 0x00000004,

        /// [`VkBufferUsageFlag::StorageTexelBuffer`] specifies that the buffer can be used to
        /// create a [`VkBufferView`] suitable for occupying a [`VkDescriptorSet`] slot of type
        /// [`VkDescriptorType::StorageTexelBuffer`].
        StorageTexelBuffer = 0x00000008,

        /// [`VkBufferUsageFlag::UniformBuffer`] specifies that the buffer can be used in a
        /// [`VkDescriptorBufferInfo`] suitable for occupying a [`VkDescriptorSet`] slot either of
        /// type [`VkDescriptorType::UniformBuffer`] or [`VkDescriptorType::UniformBufferDynamic`].
        UniformBuffer = 0x00000010,

        /// [`VkBufferUsageFlag::StorageBuffer`] specifies that the buffer can be used in a
        /// [`VkDescriptorBufferInfo`] suitable for occupying a [`VkDescriptorSet`] slot either of
        /// type [`VkDescriptorType::StorageBuffer`] or [`VkDescriptorType::StorageBufferDynamic`].
        StorageBuffer = 0x00000020,

        /// [`VkBufferUsageFlag::IndexBuffer`] specifies that the buffer is suitable for passing as
        /// the buffer parameter to [`VkCmdBindIndexBuffer2`] and [`VkCmdBindIndexBuffer`].
        IndexBuffer = 0x00000040,

        /// [`VkBufferUsageFlag::VertexBuffer`] specifies that the buffer is suitable for passing
        /// as an element of the pBuffers array to [`VkCmdBindVertexBuffers`].
        VertexBuffer = 0x00000080,

        /// [`VkBufferUsageFlag::IndirectBuffer`] specifies that the buffer is suitable for passing
        /// as the buffer parameter to [`VkCmdDrawIndirect`], [`VkCmdDrawIndexedIndirect`],
        /// [`VkCmdDrawMeshTasksIndirectNv`], [`VkCmdDrawMeshTasksIndirectCountNv`],
        /// [`VkCmdDrawMeshTasksIndirectExt`], [`VkCmdDrawMeshTasksIndirectCountExt`],
        /// [`VkCmdDrawClusterIndirectHuawei`], or [`VkCmdDispatchIndirect`]. It is also suitable
        /// for passing as the buffer member of [`VkIndirectCommandsStreamNv`], or
        /// `sequences_count_buffer` or `sequences_index_buffer` or `preprocessed_buffer` member of
        /// [`VkGeneratedCommandsInfoNv`]. It is also suitable for passing as the underlying buffer
        /// of either the `preprocess_address` or `sequence_count_address` members of
        /// [`VkGeneratedCommandsInfoExt`].
        IndirectBuffer = 0x00000100,

        /// [`VkBufferUsageFlag::ShaderDeviceAddress`] specifies that the buffer can be used to
        /// retrieve a buffer device address via [`VkGetBufferDeviceAddress`] and use that address
        /// to access the buffer’s memory from a shader.
        ///
        /// Provided by [`VK_VERSION_1_2`]
        ShaderDeviceAddress = 0x00020000,

        /// [`VkBufferUsageFlag::VideoDecodeSrcKhr`] specifies that the buffer can be used as the
        /// source video bitstream buffer in a video decode operation.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeSrcKhr = 0x00002000,

        /// [`VkBufferUsageFlag::VideoDecodeDstKhr`] is reserved for future use.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeDstKhr = 0x00004000,

        /// [`VkBufferUsageFlag::TransformFeedbackBufferExt`] specifies that the buffer is suitable
        /// for using for binding as a transform feedback buffer with
        /// [`VkCmdBindTransformFeedbackBuffers2Ext`] or [`VkCmdBindTransformFeedbackBuffersExt`].
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackBufferExt = 0x00000800,

        /// [`VkBufferUsageFlag::TransformFeedbackCounterBufferExt`] specifies that the buffer is
        /// suitable for using as a counter buffer with [`VkCmdBeginTransformFeedback2Ext`],
        /// [`VkCmdEndTransformFeedback2Ext`], [`VkCmdBeginTransformFeedbackExt`], and
        /// [`VkCmdEndTransformFeedbackExt`].
        ///
        /// Provided by [`ext_transform_feedback`]
        TransformFeedbackCounterBufferExt = 0x00001000,

        /// [`VkBufferUsageFlag::ConditionalRenderingExt`] specifies that the buffer is suitable
        /// for passing as the buffer parameter to [`VkCmdBeginConditionalRenderingExt`].
        ///
        /// Provided by [`ext_conditional_rendering`]
        ConditionalRenderingExt = 0x00000200,

        /// [`VkBufferUsageFlag::ExecutionGraphScratchAmdx`] specifies that the buffer can be used
        /// for as scratch memory for execution graph dispatch.
        ///
        /// Provided by [`amdx_shader_enqueue`]
        ExecutionGraphScratchAmdx = 0x02000000,

        /// [`VkBufferUsageFlag::DescriptorHeapExt`] specifies that the buffer can be used as a
        /// descriptor heap.
        ///
        /// Provided by [`ext_descriptor_heap`]
        DescriptorHeapExt = 0x10000000,

        /// [`VkBufferUsageFlag::AccelerationStructureBuildInputReadOnlyKhr`] specifies that the
        /// buffer is suitable for use as a read-only input to an acceleration structure build.
        ///
        /// Provided by [`khr_acceleration_structure`]
        AccelerationStructureBuildInputReadOnlyKhr = 0x00080000,

        /// [`VkBufferUsageFlag::AccelerationStructureStorageKhr`] specifies that the buffer is
        /// suitable for storage space for a [`VkAccelerationStructureKhr`].
        ///
        /// Provided by [`khr_acceleration_structure`]
        AccelerationStructureStorageKhr = 0x00100000,

        /// [`VkBufferUsageFlag::ShaderBindingTableKhr`] specifies that the buffer is suitable for
        /// use as a Shader Binding Table.
        ///
        /// Provided by [`khr_ray_tracing_pipeline`]
        ShaderBindingTableKhr = 0x00000400,

        /// [`VkBufferUsageFlag::VideoEncodeDstKhr`] specifies that the buffer can be used as the
        /// destination video bitstream buffer in a video encode operation.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeDstKhr = 0x00008000,

        /// [`VkBufferUsageFlag::VideoEncodeSrcKhr`] is reserved for future use.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeSrcKhr = 0x00010000,

        /// [`VkBufferUsageFlag::SamplerDescriptorBufferExt`] specifies that the buffer is suitable
        /// to contain sampler and combined image sampler descriptors when bound as a descriptor
        /// buffer. Buffers containing combined image sampler descriptors must also specify
        /// [`VkBufferUsageFlag::ResourceDescriptorBufferExt`].
        ///
        /// Provided by [`ext_descriptor_buffer`]
        SamplerDescriptorBufferExt = 0x00200000,

        /// [`VkBufferUsageFlag::ResourceDescriptorBufferExt`] specifies that the buffer is
        /// suitable to contain resource descriptors when bound as a descriptor buffer.
        ///
        /// Provided by [`ext_descriptor_buffer`]
        ResourceDescriptorBufferExt = 0x00400000,

        /// [`VkBufferUsageFlag::PushDescriptorsDescriptorBufferExt`] specifies that the buffer,
        /// when bound, can be used by the implementation to support push descriptors when using
        /// descriptor buffers.
        ///
        /// Provided by [`ext_descriptor_buffer`]
        PushDescriptorsDescriptorBufferExt = 0x04000000,

        /// [`VkBufferUsageFlag::MicromapBuildInputReadOnlyExt`] specifies that when building a
        /// [`VkMicromapExt`], the buffer can be used as a read-only micromap build input, which
        /// includes the data and triangleArray parameters
        ///
        /// Provided by [`ext_opacity_micromap`]
        MicromapBuildInputReadOnlyExt = 0x00800000,

        /// [`VkBufferUsageFlag::MicromapStorageExt`] specifies that the buffer can be used to
        /// create [`VkMicromapExt`] objects.
        ///
        /// Provided by [`ext_opacity_micromap`]
        MicromapStorageExt = 0x01000000,

        /// [`VkBufferUsageFlag::TileMemoryQcom`] specifies that the buffer can be bound to
        /// [`VkDeviceMemory`] allocated from a [`VkMemoryHeap`] with the
        /// [`VkMemoryHeapFlag::TileMemoryQcom`] property.
        ///
        /// Provided by [`qcom_tile_memory_heap`]
        TileMemoryQcom = 0x08000000,
    }
}
