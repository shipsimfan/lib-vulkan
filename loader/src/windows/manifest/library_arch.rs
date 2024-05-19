use json::data_format::{Converter, Deserialize, DeserializeError, Deserializer};

/// The architecture the library was built for
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(in crate::windows) enum LibraryArch {
    /// 32-bits
    _32,

    /// 64-bits
    _64,
}

struct LibraryArchConverter;

/// The current architecture of the program
#[cfg(target_pointer_width = "32")]
const CURRENT_ARCH: LibraryArch = LibraryArch::_32;

/// The current architecture of the program
#[cfg(target_pointer_width = "64")]
const CURRENT_ARCH: LibraryArch = LibraryArch::_64;

impl LibraryArch {
    /// Checks if the library was built for the architecture this is running on
    pub(in crate::windows) fn is_valid(self) -> bool {
        self == CURRENT_ARCH
    }
}

impl<'de> Deserialize<'de> for LibraryArch {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(LibraryArchConverter)
    }
}

impl<'de> Converter<'de> for LibraryArchConverter {
    type Value = LibraryArch;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a string with the library architecture")
    }

    fn convert_str<E: DeserializeError<'de>>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "32" => Ok(LibraryArch::_32),
            "64" => Ok(LibraryArch::_64),
            _ => Err(E::invalid_value(value, "\"64\" or \"32\"")),
        }
    }
}
