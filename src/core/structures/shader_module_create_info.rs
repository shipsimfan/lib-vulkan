use crate::{VkShaderModuleCreateFlags, VkStructureType};
use std::{
    ffi::{c_size_t, c_void},
    ptr::null,
};

/// Structure specifying parameters of a newly created shader module
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkShaderModuleCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::ShaderModuleCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkShaderModuleCreateFlags,

    /// `code_size` is the size, in bytes, of the code pointed to by `code`.
    ///
    /// # Valid Usage
    ///  - If `code` is a pointer to SPIR-V code, `code_size` must be a multiple of 4
    ///  - `code_size` must be greater than 0
    pub code_size: c_size_t,

    /// `code` is a pointer to code that is used to create the shader module. The type and format
    /// of the code is determined from the content of the memory addressed by `code`.
    ///
    /// # Valid Usage
    ///  - If `code` is a pointer to SPIR-V code, `code` must point to valid SPIR-V code, formatted
    ///    and packed as described by the Khronos SPIR-V Specification
    ///  - If `code` is a pointer to SPIR-V code, `code` must adhere to the validation rules
    ///    described by the Validation Rules within a Module section of the SPIR-V Environment
    ///    appendix
    ///  - If `code` is a pointer to SPIR-V code, `code` must declare the Shader capability for
    ///    SPIR-V code
    ///  - If `code` is a pointer to SPIR-V code, `code` must not declare any capability that is
    ///    not supported by the API, as described by the Capabilities section of the SPIR-V
    ///    Environment appendix
    ///  - If `code` is a pointer to SPIR-V code, and `code` declares any of the capabilities
    ///    listed in the SPIR-V Environment appendix, one of the corresponding requirements must be
    ///    satisfied
    ///  - If `code` is a pointer to SPIR-V code, `code` must not declare any SPIR-V extension that
    ///    is not supported by the API, as described by the Extension section of the SPIR-V
    ///    Environment appendix
    ///  - If `code` is a pointer to SPIR-V code, and `code` declares any of the SPIR-V extensions
    ///    listed in the SPIR-V Environment appendix, one of the corresponding requirements must be
    ///    satisfied
    ///  - If the [`nv_glsl_shader`] extension is not enabled, `code` must be a pointer to SPIR-V
    ///    code
    ///  - If `code` is a pointer to GLSL code, it must be valid GLSL code written to the
    ///    `GL_KHR_vulkan_glsl` GLSL extension specification
    ///
    /// # Valid Usage (Implicit)
    ///  - `code` must be a valid pointer to an array of `code_size / 4` [`u32`] values
    pub code: *const u32,
}

const impl Default for VkShaderModuleCreateInfo {
    fn default() -> Self {
        VkShaderModuleCreateInfo {
            r#type: VkStructureType::ShaderModuleCreateInfo,
            next: null(),
            flags: VkShaderModuleCreateFlags::empty(),
            code_size: 0,
            code: null(),
        }
    }
}
