/// Define a new set of flags
macro_rules! flags {
    {
        $(#[$struct_meta: meta])*
        pub struct $struct_name: ident;

        $(#[$enum_meta: meta])*
        pub enum $enum_name: ident {$(
            $(#[$variant_meta: meta])*
            $variant: ident = $value: expr,
        )*}
    } => {
        $(#[$struct_meta])*
        #[repr(C)]
        #[derive(Debug)]
        pub struct $struct_name(pub $crate::VkFlags);

        $(#[$enum_meta])*
        #[repr(C)]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $enum_name {$(
            $(#[$variant_meta])*
            $variant = $value,
        )*}

        impl $struct_name {
            #[doc = std::concat!(" Creates a new [`", std::stringify!($struct_name), "`] with no flags set")]
            pub const fn new() -> $struct_name {
                $struct_name::from_flags(0)
            }

            #[doc = std::concat!(" Creates a new [`", std::stringify!($struct_name), "`] from `f`")]
            pub const fn from_flag(f: $enum_name) -> $struct_name {
                $struct_name::from_flags(f as u32)
            }

            #[doc = std::concat!(" Creates a new [`", std::stringify!($struct_name), "`] from `f`")]
            pub const fn from_flags<F: [const] Into<$crate::VkFlags>>(f: F) -> $struct_name {
                $struct_name(f.into())
            }

            /// Do these flags contain `flags`?
            pub const fn contains<F: [const] Into<$struct_name>>(&self, flags: F) -> bool {
                self.0.contains(flags.into())
            }

            /// Set the flags `f`
            pub const fn set<F: [const] Into<$struct_name>>(&mut self, f: F) {
                self.0.set(f.into().0);
            }
        }

        impl const Default for $struct_name {
            fn default() -> Self {
                $struct_name::new()
            }
        }

        impl const From<u32> for $struct_name {
            fn from(flags: u32) -> Self {
                $struct_name::from_flags(flags)
            }
        }

        impl const From<$crate::VkFlags> for $struct_name {
            fn from(flags: $crate::VkFlags) -> Self {
                $struct_name::from_flags(flags)
            }
        }

        impl const From<$enum_name> for $struct_name {
            fn from(flag: $enum_name) -> Self {
                $struct_name::from_flag(flag)
            }
        }

        impl const Into<u32> for $struct_name {
            fn into(self) -> u32 {
                self.0.into()
            }
        }

        impl const Into<$crate::VkFlags> for $struct_name {
            fn into(self) -> $crate::VkFlags {
                self.0
            }
        }

        impl const Clone for $struct_name {
            fn clone(&self) -> Self {
                $struct_name::from_flags(self.0)
            }
        }

        impl Copy for $struct_name {}

        impl const PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl const PartialEq<$crate::VkFlags> for $struct_name {
            fn eq(&self, other: &$crate::VkFlags) -> bool {
                self.0.eq(other)
            }
        }

        impl const PartialEq<u32> for $struct_name {
            fn eq(&self, other: &u32) -> bool {
                self.0.eq(other)
            }
        }

        impl Eq for $struct_name {}

        impl<F: [const] Into<$struct_name>> const std::ops::BitOr<F> for $struct_name {
            type Output = $struct_name;

            fn bitor(self, rhs: F) -> Self::Output {
                $struct_name::from_flags(self.0 | rhs.into().0)
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}
pub(crate) use flags;
