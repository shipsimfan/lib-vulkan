/// Define a new set of 64-bit flags
macro_rules! flags64 {
    {
        $(#[$struct_meta: meta])*
        pub struct $struct_name: ident;

        $(#[$enum_meta: meta])*
        pub enum $enum_name: ident {$(
            $(#[$variant_meta: meta])*
            $variant: ident = $value: expr,
        )*}
    } => {
        $crate::flags64_no_bits!(
            $(#[$struct_meta])*
            pub struct $struct_name;
        );

        $(#[$enum_meta])*
        #[repr(u64)]
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $enum_name {$(
            $(#[$variant_meta])*
            $variant = $value,
        )*}

        impl $struct_name {
            #[doc = std::concat!(" Creates a new [`", std::stringify!($struct_name), "`] from `f`")]
            pub const fn from_flag(f: $enum_name) -> $struct_name {
                $struct_name::from_flags(f as u64)
            }
        }

        impl const From<$enum_name> for $struct_name {
            fn from(flag: $enum_name) -> Self {
                $struct_name::from_flag(flag)
            }
        }

        impl const PartialEq<$enum_name> for $struct_name {
            fn eq(&self, other: &$enum_name) -> bool {
                self.0.eq(&(*other as u64))
            }
        }

        impl const std::ops::BitOr for $enum_name {
            type Output = $struct_name;

            fn bitor(self, rhs: $enum_name) -> Self::Output {
                $struct_name::from_flags(self as u64 | rhs as u64)
            }
        }

        impl const std::ops::BitOr<$struct_name> for $enum_name {
            type Output = $struct_name;

            fn bitor(self, rhs: $struct_name) -> Self::Output {
                $struct_name::from_flags(self as u64 | rhs.0)
            }
        }
    };
}

/// Define a new set of 64-bit flags with no bits defined
macro_rules! flags64_no_bits {
    (
        $(#[$struct_meta: meta])*
        pub struct $struct_name: ident;
    ) => {
        $(#[$struct_meta])*
        #[repr(C)]
        #[derive(Debug)]
        pub struct $struct_name(pub $crate::VkFlags64);

        impl $struct_name {
            #[doc = std::concat!(" Creates a new [`", std::stringify!($struct_name), "`] with no flags set")]
            pub const fn new() -> $struct_name {
                $struct_name::from_flags(0)
            }

            #[doc = std::concat!(" Creates a new [`", std::stringify!($struct_name), "`] from `f`")]
            pub const fn from_flags<F: [const] Into<$crate::VkFlags64>>(f: F) -> $struct_name {
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

        impl const From<u64> for $struct_name {
            fn from(flags: u64) -> Self {
                $struct_name::from_flags(flags)
            }
        }

        impl const From<$crate::VkFlags64> for $struct_name {
            fn from(flags: $crate::VkFlags64) -> Self {
                $struct_name::from_flags(flags)
            }
        }


        impl const Into<u64> for $struct_name {
            fn into(self) -> u64 {
                self.0.into()
            }
        }

        impl const Into<$crate::VkFlags64> for $struct_name {
            fn into(self) -> $crate::VkFlags64 {
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

        impl const PartialEq<$crate::VkFlags64> for $struct_name {
            fn eq(&self, other: &$crate::VkFlags64) -> bool {
                self.0.eq(other)
            }
        }

        impl const PartialEq<u64> for $struct_name {
            fn eq(&self, other: &u64) -> bool {
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

        impl const std::ops::BitOr<$struct_name> for u64 {
            type Output = $struct_name;

            fn bitor(self, rhs: $struct_name) -> Self::Output {
                $struct_name::from_flags(self | rhs.0)
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

pub(crate) use {flags64, flags64_no_bits};
