use crate::VkDescriptorType;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying descriptor pool size
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorPoolSize {
    /// `r#type` is the type of descriptor.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be a valid VkDescriptorType value
    pub r#type: VkDescriptorType,

    /// `descriptor_count` is the number of descriptors of that type to allocate. If `r#type` is
    /// [`VkDescriptorType::InlineUniformBlock`] then `descriptor_count` is the number of bytes to
    /// allocate for descriptors of this type.
    ///
    /// # Valid Usage
    ///  - `descriptor_count` must be greater than 0
    ///  - If `r#type` is [`VkDescriptorType::InlineUniformBlock`] then `descriptor_count` must be
    ///    a multiple of 4
    pub descriptor_count: u32,
}

const impl Default for VkDescriptorPoolSize {
    fn default() -> Self {
        VkDescriptorPoolSize {
            r#type: VkDescriptorType::Sampler,
            descriptor_count: 0,
        }
    }
}
