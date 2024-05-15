use key::RegistryKey;
use std::path::PathBuf;

mod key;
mod software;
mod system;

/// Gets the paths of the driver manifests on the system
pub(super) fn get_driver_manifest_paths() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();

    paths.extend(system::get_driver_manifest_paths());
    paths.extend(software::get_driver_manifest_paths());

    paths
}
