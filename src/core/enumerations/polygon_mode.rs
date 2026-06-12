// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_TRUE, VK_VERSION_1_0};

/// Control polygon rasterization mode
///
/// # Description
/// These modes affect only the final rasterization of polygons: in particular, a polygon’s
/// vertices are shaded and the polygon is clipped and possibly culled before these modes are
/// applied.
///
/// If [`VkPhysicalDeviceMaintenance5Properties::polygon_mode_point_size`] is [`VK_TRUE`], the
/// point size of the final rasterization of polygons is taken from `point_size` when polygon mode
/// is [`VkPolygonMode::Point`].
///
/// Otherwise, if [`VkPhysicalDeviceMaintenance5Properties::polygon_mode_point_size`] is
/// [`VK_FALSE`], the point size of the final rasterization of polygons is 1.0 when polygon mode is
/// [`VkPolygonMode::Point`].
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkPolygonMode {
    /// [`VkPolygonMode::Fill`] specifies that polygons are rendered using the polygon
    /// rasterization rules.
    Fill = 0,

    /// [`VkPolygonMode::Line`] specifies that polygon edges are drawn as line segments.
    Line = 1,

    /// [`VkPolygonMode::Point`] specifies that polygon vertices are drawn as points.
    Point = 2,

    /// [`VkPolygonMode::FillRectangleNv`] specifies that polygons are rendered using polygon
    /// rasterization rules, modified to consider a sample within the primitive if the sample
    /// location is inside the axis-aligned bounding box of the triangle after projection. Note
    /// that the barycentric weights used in attribute interpolation can extend outside the range
    /// [0,1] when these primitives are shaded. Special treatment is given to a sample position on
    /// the boundary edge of the bounding box. In such a case, if two rectangles lie on either side
    /// of a common edge (with identical endpoints) on which a sample position lies, then exactly
    /// one of the triangles must produce a fragment that covers that sample during rasterization.
    ///
    /// Polygons rendered in [`VkPolygonMode::FillRectangleNv`] mode may be clipped by the
    /// frustum or by user clip planes. If clipping is applied, the triangle is culled rather than
    /// clipped.
    ///
    /// Area calculation and facingness are determined for [`VkPolygonMode::FillRectangleNv`] mode
    /// using the triangle’s vertices.
    ///
    /// Provided by [`nv_fill_rectangle`]
    FillRectangleNv = 1000153000,
}
