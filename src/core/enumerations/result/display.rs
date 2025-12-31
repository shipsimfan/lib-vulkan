use crate::VkResult;

impl std::fmt::Display for VkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_str() {
            Some(s) => f.write_str(s),
            None => write!(f, "unknown error ({})", *self as u32),
        }
    }
}
