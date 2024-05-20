// rustdoc imports
#[allow(unused_imports)]
use crate::{vk_define_handle, VK_VERSION_1_0};

/// Decleare a dispatchable object handle
///
/// [`vk_define_handle`] defines a dispatchable handle type.
///
///  * `object` is the name of the resulting type.
///
/// The only dispatchable handle types are those related to device and instance management such as
/// [`VkDevice`].
///
/// Provided by [`VK_VERSION_1_0`]
#[macro_export]
macro_rules! vk_define_handle {
    (
        $(#[$meta:meta])*
        $object: ident
    ) => {
        $(#[$meta])*
        pub type $object = *mut ::std::ffi::c_void;
    };
}
