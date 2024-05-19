use file_format_version::FileFormatVersion;
use icd::ICD;
use json::data_format::{Converter, Deserialize, DeserializeError, Deserializer, MapDeserializer};
use library_arch::LibraryArch;
use std::path::Path;

mod file_format_version;
mod icd;
mod library_arch;

/// A Vulkan driver manifest
#[derive(Debug)]
pub(super) struct Manifest {
    /// The version of the manifest file
    #[allow(unused)]
    pub file_format_version: FileFormatVersion,

    /// The description of the driver
    pub icd: ICD,
}

impl Manifest {
    pub(super) fn read(path: &Path) -> Option<Self> {
        let contents = std::fs::read_to_string(path).unwrap();

        Some(json::from_str(&contents).unwrap())
    }
}

struct ManifestConverter;

impl<'de> Deserialize<'de> for Manifest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ManifestConverter)
    }
}

impl<'de> Converter<'de> for ManifestConverter {
    type Value = Manifest;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a manifest object")
    }

    fn convert_map<M: MapDeserializer<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
        let mut file_format_version = None;
        let mut icd = None;

        while let Some(key) = map.next_key::<std::borrow::Cow<'de, str>>()? {
            match key.as_ref() {
                "file_format_version" => match file_format_version {
                    Some(_) => return Err(M::Error::duplicate_field("file_format_version")),
                    None => file_format_version = Some(map.next_value()?),
                },
                "ICD" => match icd {
                    Some(_) => return Err(M::Error::duplicate_field("ICD")),
                    None => icd = Some(map.next_value()?),
                },
                _ => {
                    map.next_value::<json::Value<'de>>()?;
                }
            }
        }

        let file_format_version =
            file_format_version.ok_or(M::Error::missing_field("file_format_version"))?;
        let icd = icd.ok_or(M::Error::missing_field("ICD"))?;

        Ok(Manifest {
            file_format_version,
            icd,
        })
    }
}
