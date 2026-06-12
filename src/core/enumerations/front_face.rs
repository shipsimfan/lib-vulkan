// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Interpret polygon front-facing orientation
///
/// # Description
/// The first step of polygon rasterization is to determine whether the triangle is back-facing or
/// front-facing. This determination is made based on the sign of the (clipped or unclipped)
/// polygon’s area computed in framebuffer coordinates.
///
/// Any triangle which is not front-facing is back-facing, including zero-area triangles.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkFrontFace {
    /// [`VkFrontFace::CounterClockwise`] specifies that a triangle with positive area is
    /// considered front-facing.
    CounterClockwise = 0,

    /// [`VkFrontFace::Clockwise`] specifies that a triangle with negative area is considered
    /// front-facing.
    Clockwise = 1,
}
