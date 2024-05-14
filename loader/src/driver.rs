use crate::os;

/// A loaded Vulkan driver
pub struct Driver(os::Driver);

impl Driver {
    /// Creates a new [`Driver`] from the given OS `driver`
    pub(crate) fn new(driver: os::Driver) -> Self {
        Driver(driver)
    }
}
