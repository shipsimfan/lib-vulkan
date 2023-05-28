use crate::{VertexInputAttributeDescription, VertexInputBindingDescription};

pub struct VertexInputStateCreateInfo {
    pub bindings: Vec<VertexInputBindingDescription>,
    pub attributes: Vec<VertexInputAttributeDescription>,
}
