use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a buffer view object
    ///
    /// A buffer view represents a contiguous range of a buffer and a specific format to be used to
    /// interpret the data. Buffer views are used to enable shaders to access buffer contents using
    /// image operations. In order to create a valid buffer view, the buffer must have been created
    /// with at least one of the following usage flags:
    ///  - [`VkBufferUsageFlag::UniformTexelBuffer`]
    ///  - [`VkBufferUsageFlag::StorageTexelBuffer`]
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkBufferView
);
