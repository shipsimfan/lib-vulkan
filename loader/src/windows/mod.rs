mod driver;
mod error;
mod paths;

pub use error::LoadError;

pub(crate) use driver::Driver;

/// Loads all of the Vulkan drivers on a Windows system
pub(crate) fn load_drivers() -> Result<Vec<Driver>, LoadError> {
    let paths = paths::get_driver_manifest_paths()?;

    println!("Vulkan driver manifests:");
    for path in paths {
        println!(" - {}", path.display());
    }

    Ok(Vec::new())
}
