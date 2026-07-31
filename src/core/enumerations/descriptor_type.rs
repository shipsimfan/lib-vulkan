// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_3};

/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkDescriptorType {
    /// [`VkDescriptorType::Sampler`] specifies a sampler descriptor.
    Sampler = 0,

    /// [`VkDescriptorType::CombinedImageSampler`] specifies a combined image sampler descriptor.
    CombinedImageSampler = 1,

    /// [`VkDescriptorType::SampledImage`] specifies a sampled image descriptor.
    SampledImage = 2,

    /// [`VkDescriptorType::StorageImage`] specifies a storage image descriptor.
    StorageImage = 3,

    /// [`VkDescriptorType::UniformTexelBuffer`] specifies a uniform texel buffer descriptor.
    UniformTexelBuffer = 4,

    /// [`VkDescriptorType::StorageTexelBuffer`] specifies a storage texel buffer descriptor.
    StorageTexelBuffer = 5,

    /// [`VkDescriptorType::UniformBuffer`] specifies a uniform buffer descriptor.
    UniformBuffer = 6,

    /// [`VkDescriptorType::StorageBuffer`] specifies a storage buffer descriptor.
    StorageBuffer = 7,

    /// [`VkDescriptorType::UniformBufferDynamic`] specifies a dynamic uniform buffer descriptor.
    UniformBufferDynamic = 8,

    /// [`VkDescriptorType::StorageBufferDynamic`] specifies a dynamic storage buffer descriptor.
    StorageBufferDynamic = 9,

    /// [`VkDescriptorType::InputAttachment`] specifies an input attachment descriptor.
    InputAttachment = 10,

    /// [`VkDescriptorType::InlineUniformBlock`] specifies an inline uniform block.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    InlineUniformBlock = 1000138000,

    /// [`VkDescriptorType::AccelerationStructureKhr`] specifies an acceleration structure
    /// descriptor.
    ///
    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureKhr = 1000150000,

    /// [`VkDescriptorType::AccelerationStructureNv`] specifies an acceleration structure
    /// descriptor.
    ///
    /// Provided by [`nv_ray_tracing`]
    AccelerationStructureNv = 1000165000,

    /// [`VkDescriptorType::SampleWeightImageQcom`] specifies a sampled weight image descriptor.
    ///
    /// Provided by [`qcom_image_processing`]
    SampleWeightImageQcom = 1000440000,

    /// [`VkDescriptorType::BlockMatchImageQcom`] specifies a block matching image descriptor.
    ///
    /// Provided by [`qcom_image_processing`]
    BlockMatchImageQcom = 1000440001,

    /// [`VkDescriptorType::TensorArm`] specifies a storage tensor descriptor.
    ///
    /// Provided by [`arm_tensors`]
    TensorArm = 1000460000,

    /// [`VkDescriptorType::MutableExt`] specifies a descriptor of mutable type.
    ///
    /// Provided by [`ext_mutable_descriptor_type`]
    MutableExt = 1000351000,

    /// [`VkDescriptorType::PartitionedAccelerationStructureNv`] specifies a partitioned
    /// acceleration structure descriptor.
    ///
    /// Provided by [`nv_partitioned_acceleration_structure`]
    PartitionedAccelerationStructureNv = 1000570000,
}
