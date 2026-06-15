// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Specify the bind point of a pipeline object to a command buffer
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkPipelineBindPoint {
    /// [`VkPipelineBindPoint::Graphics`] specifies binding as a graphics pipeline.
    Graphics = 0,

    /// [`VkPipelineBindPoint::Compute`] specifies binding as a compute pipeline.
    Compute = 1,

    /// [`VkPipelineBindPoint::ExecutionGraphAmdx`] specifies binding as an execution graph
    /// pipeline.
    ///
    /// Provided by [`amdx_shader_enqueue`]
    ExecutionGraphAmdx = 1000134000,

    /// [`VkPipelineBindPoint::RayTracingKhr`] specifies binding as a ray tracing pipeline.
    ///
    /// Provided by [`khr_ray_tracing_pipeline`]
    RayTracingKhr = 1000165000,

    /// [`VkPipelineBindPoint::SubpassShadingHuawei`] specifies binding as a subpass shading
    /// pipeline.
    ///
    /// Provided by [`huawei_subpass_shading`]
    SubpassShadingHuawei = 1000369003,

    /// Provided by [`arm_data_graph`]
    DataGraphArm = 1000507000,
}
