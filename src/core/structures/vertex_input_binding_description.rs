use crate::VkVertexInputRate;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkPhysicalDeviceLimits};

/// Structure specifying vertex input binding description
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkVertexInputBindingDescription {
    /// `binding` is the binding number that this structure describes.
    ///
    /// # Valid Usage
    ///  - `binding` must be less than [`VkPhysicalDeviceLimits::max_vertex_input_bindings`]
    pub binding: u32,

    /// `stride` is the byte stride between consecutive elements within the buffer.
    ///
    /// # Valid Usage
    ///  - `stride` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_vertex_input_binding_stride`]
    ///  - If the [`khr_portability_subset`] extension is enabled, stride must be a multiple of,
    ///    and at least as large as,
    ///    [`VkPhysicalDevicePortabilitySubsetPropertiesKhr::min_vertex_input_binding_stride_alignment`]
    pub stride: u32,

    /// `input_rate` is a [`VkVertexInputRate`] value specifying whether vertex attribute
    /// addressing is a function of the vertex index or of the instance index.
    ///
    /// # Valid Usage
    ///  - `input_rate` must be a valid [`VkVertexInputRate`] value
    pub input_rate: VkVertexInputRate,
}

impl const Default for VkVertexInputBindingDescription {
    fn default() -> Self {
        VkVertexInputBindingDescription {
            binding: 0,
            stride: 0,
            input_rate: VkVertexInputRate::Vertex,
        }
    }
}
