use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_2, VK_VERSION_1_4, VkPipelineCreateFlag};

flags! {
    /// Bitmask of [`VkDescriptorSetLayoutCreateFlag`]s
    ///
    /// # Description
    /// [`VkDescriptorSetLayoutCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkDescriptorSetLayoutCreateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkDescriptorSetLayoutCreateFlags;

    /// Bitmask specifying descriptor set layout properties
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkDescriptorSetLayoutCreateFlag {
        /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] specifies that descriptor sets
        /// using this layout must be allocated from a descriptor pool created with the
        /// [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] bit set. Descriptor set layouts created
        /// with this bit set have alternate limits for the maximum number of descriptors per-stage
        /// and per-pipeline layout. The non-`update_after_bind` limits only count descriptors in
        /// sets created without this flag. The `update_after_bind` limits count all descriptors,
        /// but the limits may be higher than the non-`update_after_bind` limits.
        ///
        /// Provided by [`VK_VERSION_1_2`]
        UpdateAfterBindPool = 0x00000002,

        /// [`VkDescriptorSetLayoutCreateFlag::PushDescriptor`] specifies that descriptor sets must
        /// not be allocated using this layout, and descriptors are instead pushed by
        /// [`VkCmdPushDescriptorSet`].
        ///
        /// Provided by [`VK_VERSION_1_4`]
        PushDescriptor = 0x00000001,

        /// [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`] specifies that this layout
        /// must only be used with descriptor buffers.
        ///
        /// Provided by [`ext_descriptor_buffer`]
        DescriptorBufferExt = 0x00000010,

        /// [`VkDescriptorSetLayoutCreateFlag::EmbeddedImmutableSamplersExt`] specifies that this
        /// is a layout only containing immutable samplers that can be bound by
        /// [`VkCmdBindDescriptorBufferEmbeddedSamplersExt`]. Unlike normal immutable samplers,
        /// embedded immutable samplers do not require the application to provide them in a
        /// descriptor buffer.
        ///
        /// Provided by [`ext_descriptor_buffer`]
        EmbeddedImmutableSamplersExt = 0x00000020,

        /// [`VkDescriptorSetLayoutCreateFlag::IndirectBindableNv`] specifies that descriptor sets
        /// using this layout allows them to be bound with compute pipelines that are created with
        /// [`VkPipelineCreateFlag::IndirectBindableNv`] flag set to be used in Device-Generated
        /// Commands.
        ///
        /// Provided by [`nv_device_generated_commands_compute`]
        IndirectBindableNv = 0x00000080,

        /// [`VkDescriptorSetLayoutCreateFlag::HostOnlyPoolExt`] specifies that descriptor sets
        /// using this layout must be allocated from a descriptor pool created with the
        /// [`VkDescriptorPoolCreateFlag::HostOnlyExt`] bit set. Descriptor set layouts created
        /// with this bit have no expressible limit for maximum number of descriptors per-stage.
        /// Host descriptor sets are limited only by available host memory, but may be limited for
        /// implementation specific reasons. Implementations may limit the number of supported
        /// descriptors to `update_after_bind` limits or non-`update_after_bind` limits, whichever
        /// is larger.
        ///
        /// Provided by [`ext_mutable_descriptor_type`]
        HostOnlyPoolExt = 0x00000004,

        /// [`VkDescriptorSetLayoutCreateFlag::PerStageNv`] specifies that binding numbers in
        /// descriptor sets using this layout may represent different resources and/or types of
        /// resources in each stage.
        ///
        /// Provided by [`nv_per_stage_descriptor_set`]
        PerStageNv = 0x00000040,
    }
}
