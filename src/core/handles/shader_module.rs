use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a shader module object
    ///
    /// Shader modules contain shader code and one or more entry points. Shaders are selected from
    /// a shader module by specifying an entry point as part of pipeline creation. The stages of a
    /// pipeline can use shaders that come from different modules. The shader code defining a
    /// shader module must be in the SPIR-V format.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkShaderModule
);
