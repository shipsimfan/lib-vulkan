use manifest::Manifest;

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

            driver_paths.push(path.join(manifest.icd.library_path));
        }
    }

    let mut drivers = Vec::with_capacity(driver_paths.len());
    for driver_path in driver_paths {
        if let Ok(driver) = Driver::open(&driver_path) {
            drivers.push(driver);
        }
    }

    drivers
}
