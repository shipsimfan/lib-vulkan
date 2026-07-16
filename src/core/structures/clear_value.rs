use crate::{VkClearColorValue, VkClearDepthStencilValue};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying a clear value
///
/// # Description
/// This union is used where part of the API requires either color or depth/stencil clear values,
/// depending on the attachment, and defines the initial clear values in the
/// [`VkRenderPassBeginInfo`] structure.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Clone, Copy)]
pub union VkClearValue {
    /// `color` specifies the color image clear values to use when clearing a color image or
    /// attachment.
    pub color: VkClearColorValue,

    /// `depth_stencil` specifies the depth and stencil clear values to use when clearing a
    /// depth/stencil image or attachment.
    pub depth_stencil: VkClearDepthStencilValue,
}

const impl Default for VkClearValue {
    fn default() -> Self {
        VkClearValue {
            color: VkClearColorValue::default(),
        }
    }
}
