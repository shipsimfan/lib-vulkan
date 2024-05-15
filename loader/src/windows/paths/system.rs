use super::RegistryKey;
use std::{
    path::PathBuf,
    ptr::{null, null_mut},
};
use win32::{RegGetValue, ERROR_SUCCESS, RRF_RT_REG_MULTI_SZ, RRF_RT_REG_SZ};

const ROOT: &str = "System\\CurrentControlSet\\Control\\Class";

const VALUE_NAME: &[u16] = &[
    0x56, 0x75, 0x6c, 0x6b, 0x61, 0x6e, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65,
    0x00,
]; // "VulkanDriverName" encoded in UTF-16

/// Gets the paths of the driver manifests from
/// "HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\Class" in the Windows registry
pub(super) fn get_driver_manifest_paths() -> Vec<PathBuf> {
    let root_key = match RegistryKey::open(None, ROOT) {
        Some(root_key) => root_key,
        None => return Vec::new(),
    };

    let mut paths = Vec::new();
    for child in root_key.children() {
        if let Some(child) = RegistryKey::open_utf16(Some(&root_key), &child) {
            paths.extend(check_adapter(child));
        };
    }

    paths
}

/// Checks the keys under a given `adapter_key` for Vulkan drivers
fn check_adapter(adapter_key: RegistryKey) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for child in adapter_key.children() {
        if !is_valid_vulkan_key(&child) {
            continue;
        }

        if let Some(child) = RegistryKey::open_utf16(Some(&adapter_key), &child) {
            paths.extend(check_device(child));
        }
    }

    paths
}

/// Is `key` a device that might contain a Vulkan driver?
fn is_valid_vulkan_key(key: &[u16]) -> bool {
    if key.len() != 4 {
        return false;
    }

    for c in key {
        if *c < b'0' as u16 || *c > b'9' as u16 {
            return false;
        }
    }

    true
}

/// Checks if `device_key` has a Vulkan driver
fn check_device(device_key: RegistryKey) -> Vec<PathBuf> {
    let raw_paths = get_device_paths(device_key);
    if raw_paths.len() == 0 {
        return Vec::new();
    }

    let mut paths: Vec<PathBuf> = Vec::new();
    let mut start = 0;
    for i in 0..raw_paths.len() {
        if raw_paths[i] != 0 {
            continue;
        }

        if i - start <= 1 {
            break;
        }

        paths.push(PathBuf::from(&String::from_utf16_lossy(&raw_paths)));
        start = i + 1;
    }

    paths
}

fn get_device_paths(device_key: RegistryKey) -> Vec<u16> {
    let mut cb_data = 0;
    let status = unsafe {
        RegGetValue(
            device_key.inner(),
            null(),
            VALUE_NAME.as_ptr(),
            RRF_RT_REG_SZ | RRF_RT_REG_MULTI_SZ,
            null_mut(),
            null_mut(),
            &mut cb_data,
        )
    };
    if status != ERROR_SUCCESS {
        return Vec::new();
    }

    let mut raw_paths: Vec<u16> = vec![0; cb_data as usize];
    let status = unsafe {
        RegGetValue(
            device_key.inner(),
            null(),
            VALUE_NAME.as_ptr(),
            RRF_RT_REG_SZ | RRF_RT_REG_MULTI_SZ,
            null_mut(),
            raw_paths.as_mut_ptr().cast(),
            &mut cb_data,
        )
    };
    if status != ERROR_SUCCESS {
        return Vec::new();
    }

    unsafe { raw_paths.set_len(cb_data as usize) };
    raw_paths
}
