use crate::{
    Rect2D, Viewport, VkPipelineViewportStateCreateFlags, VkPipelineViewportStateCreateInfo,
    VkStructureType,
};
use std::ptr::null;

pub enum PipelineViewportState<T> {
    Dynamic(u32),
    Static(Vec<T>),
}

pub struct PipelineViewportStateCreateInfo {
    pub viewports: PipelineViewportState<Viewport>,
    pub scissors: PipelineViewportState<Rect2D>,
}

impl PipelineViewportStateCreateInfo {
    pub(super) fn into_binding(&self) -> VkPipelineViewportStateCreateInfo {
        VkPipelineViewportStateCreateInfo {
            s_type: VkStructureType::PipelineViewportStateCreateInfo,
            p_next: null(),
            flags: VkPipelineViewportStateCreateFlags::default(),
            viewport_count: self.viewports.len(),
            p_viewports: if self.viewports.len() == 0 {
                null()
            } else {
                self.viewports.as_ptr()
            },
            scissor_count: self.scissors.len(),
            p_scissors: if self.scissors.len() == 0 {
                null()
            } else {
                self.scissors.as_ptr()
            },
        }
    }
}

impl<T> PipelineViewportState<T> {
    pub(self) fn len(&self) -> u32 {
        match self {
            Self::Dynamic(_) => 0,
            Self::Static(value) => value.len() as u32,
        }
    }

    pub(self) fn as_ptr(&self) -> *const T {
        match self {
            Self::Dynamic(_) => null(),
            Self::Static(value) => value.as_ptr(),
        }
    }
}
