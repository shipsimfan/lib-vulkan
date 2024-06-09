// rustdoc imports
#[allow(unused_imports)]
use crate::{vk_define_non_dispatchable_handle, VK_VERSION_1_0};

/// Declare a non-dispatchable object handle
///
///  - `object` is the name of the resulting C type.
///
/// Most Vulkan handle types, such as [`VkBuffer`], are non-dispatchable.
///
/// Provided by [`VK_VERSION_1_0`]
#[macro_export]
macro_rules! vk_define_non_dispatchable_handle {
    (
        $(#[$meta:meta])*
        $object: ident
    ) => {
        #[cfg(target_pointer_width = "64")]
        $(#[$meta])*
        pub type $object = *mut ::std::ffi::c_void;


        #[cfg(not(target_pointer_width = "64"))]
        $(#[$meta])*
        pub type $object = u64;
    };
}
