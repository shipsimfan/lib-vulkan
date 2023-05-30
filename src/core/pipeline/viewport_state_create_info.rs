use crate::{Rect2D, Viewport};

pub enum PipelineViewportState<T> {
    Dynamic(u32),
    Static(Vec<T>),
}

pub struct PipelineViewportStateCreateInfo {
    pub viewports: PipelineViewportState<Viewport>,
    pub scissors: PipelineViewportState<Rect2D>,
}
