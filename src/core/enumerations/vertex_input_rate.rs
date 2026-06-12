// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Specify rate at which vertex attributes are pulled from buffers
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkVertexInputRate {
    /// [`VkVertexInputRate::Vertex`] specifies that vertex attribute addressing is a function of
    /// the vertex index.
    Vertex = 0,

    /// [`VkVertexInputRate::Instance`] specifies that vertex attribute addressing is a function of
    /// the instance index.
    Instance = 1,
}
