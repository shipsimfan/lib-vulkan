use crate::VkFormat;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_VERSION_1_0, VkPhysicalDeviceLimits, VkVertexInputBindingDescription};

/// Structure specifying vertex input attribute description
///
/// # Valid Usage
///  - If the [`khr_portability_subset`] extension is enabled, and
///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::vertex_attribute_access_beyond_stride`] is
///    [`VK_FALSE`], the sum of `offset` plus the size of the vertex attribute data described by
///    `format` must not be greater than `stride` in the [`VkVertexInputBindingDescription`]
///    referenced in `binding`
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkVertexInputAttributeDescription {
    /// `location` is the shader input location number for this attribute.
    ///
    /// # Valid Usage
    ///  - `location` must be less than [`VkPhysicalDeviceLimits::max_vertex_input_attributes`]
    pub location: u32,

    /// `binding` is the binding number which this attribute takes its data from.
    ///
    /// # Valid Usage
    ///  - `binding` must be less than [`VkPhysicalDeviceLimits::max_vertex_input_bindings`]
    pub binding: u32,

    /// `format` is the size and type of the vertex attribute data.
    ///
    /// # Valid Usage
    ///  - The `format` features of format must contain [`VkFormatFeatureFlag::VertexBuffer`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `format` must be a valid [`VkFormat`] value
    pub format: VkFormat,

    /// `offset` is a byte offset of this attribute relative to the start of an element in the
    /// vertex input binding.
    ///
    /// # Valid Usage
    ///  - `offset` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_vertex_input_attribute_offset`]
    pub offset: u32,
}

const impl Default for VkVertexInputAttributeDescription {
    fn default() -> Self {
        VkVertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: VkFormat::Undefined,
            offset: 0,
        }
    }
}
