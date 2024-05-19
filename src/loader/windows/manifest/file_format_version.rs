use json::data_format::{Converter, Deserialize, DeserializeError, Deserializer};

/// The version of the file format the manifest is using
#[derive(Debug)]
pub(in crate::loader::windows) enum FileFormatVersion {
    /// Version 1.0.0
    _1_0_0,

    /// Version 1.0.1
    _1_0_1,
}

struct FileFormatVersionConverter;

impl<'de> Deserialize<'de> for FileFormatVersion {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(FileFormatVersionConverter)
    }
}

impl<'de> Converter<'de> for FileFormatVersionConverter {
    type Value = FileFormatVersion;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a string with a version")
    }

    fn convert_string<E: DeserializeError<'de>>(self, value: String) -> Result<Self::Value, E> {
        self.convert_str(&value)
    }

    fn convert_str<E: DeserializeError<'de>>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "1.0.0" => Ok(FileFormatVersion::_1_0_0),
            "1.0.1" => Ok(FileFormatVersion::_1_0_1),
            _ => Err(E::invalid_value(value, "\"1.0.0\" or \"1.0.1\"")),
        }
    }
}
