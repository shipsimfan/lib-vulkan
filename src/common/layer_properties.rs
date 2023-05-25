use crate::VkVersion;

pub struct LayerProperties {
    pub name: String,
    pub version: VkVersion,
    pub implementation_version: u32,
    pub description: String,
}
