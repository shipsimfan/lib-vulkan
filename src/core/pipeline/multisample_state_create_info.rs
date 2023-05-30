use crate::SampleCountFlagBits;

pub struct PipelineMultisampleStateCreateInfo {
    pub samples: SampleCountFlagBits,
    pub sample_shading_enable: bool,
    pub min_sample_shading: f32,
    pub sample_mask: Vec<u32>,
    pub alpha_to_coverage_enable: bool,
    pub alpha_to_one_enable: bool,
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
