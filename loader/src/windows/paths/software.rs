use std::path::PathBuf;

/// Gets the paths of the driver manifests from
/// "HKEY_LOCAL_MACHINE\SOFTWARE\Khronos\Vulkan\Drivers" in the Windows registry
pub(super) fn get_driver_manifest_paths() -> Vec<PathBuf> {
    Vec::new()
}
