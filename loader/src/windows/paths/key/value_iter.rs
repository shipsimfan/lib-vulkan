use super::RegistryKey;
use std::ptr::null_mut;
use win32::{RegEnumValue, DWORD, ERROR_SUCCESS};

pub(in crate::windows::paths) struct RegistryKeyValueIter<'a> {
    key: &'a RegistryKey,
    index: u32,
}

pub(super) const NAME_BUFFER_SIZE: usize = 32_767;

impl<'a> RegistryKeyValueIter<'a> {
    pub(super) fn new(key: &'a RegistryKey) -> Self {
        RegistryKeyValueIter { key, index: 0 }
    }
}

impl<'a> Iterator for RegistryKeyValueIter<'a> {
    type Item = (Vec<u16>, DWORD);

    fn next(&mut self) -> Option<Self::Item> {
        let mut value_name = Vec::with_capacity(NAME_BUFFER_SIZE);
        let mut cch_value_name = NAME_BUFFER_SIZE as DWORD;
        let mut r#type = 0;

        let status = unsafe {
            RegEnumValue(
                self.key.0,
                self.index,
                value_name.as_mut_ptr(),
                &mut cch_value_name,
                null_mut(),
                &mut r#type,
                null_mut(),
                null_mut(),
            )
        };
        if status != ERROR_SUCCESS {
            return None;
        }

        self.index += 1;

        unsafe { value_name.set_len(cch_value_name as usize) };

        Some((value_name, r#type))
    }
}
