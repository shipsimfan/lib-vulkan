use manifest::Manifest;

mod driver;
mod manifest;
mod paths;

pub(crate) use driver::Driver;

/// Loads all of the Vulkan drivers on a Windows system
pub(crate) fn load_drivers() -> Vec<Driver> {
    let paths = paths::get_driver_manifest_paths();

    assert!(paths.len() > 0);

    let mut manifests = Vec::new();
    for path in paths {
        if let Some(manifest) = Manifest::read(&path) {
            println!("{}:", path.display());
            println!("  {:?}", manifest);
            manifests.push((path, manifest));
        }
    }

    Vec::new()
}
