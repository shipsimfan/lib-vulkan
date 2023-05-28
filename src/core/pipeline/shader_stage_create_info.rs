use crate::{Loader, NativeLoader, ShaderModule, ShaderStageFlagBits};

pub struct PipelineShaderStageCreateInfo<L: Loader = NativeLoader> {
    pub stage: ShaderStageFlagBits,
    pub module: ShaderModule<L>,
    pub name: String,
}
