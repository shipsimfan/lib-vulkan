#[non_exhaustive]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkVertexInputRate {
    Vertex = 0,
    Instance = 1,
}
