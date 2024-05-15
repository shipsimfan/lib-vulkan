use super::RegistryKey;
use std::{path::PathBuf, ptr::null_mut, str::FromStr};
use win32::{RegGetValue, DWORD, ERROR_SUCCESS, REG_DWORD, RRF_RT_DWORD};

const ROOT: &str = "SOFTWARE\\Khronos\\Vulkan\\Drivers";

/// Gets the paths of the driver manifests from
/// "HKEY_LOCAL_MACHINE\SOFTWARE\Khronos\Vulkan\Drivers" in the Windows registry
pub(super) fn get_driver_manifest_paths() -> Vec<PathBuf> {
    let root_key = match RegistryKey::open(None, ROOT) {
        Some(root_key) => root_key,
        None => return Vec::new(),
    };

    let mut paths = Vec::new();
    for (value, r#type) in root_key.values() {
        if r#type != REG_DWORD {
            continue;
        }

        let mut dword_value: DWORD = 0;
        let mut dword_size = std::mem::size_of::<DWORD>() as DWORD;
        let status = unsafe {
            RegGetValue(
                root_key.inner(),
                null_mut(),
                value.as_ptr(),
                RRF_RT_DWORD,
                null_mut(),
                (&mut dword_value as *mut DWORD).cast(),
                &mut dword_size,
            )
        };
        if status != ERROR_SUCCESS || dword_value != 0 {
            continue;
        }

        paths.push(PathBuf::from_str(&String::from_utf16_lossy(&value)).unwrap());
    }

    paths
}
