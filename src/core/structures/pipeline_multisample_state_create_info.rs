use crate::{
    VK_FALSE, VkBool32, VkPipelineMultisampleStateCreateFlags, VkSampleCountFlag, VkSampleMask,
    VkStructureType, util::NextChain,
};
use std::{
    ffi::{c_float, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VkDynamicState};

/// Structure specifying parameters of a newly created pipeline multisample state
///
/// # Description
/// Each bit in the sample mask is associated with a unique sample index as defined for the
/// coverage mask. Each bit `b` for mask word `w` in the sample mask corresponds to sample index
/// `i`, where `i = 32 × w + b`. `sample_mask` has a length equal to `⌈`rasterization_samples` / 32⌉`
/// words.
///
/// If `sample_mask` is [`null`], it is treated as if the mask has all bits set to 1.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkPipelineMultisampleStateCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineMultisampleStateCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkPipelineCoverageModulationStateCreateInfoNv`],
    ///    [`VkPipelineCoverageReductionStateCreateInfoNv`],
    ///    [`VkPipelineCoverageToColorStateCreateInfoNv`], or
    ///    [`VkPipelineSampleLocationsStateCreateInfoExt`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkPipelineMultisampleStateCreateFlags,

    /// `rasterization_samples` is a [`VkSampleCountFlag`] value specifying the number of samples
    /// used in rasterization. This value is ignored for the purposes of setting the number of
    /// samples used in rasterization if the pipeline is created with the
    /// [`VkDynamicState::RasterizationSamplesExt`] dynamic state set, but if
    /// [`VkDynamicState::SampleMaskExt`] dynamic state is not set, it is still used to define the
    /// size of the `sample_mask` array as described below.
    ///
    /// # Valid Usage
    ///  - If the [`nv_framebuffer_mixed_samples`] extension is enabled, and the
    ///    `coverage_reduction_mode` feature is not enabled, or the `next` chain does not contain
    ///    [`VkPipelineCoverageReductionStateCreateInfoNv`], or
    ///    [`VkPipelineCoverageReductionStateCreateInfoNv::coverage_reduction_mode`] is not set to
    ///    [`VkCoverageReductionModeNv::TruncateNv`], and the subpass has any color attachments,
    ///    and `rasterization_samples` is greater than the number of color samples, then sample
    ///    shading must not be enabled
    ///
    /// # Valid Usage (Implicit)
    ///  - `rasterization_samples` must be a valid [`VkSampleCountFlag`] value
    pub rasterization_samples: VkSampleCountFlag,

    /// `sample_shading_enable` can be used to enable Sample Shading.
    ///
    /// # Valid Usage
    ///  - If the `sample_rate_shading` feature is not enabled, `sample_shading_enable` must be
    ///    [`VK_FALSE`]
    pub sample_shading_enable: VkBool32,

    /// `min_sample_shading` specifies a minimum fraction of sample shading if
    /// `sample_shading_enable` is [`VK_TRUE`].
    ///
    /// # Valid Usage
    ///  - `min_sample_shading` must be in the range `[0, 1]`
    pub min_sample_shading: c_float,

    /// `sample_mask` is a pointer to an array of [`VkSampleMask`] values used in the sample mask
    /// test.
    pub sample_mask: *const VkSampleMask,

    /// `alpha_to_coverage_enable` controls whether a temporary coverage value is generated based
    /// on the alpha component of the fragment’s first color output.
    pub alpha_to_coverage_enable: VkBool32,

    /// `alpha_to_one_enable` controls whether the alpha component of the fragment’s first color
    /// output is replaced with one.
    ///
    /// # Valid Usage
    ///  - If the `alpha_to_one` feature is not enabled, `alpha_to_one_enable` must be [`VK_FALSE`]
    pub alpha_to_one_enable: VkBool32,
}

impl const Default for VkPipelineMultisampleStateCreateInfo {
    fn default() -> Self {
        VkPipelineMultisampleStateCreateInfo {
            r#type: VkStructureType::PipelineMultisampleStateCreateInfo,
            next: null(),
            flags: VkPipelineMultisampleStateCreateFlags::empty(),
            rasterization_samples: VkSampleCountFlag::_1,
            sample_shading_enable: VK_FALSE,
            min_sample_shading: 0.0,
            sample_mask: null(),
            alpha_to_coverage_enable: VK_FALSE,
            alpha_to_one_enable: VK_FALSE,
        }
    }
}

impl NextChain for VkPipelineMultisampleStateCreateInfo {
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
