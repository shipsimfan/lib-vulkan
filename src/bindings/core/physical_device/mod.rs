mod features;
mod functions;
mod handle;
mod limits;
mod properties;
mod sample_count_flags;
mod sparse_properties;
mod r#type;

pub use r#type::VkPhysicalDeviceType;
pub use sample_count_flags::{VkSampleCountFlagBits, VkSampleCountFlags};

pub(crate) use features::VkPhysicalDeviceFeatures;
pub(crate) use functions::{VkGetPhysicalDeviceFeatures, VkGetPhysicalDeviceProperties};
pub(crate) use handle::VkPhysicalDevice;
pub(crate) use limits::VkPhysicalDeviceLimits;
pub(crate) use properties::VkPhysicalDeviceProperties;
pub(crate) use sparse_properties::VkPhysicalDeviceSparseProperties;
