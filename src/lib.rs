#![feature(extern_types)]
#![feature(concat_idents)]

mod bindings;
mod library;
mod loader;

pub use bindings::{VkInstance, VkResult};
pub use library::Library;
pub use loader::{Loader, NativeLoader};

pub(crate) use bindings::*;

pub type Result<T> = std::result::Result<T, VkResult>;
