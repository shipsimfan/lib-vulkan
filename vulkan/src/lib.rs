mod bindings;
mod instance;
mod macros;
mod version;
mod vulkan;

pub(self) use macros::*;

pub use crate::vulkan::Vulkan;
pub use bindings::{
    VkApplicationInfo, VkExtensionProperties, VkInstanceCreateFlagBits, VkInstanceCreateFlags,
    VkInstanceCreateInfo, VkLayerProperties, VkResult,
};
pub use instance::VkInstance;
pub use loader::Loader;
pub use native::NativeLoader;
pub use version::{
    VkVersion, VK_API_VERSION_1_0, VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3,
    VK_HEADER_VERSION, VK_HEADER_VERSION_COMPLETE,
};

pub type Result<T> = std::result::Result<T, VkResult>;
