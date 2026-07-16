use std::ffi::c_float;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying a clear depth stencil value
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VkClearDepthStencilValue {
    /// `depth` is the clear value for the depth aspect of the depth/stencil attachment. It is a
    /// floating-point value which is automatically converted to the attachment’s format.
    ///
    /// # Valid Usage
    ///  - Unless the `ext_depth_range_unrestricted` extension is enabled depth must be between 0.0
    ///    and 1.0, inclusive
    pub depth: c_float,

    /// `stencil` is the clear value for the stencil aspect of the depth/stencil attachment. It is
    /// a 32-bit integer value which is converted to the attachment’s format by taking the
    /// appropriate number of LSBs.
    pub stencil: u32,
}

const impl Default for VkClearDepthStencilValue {
    fn default() -> Self {
        Self {
            depth: 0.0,
            stencil: 0,
        }
    }
}
