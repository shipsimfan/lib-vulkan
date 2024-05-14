use crate::LoadError;
use std::path::PathBuf;

mod software;
mod system;

/// Gets the paths of the driver manifests on the system
pub(super) fn get_driver_manifest_paths() -> Result<Vec<PathBuf>, LoadError> {
    let mut paths: Vec<PathBuf> = Vec::new();
    paths.extend(system::get_driver_manifest_paths()?);
    paths.extend(software::get_driver_manifest_paths()?);
    Ok(paths)
}
