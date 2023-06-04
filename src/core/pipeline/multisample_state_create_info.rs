use crate::{
    SampleCountFlagBits, VkPipelineMultisampleStateCreateFlags,
    VkPipelineMultisampleStateCreateInfo, VkStructureType,
};
use std::ptr::null;

pub struct PipelineMultisampleStateCreateInfo {
    pub samples: SampleCountFlagBits,
    pub sample_shading_enable: bool,
    pub min_sample_shading: f32,
    pub sample_mask: Vec<u32>,
    pub alpha_to_coverage_enable: bool,
    pub alpha_to_one_enable: bool,
}

impl PipelineMultisampleStateCreateInfo {
    pub(super) fn into_binding(&self) -> VkPipelineMultisampleStateCreateInfo {
        VkPipelineMultisampleStateCreateInfo {
            s_type: VkStructureType::PipelineMultisampleStateCreateInfo,
            p_next: null(),
            flags: VkPipelineMultisampleStateCreateFlags::default(),
            rasterization_samples: self.samples,
            sample_shading_enable: self.sample_shading_enable as u32,
            min_shading_enable: self.min_sample_shading,
            p_sample_mask: if self.sample_mask.len() == 0 {
                null()
            } else {
                self.sample_mask.as_ptr()
            },
            alpha_to_converage_enable: self.alpha_to_coverage_enable as u32,
            alpha_to_one_enable: self.alpha_to_one_enable as u32,
        }
    }
}

impl Default for PipelineMultisampleStateCreateInfo {
    fn default() -> Self {
        PipelineMultisampleStateCreateInfo {
            samples: SampleCountFlagBits::_1Bit,
            sample_shading_enable: false,
            min_sample_shading: 1.0,
            sample_mask: Vec::new(),
            alpha_to_coverage_enable: false,
            alpha_to_one_enable: false,
        }
    }
}
