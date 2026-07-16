use crate::VkShaderStageFlags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkPhysicalDeviceLimits, VkShaderStageFlag};

/// Structure specifying a push constant range
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPushConstantRange {
    /// `stage_flags` is a set of stage flags describing the shader stages that will access a range
    /// of push constants. If a particular stage is not included in the range, then accessing
    /// members of that range of push constants from the corresponding shader stage will return
    /// undefined values.
    ///
    /// # Valid Usage (Implicit)
    ///  - `stage_flags` must be a valid combination of [`VkShaderStageFlag`]s values
    ///  - `stage_flags` must not be 0
    pub stage_flags: VkShaderStageFlags,

    /// `offset` is the start offset consumed by the range. Offset is in units of bytes and must be
    /// a multiple of 4. The layout of the push constant variables is specified in the shader.
    ///
    /// # Valid Usage
    ///  - `offset` must be less than [`VkPhysicalDeviceLimits::max_push_constants_size`]
    ///  - `offset` must be a multiple of 4
    pub offset: u32,

    /// `size` is the size consumed by the range. Size is in units of bytes and must be a multiple
    /// of 4. The layout of the push constant variables is specified in the shader.
    ///
    /// # Valid Usage
    ///  - `size` must be greater than 0
    ///  - `size` must be a multiple of 4
    ///  - `size` must be less than or equal to [`VkPhysicalDeviceLimits::max_push_constants_size`]
    ///    minus `offset`
    pub size: u32,
}

const impl Default for VkPushConstantRange {
    fn default() -> Self {
        VkPushConstantRange {
            stage_flags: VkShaderStageFlags::empty(),
            offset: 0,
            size: 0,
        }
    }
}
