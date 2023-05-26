macro_rules! opt_cstring_as_ptr {
    ($cstring: expr) => {
        $cstring
            .as_ref()
            .map(|cstring| cstring.as_ptr())
            .unwrap_or(std::ptr::null())
    };
}

macro_rules! string_vec_to_cstring_vec {
    ($string_vec: expr) => {{
        let cstring_vec: Vec<_> = $string_vec
            .into_iter()
            .map(|string| std::ffi::CString::new(string).unwrap())
            .collect();
        let ptrs_vec: Vec<_> = cstring_vec.iter().map(|cstring| cstring.as_ptr()).collect();
        (cstring_vec, ptrs_vec)
    }};
}

pub(crate) use {opt_cstring_as_ptr, string_vec_to_cstring_vec};
