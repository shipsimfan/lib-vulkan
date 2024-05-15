use key::RegistryKey;
use std::{collections::HashSet, path::PathBuf};

mod key;
mod software;
mod system;

/// Gets the paths of the driver manifests on the system
pub(super) fn get_driver_manifest_paths() -> HashSet<PathBuf> {
    let mut paths = HashSet::new();

    for path in system::get_driver_manifest_paths() {
        paths.insert(path);
    }

    for path in software::get_driver_manifest_paths() {
        paths.insert(path);
    }

    paths
}
