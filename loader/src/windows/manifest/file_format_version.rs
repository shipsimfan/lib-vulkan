use json::data_format::{Converter, Deserialize};

pub(in crate::windows) enum FileFormatVersion {
    _1_0_0,
    _1_0_1,
}

struct FileFormatVersionConverter;

impl<'de> Deserialize<'de> for FileFormatVersion {
    fn deserialize<D: json::data_format::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        deserializer.deserialize_str(FileFormatVersionConverter)
    }
}

impl<'de> Converter<'de> for FileFormatVersionConverter {
    type Value = FileFormatVersion;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a string with a version")
    }

    fn convert_str<E: json::data_format::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "1.0.0" => Ok(FileFormatVersion::_1_0_0),
            "1.0.1" => Ok(FileFormatVersion::_1_0_1),
            _ => Err(E::invalid_value(value, "\"1.0.0\" or \"1.0.1\"")),
        }
    }
}
