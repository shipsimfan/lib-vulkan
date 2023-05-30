use crate::{VertexInputAttributeDescription, VertexInputBindingDescription};

pub struct PipelineVertexInputStateCreateInfo {
    pub bindings: Vec<VertexInputBindingDescription>,
    pub attributes: Vec<VertexInputAttributeDescription>,
}
