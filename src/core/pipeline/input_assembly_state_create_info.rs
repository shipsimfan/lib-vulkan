use crate::PrimitiveTopology;

pub struct PipelineInputAssemblyStateCreateInfo {
    pub topology: PrimitiveTopology,
    pub primitive_restart_enabled: bool,
}

impl Default for PipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        PipelineInputAssemblyStateCreateInfo {
            topology: PrimitiveTopology::TriangleList,
            primitive_restart_enabled: false,
        }
    }
}
