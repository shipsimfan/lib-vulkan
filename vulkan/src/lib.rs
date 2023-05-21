mod bindings;
mod version;
mod vulkan;

pub use crate::vulkan::Vulkan;
pub use bindings::{VkLayerProperties, VkResult};
pub use loader::Loader;
pub use version::{
    VkVersion, VK_API_VERSION_1_0, VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3,
    VK_HEADER_VERSION, VK_HEADER_VERSION_COMPLETE,
};

#[cfg(feature = "native_loader")]
pub use native::NativeLoader;

pub type Result<T> = std::result::Result<T, VkResult>;
