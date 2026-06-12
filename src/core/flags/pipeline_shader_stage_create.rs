use crate::macros::flags;

flags! {
    /// Bitmask of [`VkPipelineShaderStageCreateFlag`]
    ///
    /// # Description
    /// [`VkPipelineShaderStageCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkPipelineShaderStageCreateFlag`].
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkPipelineShaderStageCreateFlags;


    /// Bitmask controlling how a pipeline shader stage is created
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkPipelineShaderStageCreateFlag {
        /// [`VkPipelineShaderStageCreateFlag::AllowVaryingSubgroupSize`] specifies that the
        /// `subgroup_size` may vary in the shader stage.
        AllowVaryingSubgroupSize = 0x00000001,

        /// [`VkPipelineShaderStageCreateFlag::RequireFullSubgroups`] specifies that the subgroup
        /// sizes must be launched with all invocations active in the task, mesh, or compute stage.
        RequireFullSubgroups = 0x00000002,
    }
}
