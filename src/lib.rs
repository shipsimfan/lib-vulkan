#![feature(extern_types)]
#![feature(concat_idents)]

mod bindings;
mod common;
mod instance;
mod library;
mod loader;
mod physical_device;

pub use bindings::{
    VkInstance, VkInstanceCreateFlagBits, VkInstanceCreateFlags, VkResult, VkVersion,
    VK_API_VERSION_1_0, VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3,
    VK_HEADER_VERSION, VK_HEADER_VERSION_COMPLETE,
};
pub use common::{ExtensionProperties, LayerProperties};
pub use instance::{ApplicationInfo, Instance, InstanceCreateInfo};
pub use library::Library;
pub use loader::{Loader, NativeLoader};
pub use physical_device::PhysicalDevice;

pub(crate) use bindings::*;

pub type Result<T> = std::result::Result<T, VkResult>;
