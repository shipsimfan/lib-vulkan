use crate::PrimitiveTopology;

pub struct PipelineInputAssemblyStateCreateInfo {
    topology: PrimitiveTopology,
    primitive_restart_enabled: bool,
}

impl Default for PipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        PipelineInputAssemblyStateCreateInfo {
            topology: PrimitiveTopology::TriangleList,
            primitive_restart_enabled: false,
        }
    }
}
