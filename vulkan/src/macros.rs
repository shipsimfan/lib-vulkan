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

macro_rules! assert_null_terminated_list {
    ($list: expr) => {
        #[cfg(debug_assertions)]
        {
            for item in $list {
                $crate::assert_null_terminated!(item)
            }
        }
    };
}

pub(crate) use {assert_null_terminated, assert_null_terminated_list};
