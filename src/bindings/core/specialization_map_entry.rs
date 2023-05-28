#[repr(C)]
pub struct VkSpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}
