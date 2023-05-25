#![feature(extern_types)]
#![feature(concat_idents)]

mod bindings;
mod common;
mod instance;
mod library;
mod loader;
mod physical_device;

pub use bindings::{
    VkExtent3D as Extent3D, VkInstance, VkInstanceCreateFlagBits as InstanceCreateFlagBits,
    VkInstanceCreateFlags as InstanceCreateFlags, VkPhysicalDeviceType as PhysicalDeviceType,
    VkQueueFamilyProperties as QueueFamilyProperties, VkQueueFlagBits as QueueFlagBits,
    VkQueueFlags as QueueFlags, VkResult, VkSampleCountFlagBits as SampleCountFlagBits,
    VkSampleCountFlags as SampleCountFlags, VkVersion as Version, VK_API_VERSION_1_0,
    VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3, VK_HEADER_VERSION,
    VK_HEADER_VERSION_COMPLETE, VK_UUID_SIZE,
};
pub use common::{ExtensionProperties, LayerProperties};
pub use instance::{ApplicationInfo, Instance, InstanceCreateInfo};
pub use library::Library;
pub use loader::{Loader, NativeLoader};
pub use physical_device::{
    PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceLimits, PhysicalDeviceProperties,
    PhysicalDeviceSparseProperties,
};

pub(crate) use bindings::*;
pub(crate) use physical_device::PhysicalDeviceFunctions;

pub type Result<T> = std::result::Result<T, VkResult>;
