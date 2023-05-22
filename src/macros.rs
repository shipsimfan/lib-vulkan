macro_rules! assert_null_terminated {
    ($string: expr) => {
        #[cfg(debug_assertions)]
        {
            let string = $string;
            debug_assert_eq!(
                string.as_bytes()[string.len() - 1],
                0,
                "Passed string \"{}\" is not null terminated",
                string
            );
        }
    };
}

macro_rules! get_instance_proc_addr_opt {
    ($loader: expr, $instance: expr, $name: literal) => {{
        $loader
            .get_instance_proc_addr($instance, concat!($name, "\0"))
            .map(|function| unsafe { std::mem::transmute(function) })
    }};
}

macro_rules! get_instance_proc_addr {
    ($loader: expr, $instance: expr, $name: literal) => {
        $crate::get_instance_proc_addr_opt!($loader, $instance, $name)
            .ok_or($crate::VkResult::ErrorIncompatibleDriver)
    };
}

macro_rules! get_device_proc_addr_opt {
    ($fn: ident, $device: expr, $name: literal) => {
        ($fn)($device, unsafe {
            std::ptr::NonNull::new_unchecked(concat!($name, "\0").as_ptr() as _)
        })
        .map(|function| unsafe { std::mem::transmute(function) })
    };
}

macro_rules! get_device_proc_addr {
    ($fn: ident, $device: expr, $name: literal) => {
        $crate::get_device_proc_addr_opt!($fn, $device, $name)
            .ok_or($crate::VkResult::ErrorIncompatibleDriver)
    };
}

pub(crate) use {
    assert_null_terminated, get_device_proc_addr, get_device_proc_addr_opt, get_instance_proc_addr,
    get_instance_proc_addr_opt,
};
