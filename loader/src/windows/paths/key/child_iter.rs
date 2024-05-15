use super::RegistryKey;
use std::ptr::null_mut;
use win32::{RegEnumKeyEx, DWORD, ERROR_SUCCESS};

pub(in crate::windows::paths) struct RegistryKeyChildIter<'a> {
    key: &'a RegistryKey,
    index: u32,
}

pub(super) const NAME_BUFFER_SIZE: usize = 256;

impl<'a> RegistryKeyChildIter<'a> {
    pub(super) fn new(key: &'a RegistryKey) -> Self {
        RegistryKeyChildIter { key, index: 0 }
    }
}

impl<'a> Iterator for RegistryKeyChildIter<'a> {
    type Item = Vec<u16>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut name = Vec::with_capacity(NAME_BUFFER_SIZE);
        let mut cch_name = NAME_BUFFER_SIZE as DWORD;

        let status = unsafe {
            RegEnumKeyEx(
                self.key.0,
                self.index,
                name.as_mut_ptr(),
                &mut cch_name,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        if status != ERROR_SUCCESS {
            return None;
        }

        self.index += 1;

        unsafe { name.set_len(cch_name as usize) };

        Some(name)
    }
}
