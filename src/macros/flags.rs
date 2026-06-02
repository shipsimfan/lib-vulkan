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
        $crate::flags_no_bits!(
            $(#[$struct_meta])*
            pub struct $struct_name;
        );

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
            pub const fn new<F: [const] Into<$struct_name>>(flags: F) -> $struct_name {
                flags.into()
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

        impl const From<$enum_name> for $struct_name {
            fn from(flag: $enum_name) -> Self {
                $struct_name::new(flag as u32)
            }
        }

        impl const From<$crate::VkFlags> for $struct_name {
            fn from(flag: $crate::VkFlags) -> Self {
                $struct_name(flag)
            }
        }

        impl const From<u32> for $struct_name {
            fn from(flag: u32) -> Self {
                $struct_name(flag.into())
            }
        }

        impl const PartialEq<$enum_name> for $struct_name {
            fn eq(&self, other: &$enum_name) -> bool {
                self.0.eq(&(*other as u32))
            }
        }

        impl const std::ops::BitOr for $enum_name {
            type Output = $struct_name;

            fn bitor(self, rhs: $enum_name) -> Self::Output {
                $struct_name::new(self as u32 | rhs as u32)
            }
        }

        impl const std::ops::BitOr<$struct_name> for $enum_name {
            type Output = $struct_name;

            fn bitor(self, rhs: $struct_name) -> Self::Output {
                $struct_name::new(self as u32 | rhs.0)
            }
        }

        impl const std::ops::BitOr for $struct_name {
            type Output = $struct_name;

            fn bitor(self, rhs: $struct_name) -> Self::Output {
                $struct_name::new(self.0 | rhs.0)
            }
        }

        impl const std::ops::BitOr<$enum_name> for $struct_name {
            type Output = $struct_name;

            fn bitor(self, rhs: $enum_name) -> Self::Output {
                $struct_name::new(self.0 | rhs as u32)
            }
        }

        impl const std::ops::BitOrAssign for $struct_name {
            fn bitor_assign(&mut self, rhs: $struct_name) {
                *self = *self | rhs;
            }
        }

        impl const std::ops::BitOrAssign<$enum_name> for $struct_name {
            fn bitor_assign(&mut self, rhs: $enum_name) {
                *self = *self | rhs;
            }
        }
    };
}

/// Define a new set of flags with no bits defined
macro_rules! flags_no_bits {
    (
        $(#[$struct_meta: meta])*
        pub struct $struct_name: ident;
    ) => {
        $(#[$struct_meta])*
        #[repr(C)]
        #[derive(Debug)]
        pub struct $struct_name(pub $crate::VkFlags);

        impl $struct_name {
            #[doc = std::concat!(" Creates a new [`", std::stringify!($struct_name), "`] with no flags set")]
            pub const fn empty() -> $struct_name {
                $struct_name($crate::VkFlags::empty())
            }
        }

        impl const Default for $struct_name {
            fn default() -> Self {
                $struct_name::empty()
            }
        }

        impl const Into<$crate::VkFlags> for $struct_name {
            fn into(self) -> $crate::VkFlags {
                self.0
            }
        }

        impl const Into<u32> for $struct_name {
            fn into(self) -> u32 {
                self.0.into()
            }
        }

        impl const Clone for $struct_name {
            fn clone(&self) -> Self {
                $struct_name(self.0)
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

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

pub(crate) use {flags, flags_no_bits};
