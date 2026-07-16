use crate::{
    VK_FALSE, VkBool32, VkCullModeFlags, VkFrontFace, VkPipelineRasterizationStateCreateFlags,
    VkPolygonMode, VkStructureType, util::NextChain,
};
use std::{
    ffi::{c_float, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkCullModeFlag};

/// Structure specifying parameters of a newly created pipeline rasterization state
///
/// # Description
/// The application can also add a [`VkPipelineRasterizationStateRasterizationOrderAmd`] structure
/// to the `next` chain of a [`VkPipelineRasterizationStateCreateInfo`] structure. This structure
/// enables selecting the rasterization order to use when rendering with the corresponding graphics
/// pipeline as described in Rasterization Order.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkPipelineRasterizationStateCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineRasterizationStateCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkDepthBiasRepresentationInfoExt`],
    ///    [`VkPipelineRasterizationConservativeStateCreateInfoExt`],
    ///    [`VkPipelineRasterizationDepthClipStateCreateInfoExt`],
    ///    [`VkPipelineRasterizationLineStateCreateInfo`],
    ///    [`VkPipelineRasterizationProvokingVertexStateCreateInfoExt`],
    ///    [`VkPipelineRasterizationStateRasterizationOrderAmd`], or
    ///    [`VkPipelineRasterizationStateStreamCreateInfoExt`]
    ///  - The ``r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkPipelineRasterizationStateCreateFlags,

    /// `depth_clamp_enable` controls whether to clamp the fragment’s depth values as described in
    /// Depth Test. If the pipeline is not created with
    /// [`VkPipelineRasterizationDepthClipStateCreateInfoExt`] present then enabling depth clamp
    /// will also disable clipping primitives to the z planes of the frustum as described in
    /// Primitive Clipping. Otherwise depth clipping is controlled by the state set in
    /// [`VkPipelineRasterizationDepthClipStateCreateInfoExt`].
    ///
    /// # Valid Usage
    ///  - If the `depth_clamp` feature is not enabled, `depth_clamp_enable` must be [`VK_FALSE`]
    pub depth_clamp_enable: VkBool32,

    /// `rasterizer_discard_enable` controls whether primitives are discarded immediately before
    /// the rasterization stage.
    pub rasterizer_discard_enable: VkBool32,

    /// `polygon_mode` is the triangle rendering mode. See [`VkPolygonMode`].
    ///
    /// # Valid Usage
    ///  - If the `fill_mode_non_solid` feature is not enabled, `polygon_mode` must be
    ///    [`VkPolygonMode::Fill`] or [`VkPolygonMode::FillRectangleNv`]
    ///  - If the [`nv_fill_rectangle`] extension is not enabled, `polygon_mode` must not be
    ///    [`VkPolygonMode::FillRectangleNv`]
    ///  - If the [`khr_portability_subset`] extension is enabled, and
    ///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::point_polygons`] is [`VK_FALSE`], and
    ///    `rasterizer_discard_enable` is [`VK_FALSE`], `polygon_mode` must not be
    ///    [`VkPolygonMode::Point`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `polygon_mode` must be a valid [`VkPolygonMode`] value
    pub polygon_mode: VkPolygonMode,

    /// `cull_mode` is the triangle facing direction used for primitive culling. See
    /// [`VkCullModeFlag`].
    ///
    /// # Valid Usage (Implicit)
    ///  - `cull_mode` must be a valid combination of [`VkCullModeFlag`] values
    pub cull_mode: VkCullModeFlags,

    /// `front_face` is a [`VkFrontFace`] value specifying the front-facing triangle orientation to
    /// be used for culling.
    ///
    /// # Valid Usage (Implicit)
    ///  - `front_face` must be a valid [`VkFrontFace`] value
    pub front_face: VkFrontFace,

    /// `depth_bias_enable` controls whether to bias fragment depth values.
    pub depth_bias_enable: VkBool32,

    /// `depth_bias_constant_factor` is a scalar factor controlling the constant depth value added
    /// to each fragment.
    pub depth_bias_constant_factor: c_float,

    /// `depth_bias_clamp` is the maximum (or minimum) depth bias of a fragment.
    pub depth_bias_clamp: c_float,

    /// `depth_bias_slope_factor` is a scalar factor applied to a fragment’s slope in depth bias
    /// calculations.
    pub depth_bias_slope_factor: c_float,

    /// `line_width` is the width of rasterized line segments.
    pub line_width: c_float,
}

const impl Default for VkPipelineRasterizationStateCreateInfo {
    fn default() -> Self {
        VkPipelineRasterizationStateCreateInfo {
            r#type: VkStructureType::PipelineRasterizationStateCreateInfo,
            next: null(),
            flags: VkPipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: VK_FALSE,
            rasterizer_discard_enable: VK_FALSE,
            polygon_mode: VkPolygonMode::Fill,
            cull_mode: VkCullModeFlags::empty(),
            front_face: VkFrontFace::CounterClockwise,
            depth_bias_enable: VK_FALSE,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 0.0,
        }
    }
}

impl NextChain for VkPipelineRasterizationStateCreateInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
