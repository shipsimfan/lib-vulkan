//! Vulkan raw bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod enumerations;
mod functions;
mod macros;
mod types;

pub use enumerations::*;
pub use functions::*;
pub use types::*;

pub use win32;
