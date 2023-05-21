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
    ($loader: expr, $instance: expr, $name: literal) => {
        $loader
            .get_instance_proc_addr($instance, concat!($name, "\0"))
            .map(|function| unsafe { std::mem::transmute(function) })
    };
}

macro_rules! get_instance_proc_addr {
    ($loader: expr, $instance: expr, $name: literal) => {
        $crate::get_instance_proc_addr_opt!($loader, $instance, $name)
            .ok_or($crate::VkResult::ErrorIncompatibleDriver)
    };
}

pub(crate) use {assert_null_terminated, get_instance_proc_addr, get_instance_proc_addr_opt};
