mod enumerate_instance_version;
mod get_instance_proc_addr;

pub use enumerate_instance_version::{VkEnumerateInstanceVersion, VK_ENUMERATE_INSTANCE_VERSION};
pub use get_instance_proc_addr::{
    vkGetInstanceProcAddr, VkGetInstanceProcAddr, VK_GET_INSTANCE_PROC_ADDR,
};
