use manifest::Manifest;
use std::path::Path;

mod driver;
mod manifest;
mod paths;

pub(super) use driver::Driver;

/// Loads all of the Vulkan drivers on a Windows system
pub(crate) fn load_drivers() -> Vec<Driver> {
    let paths = paths::get_driver_manifest_paths();

    assert!(paths.len() > 0);

    let mut driver_paths = Vec::with_capacity(paths.len());
    for path in paths {
        if let Some(manifest) = Manifest::read(&path) {
            if let Some(arch) = manifest.icd.library_arch {
                if !arch.is_valid() {
                    continue;
                }
            }

            driver_paths.push((
                path.parent()
                    .unwrap_or(&Path::new(""))
                    .join(manifest.icd.library_path),
                manifest.icd.api_version,
            ));
        }
    }

    let mut drivers = Vec::with_capacity(driver_paths.len());
    for (driver_path, api_version) in driver_paths {
        if let Some(driver) = Driver::open(&driver_path, api_version) {
            drivers.push(driver);
        }
    }

    drivers
}

/// Converts `s` to a UTF-16 encoded array with a trailing null
fn str_to_utf16(s: &str) -> Vec<u16> {
    let mut result: Vec<_> = s.encode_utf16().collect();
    result.push(0);
    result
}
