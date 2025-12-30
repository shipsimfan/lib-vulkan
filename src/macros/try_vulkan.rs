// rustdoc imports
#[allow(unused_imports)]
use crate::VkResult;

/// Attempts `expr` as a Vulkan function converting the [`VkResult`] to `Result<VkResult, VkResult>`
#[macro_export]
macro_rules! try_vulkan {
    ($expr: expr) => {{
        let result = $expr;
        if result as usize >= 0 {
            Ok(result)
        } else {
            Err(result)
        }
    }};
}
