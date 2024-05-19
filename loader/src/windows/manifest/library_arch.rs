use json::data_format::{Converter, Deserialize, DeserializeError, Deserializer};

#[derive(Debug)]
pub(in crate::windows) enum LibraryArch {
    _32,
    _64,
}

struct LibraryArchConverter;

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
