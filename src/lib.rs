//! Vulkan raw bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(c_size_t)]

mod constants;
mod enumerations;
mod functions;
mod macros;
mod structures;
mod types;

pub use constants::*;
pub use enumerations::*;
pub use functions::*;
pub use structures::*;
pub use types::*;
