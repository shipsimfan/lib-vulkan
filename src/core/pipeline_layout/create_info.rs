use crate::PushConstantRange;

pub struct PipelineLayoutCreateInfo {
    pub push_constant_ranges: Vec<PushConstantRange>,
}
