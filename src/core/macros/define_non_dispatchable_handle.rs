// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkBuffer, vk_define_non_dispatchable_handle};

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
        #[repr(C)]
        #[derive(Debug)]
        pub struct $object(*mut ::std::ffi::c_void);


        #[cfg(not(target_pointer_width = "64"))]
        $(#[$meta])*
        #[repr(C)]
        #[derive(Debug)]
        pub struct $object(u64);

        impl $object {
            #[doc = std::concat!("Create a new [`", std::stringify!($object), "`] containing `null`")]
            #[cfg(target_pointer_width = "64")]
            pub const fn null() -> $object {
                $object(std::ptr::null_mut())
            }

            #[doc = std::concat!("Create a new [`", std::stringify!($object), "`] containing `null`")]
            #[cfg(not(target_pointer_width = "64"))]
            pub const fn null() -> $object {
                $object(0)
            }

            /// Is this a null pointer?
            #[cfg(target_pointer_width = "64")]
            pub const fn is_null(&self) -> bool {
                self.0.is_null()
            }

            /// Is this a null pointer?
            #[cfg(not(target_pointer_width = "64"))]
            pub const fn is_null(&self) -> bool {
                self.0 == 0
            }

            /// Get the underlying handle as a [`u64`]
            #[cfg(target_pointer_width = "64")]
            pub fn as_u64(&self) -> u64 {
                self.0 as _
            }

            /// Get the underlying handle as a [`u64`]
            #[cfg(not(target_pointer_width = "64"))]
            pub fn as_u64(&self) -> u64 {
                self.0
            }
        }

        unsafe impl Send for $object {}
        unsafe impl Sync for $object {}

        const impl Clone for $object {
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

        #[cfg(target_pointer_width = "64")]
        impl Into<u64> for $object {
            fn into(self) -> u64 {
                self.0 as _
            }
        }

        #[cfg(not(target_pointer_width = "64"))]
        impl Into<u64> for $object {
            fn into(self) -> u64 {
                self.0
            }
        }
    };
}
