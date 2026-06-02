use crate::{VkPipelineStageFlags2, VkSemaphore, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkPipelineStageFlag2};

/// Structure specifying a semaphore signal or wait operation
///
/// # Description
/// Whether this structure defines a semaphore wait or signal operation is defined by how it is
/// used. The first synchronization scope of a semaphore signal operation or the second
/// synchronization scope of a semaphore wait operation defined by this structure are limited to
/// operations in stages indicated by `stage_mask`.
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSemaphoreSubmitInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::SemaphoreSubmitInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `semaphore` is a [`VkSemaphore`] affected by this operation.
    ///
    /// # Valid Usage
    ///  - If the device that `semaphore` was created on is not a device group, `device_index` must
    ///    be 0
    ///  - If the device that `semaphore` was created on is a device group, `device_index` must be
    ///    a valid device index
    ///
    /// # Valid Usage (Implicit)
    ///  - `semaphore` must be a valid [`VkSemaphore`] handle
    pub semaphore: VkSemaphore,

    /// `value` is either the value used to signal semaphore or the value waited on by semaphore,
    /// if semaphore is a timeline semaphore. Otherwise it is ignored.
    pub value: u64,

    /// `stage_mask` is a [`VkPipelineStageFlags2`] mask of pipeline stages which limit the first
    /// synchronization scope of a semaphore signal operation, or second synchronization scope of a
    /// semaphore wait operation as described in the semaphore wait operation and semaphore signal
    /// operation sections of the synchronization chapter.
    ///
    /// # Valid Usage
    ///  - If the `geometry_shader` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::GeometryShader`]
    ///  - If the `tessellation_shader` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::TessellationControlShader`] or
    ///    [`VkPipelineStageFlag2::TessellationEvaluationShader`]
    ///  - If the `conditional_rendering` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::ConditionalRenderingExt`]
    ///  - If the `fragment_density_map` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::FragmentDensityProcessExt`]
    ///  - If the `transform_feedback` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::TransformFeedbackExt`]
    ///  - If the `mesh_shader` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::MeshShaderExt`]
    ///  - If the `task_shader` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::TaskShaderExt`]
    ///  - If neither of the `shading_rate_image` or the `attachment_fragment_shading_rate`
    ///    features are enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::FragmentShadingRateAttachmentKhr`]
    ///  - If the `subpass_shading` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::SubpassShaderHuawei`]
    ///  - If the `invocation_mask` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::InvocationMaskHuawei`]
    ///  - If neither the `nv_ray_tracing` extension or the `ray_tracing_pipeline` feature are
    ///    enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::RayTracingShaderKhr`]
    ///  - If the `acceleration_structure` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`]
    ///  - If the `ray_tracing_maintenance1` feature is not enabled, `stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`]
    ///  - If the [`VkPhysicalDeviceOpacityMicromapFeaturesExt::micromap`] feature is not enabled,
    ///    `stage_mask` must not contain [`VkPipelineStageFlag2::MicromapBuildExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `stage_mask` must be a valid combination of [`VkPipelineStageFlag2`] values
    pub stage_mask: VkPipelineStageFlags2,

    /// `device_index` is the index of the device within a device group that executes the semaphore
    /// wait or signal operation.
    pub device_index: u32,
}

impl const Default for VkSemaphoreSubmitInfo {
    fn default() -> Self {
        VkSemaphoreSubmitInfo {
            r#type: VkStructureType::SemaphoreSubmitInfo,
            next: null(),
            semaphore: VkSemaphore::null(),
            value: 0,
            stage_mask: VkPipelineStageFlags2::empty(),
            device_index: 0,
        }
    }
}

impl NextChain for VkSemaphoreSubmitInfo {
    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: *const c_void) {
        self.next = next;
    }
}
