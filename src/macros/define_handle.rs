/// Decleare a dispatchable object handle
///
/// [`vk_define_handle`] defines a dispatchable handle type.
///
///  * `object` is the name of the resulting type.
///
/// The only dispatchable handle types are those related to device and instance management such as
/// [`VkDevice`].
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
