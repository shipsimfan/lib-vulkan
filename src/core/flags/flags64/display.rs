use crate::VkFlags64;

impl std::fmt::Display for VkFlags64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
