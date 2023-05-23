macro_rules! get_instance_proc_addr_opt {
    ($loader: expr, $instance: expr, $name: literal) => {{
        $loader
            .get_instance_proc_addr(
                $instance,
                std::ffi::CStr::from_bytes_with_nul(concat!($name, "\0").as_bytes()).unwrap(),
            )
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
        ($fn)(
            $device,
            std::ffi::CStr::from_bytes_with_nul(concat!($name, "\0").as_bytes())
                .unwrap()
                .as_ptr(),
        )
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
    get_device_proc_addr, get_device_proc_addr_opt, get_instance_proc_addr,
    get_instance_proc_addr_opt,
};
