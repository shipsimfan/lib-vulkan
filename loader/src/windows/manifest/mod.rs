use file_format_version::FileFormatVersion;
use icd::ICD;
use json::data_format::{Converter, Deserialize, Error};
use library_arch::LibraryArch;
use std::path::Path;

mod file_format_version;
mod icd;
mod library_arch;

/// A Vulkan driver manifest
pub(super) struct Manifest {
    /// The version of the manifest file
    pub file_format_version: FileFormatVersion,

    /// The description of the driver
    pub icd: ICD,
}

impl Manifest {
    pub(super) fn read(path: &Path) -> Option<Self> {
        let mut contents = std::fs::read_to_string(path).unwrap()?;

        json::from_string(contents).unwrap()
    }
}

struct ManifestConverter;

impl<'de> Deserialize<'de> for Manifest {
    fn deserialize<D: json::data_format::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ManifestConverter)
    }
}

impl<'de> Converter<'de> for ManifestConverter {
    type Value = Manifest<'de>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a manifest object")
    }

    fn convert_map<M: json::data_format::MapDeserializer<'de>>(
        self,
        mut map: M,
    ) -> Result<Self::Value, M::Error> {
        let mut file_format_version = None;
        let mut icd = None;

        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "file_format_version" => match file_format_version {
                    Some(_) => return Err(M::Error::duplicate_field("file_format_version")),
                    None => file_format_version = Some(map.next_value()?),
                },
                "ICD" => match icd {
                    Some(_) => return Err(M::Error::duplicate_field("ICD")),
                    None => icd = Some(map.next_value()?),
                },
                _ => {}
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
