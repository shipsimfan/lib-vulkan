use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of pipeline layout creation flag bits
    ///
    /// # Description
    /// [`VkPipelineLayoutCreateFlags`] is a bitmask type for setting a mask of
    /// [`VkPipelineLayoutCreateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkPipelineLayoutCreateFlags;

    /// Pipeline layout creation flag bits
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkPipelineLayoutCreateFlag {
        /// [`VkPipelineLayoutCreateFlag::IndependentSetsExt`] specifies that implementations must
        /// ensure that the properties and/or absence of a particular descriptor set do not
        /// influence any other properties of the pipeline layout. This allows pipelines libraries
        /// linked without [`VkPipelineCreateFlag::LinkTimeOptimizationBitExt`] to be created with
        /// a subset of the total descriptor sets.
        ///
        /// Provided by [`khr_maintenance11`] with [`ext_shader_object`],
        /// [`ext_graphics_pipeline_library`]
        IndependentSetsExt = 0x00000002,

        /// [`VkPipelineLayoutCreateFlag::NoTaskShaderKhr`], when used in combination with
        /// [`VkPipelineLayoutCreateFlag::IndependentSetsExt`], specifies that this pipeline layout
        /// will only be used to draw with shader objects created with
        /// [`VkShaderCreateFlagExt::NoTaskShaderKhr`].
        ///
        /// Provided by [`khr_maintenance11`] with [`ext_shader_object`] and ([`ext_mesh_shader`]
        /// or [`nv_mesh_shader`])
        NoTaskShaderKhr = 0x00000004,
    }
}
