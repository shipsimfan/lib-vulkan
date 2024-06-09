use crate::vk_define_non_dispatchable_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

vk_define_non_dispatchable_handle!(
    /// Opaque handle to a debug messenger object
    ///
    /// The debug messenger will provide detailed feedback on the application’s use of Vulkan when
    /// events of interest occur. When an event of interest does occur, the debug messenger will
    /// submit a debug message to the debug callback that was provided during its creation.
    /// Additionally, the debug messenger is responsible with filtering out debug messages that the
    /// callback is not interested in and will only provide desired debug messages.
    ///
    /// Provided by [`ext_debug_utils`]
    VkDebugUtilsMessengerEXT
);
