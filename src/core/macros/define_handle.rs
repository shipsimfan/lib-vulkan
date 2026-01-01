// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkDevice, vk_define_handle};

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
        #[repr(C)]
        #[derive(Debug)]
        pub struct $object(*mut ::std::ffi::c_void);

        impl $object {
            #[doc = std::concat!("Create a new [`", std::stringify!($object), "`] containing `null`")]
            pub const fn null() -> $object {
                $object(std::ptr::null_mut())
            }

            /// Is this a null pointer?
            pub const fn is_null(&self) -> bool {
                self.0.is_null()
            }
        }

        unsafe impl Send for $object {}

        impl const Clone for $object {
            fn clone(&self) -> Self {
                $object(self.0)
            }
        }

        impl Copy for $object {}

        impl PartialEq for $object {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl Eq for $object {}
    };
}
