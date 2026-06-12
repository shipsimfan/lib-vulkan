// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_VERSION_1_0};

/// Supported primitive topologies
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkPrimitiveTopology {
    /// [`VkPrimitiveTopology::PointList`] specifies a series of separate point primitives.
    PointList = 0,

    /// [`VkPrimitiveTopology::LineList`] specifies a series of separate line primitives.
    LineList = 1,

    /// [`VkPrimitiveTopology::LineStrip`] specifies a series of connected line primitives with
    /// consecutive lines sharing a vertex.
    LineStrip = 2,

    /// [`VkPrimitiveTopology::TriangleList`] specifies a series of separate triangle primitives.
    TriangleList = 3,

    /// [`VkPrimitiveTopology::TriangleStrip`] specifies a series of connected triangle primitives
    /// with consecutive triangles sharing an edge.
    TriangleStrip = 4,

    /// [`VkPrimitiveTopology::TriangleFan`] specifies a series of connected triangle primitives
    /// with all triangles sharing a common vertex. If the [`khr_portability_subset`] extension is
    /// enabled, and [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::triangle_fans`] is
    /// [`VK_FALSE`], then triangle fans are not supported by the implementation, and
    /// [`VkPrimitiveTopology::TriangleFan`] must not be used.
    TriangleFan = 5,

    /// [`VkPrimitiveTopology::LineListWithAdjacency`] specifies a series of separate line
    /// primitives with adjacency.
    LineListWithAdjacency = 6,

    /// [`VkPrimitiveTopology::LineStripWithAdjacency`] specifies a series of connected line
    /// primitives with adjacency, with consecutive primitives sharing three vertices.
    LineStripWithAdjacency = 7,

    /// [`VkPrimitiveTopology::TriangleListWithAdjacency`] specifies a series of separate triangle
    /// primitives with adjacency.
    TriangleListWithAdjacency = 8,

    /// [`VkPrimitiveTopology::TriangleStripWithAdjacency`] specifies connected triangle primitives
    /// with adjacency, with consecutive triangles sharing an edge.
    TriangleStripWithAdjacency = 9,

    /// [`VkPrimitiveTopology::PatchList`] specifies separate patch primitives.
    PatchList = 10,
}
