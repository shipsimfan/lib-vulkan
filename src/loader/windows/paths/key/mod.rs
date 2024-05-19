use crate::loader::windows::str_to_utf16;
use std::ptr::null_mut;
use win32::{
    RegCloseKey, RegOpenKeyEx, ERROR_SUCCESS, HKEY, HKEY_LOCAL_MACHINE, KEY_ENUMERATE_SUB_KEYS,
    KEY_QUERY_VALUE, KEY_READ,
};

mod child_iter;
mod value_iter;

pub(super) use child_iter::RegistryKeyChildIter;
pub(super) use value_iter::RegistryKeyValueIter;

/// An opened Windows registry key
pub(super) struct RegistryKey(HKEY);

impl RegistryKey {
    /// Opens to open the `path` key in the Windows registry
    pub(super) fn open(parent: Option<&RegistryKey>, path: &str) -> Option<RegistryKey> {
        let path = str_to_utf16(path);

        Self::open_utf16(parent, &path)
    }

    pub(super) fn open_utf16(parent: Option<&RegistryKey>, path: &[u16]) -> Option<RegistryKey> {
        let parent = parent.map(|key| key.0).unwrap_or(HKEY_LOCAL_MACHINE);
        let mut key = null_mut();
        let status = unsafe {
            RegOpenKeyEx(
                parent,
                path.as_ptr(),
                0,
                KEY_READ | KEY_ENUMERATE_SUB_KEYS | KEY_QUERY_VALUE,
                &mut key,
            )
        };
        if status != ERROR_SUCCESS {
            return None;
        }

        Some(RegistryKey(key))
    }

    /// Gets an iterator over the child keys of this key
    pub(super) fn children(&self) -> RegistryKeyChildIter {
        RegistryKeyChildIter::new(self)
    }

    /// Gets an iterator over the values of this key
    pub(super) fn values(&self) -> RegistryKeyValueIter {
        RegistryKeyValueIter::new(self)
    }

    pub(super) fn inner(&self) -> HKEY {
        self.0
    }
}

impl Drop for RegistryKey {
    fn drop(&mut self) {
        unsafe { RegCloseKey(self.0) };
    }
}
