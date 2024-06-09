//! Due to the nature of the Vulkan interface, there is very little error information available to
//! the developer and application. By using the [`ext_debug_utils`] extension, developers can
//! obtain more information. When combined with validation layers, even more detailed feedback on
//! the application’s use of Vulkan will be provided.
//!
//! This extension provides the following capabilities:
//!  - The ability to create a debug messenger which will pass along debug messages to an
//!    application supplied callback.
//!  - The ability to identify specific Vulkan objects using a name or tag to improve tracking.
//!  - The ability to identify specific sections within a [`VkQueue`] or [`VkCommandBuffer`] using
//!    labels to aid organization and offline analysis in external tools.
//!
//! The main difference between this extension and [`ext_debug_report`] and [`ext_debug_marker`] is
//! that those extensions use [`VkDebugReportObjectTypeEXT`] to identify objects. This extension
//! uses the core [`VkObjectType`] in place of [`VkDebugReportObjectTypeEXT`]. The primary reason
//! for this move is that no future object type handle enumeration values will be added to
//! [`VkDebugReportObjectTypeEXT`] since the creation of [`VkObjectType`].
//!
//! In addition, this extension combines the functionality of both [`ext_debug_report`] and
//! [`ext_debug_marker`] by allowing object name and debug markers (now called labels) to be
//! returned to the application’s callback function. This should assist in clarifying the details
//! of a debug message including: what objects are involved and potentially which location within a
//! [`VkQueue`] or [`VkCommandBuffer`] the message occurred.

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

mod constants;
mod enumerations;
mod structures;
mod types;

pub use constants::*;
pub use enumerations::*;
pub use structures::*;
pub use types::*;
