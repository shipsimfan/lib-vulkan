use super::LibraryArch;
use json::data_format::{Converter, Deserialize, DeserializeError, Deserializer, MapDeserializer};

/// The description of the driver library
#[derive(Debug)]
pub(in crate::loader::windows) struct ICD {
    /// The path to the library itself
    pub library_path: String,

    /// The archictecture the library was built for
    pub library_arch: Option<LibraryArch>,

    /// The Vulkan version the library supports
    #[allow(unused)]
    pub api_version: String,

    /// Is the library a portability driver?
    #[allow(unused)]
    pub is_portability_driver: Option<bool>,
}

struct ICDConverter;

impl<'de> Deserialize<'de> for ICD {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ICDConverter)
    }
}

impl<'de> Converter<'de> for ICDConverter {
    type Value = ICD;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "an ICD object")
    }

    fn convert_map<M: MapDeserializer<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
        let mut library_path = None;
        let mut library_arch = None;
        let mut api_version = None;
        let mut is_portability_driver = None;

        while let Some(key) = map.next_key::<std::borrow::Cow<'de, str>>()? {
            match key.as_ref() {
                "library_path" => match library_path {
                    Some(_) => return Err(M::Error::duplicate_field("library_path")),
                    None => library_path = Some(map.next_value()?),
                },
                "library_arch" => match library_arch {
                    Some(_) => return Err(M::Error::duplicate_field("library_arch")),
                    None => library_arch = Some(map.next_value()?),
                },
                "api_version" => match api_version {
                    Some(_) => return Err(M::Error::duplicate_field("api_version")),
                    None => api_version = Some(map.next_value()?),
                },
                "is_portability_driver" => match is_portability_driver {
                    Some(_) => return Err(M::Error::duplicate_field("is_portability_driver")),
                    None => is_portability_driver = Some(map.next_value()?),
                },
                _ => {
                    return Err(M::Error::unknown_field(
                        key,
                        &[
                            "library_path",
                            "library_arch",
                            "api_version",
                            "is_portability_driver",
                        ],
                    ))
                }
            }
        }

        let library_path = library_path.ok_or(M::Error::missing_field("library_path"))?;
        let api_version = api_version.ok_or(M::Error::missing_field("api_version"))?;

        Ok(ICD {
            library_path,
            library_arch,
            api_version,
            is_portability_driver,
        })
    }
}
