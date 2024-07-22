#![feature(decl_macro)]

pub mod errors;
pub mod semantics;
pub mod util;

/// Unified compiler module.
pub mod ns {
    pub use as3_parser::ns::*;
    pub use super::errors::*;
    pub use super::semantics::*;
    pub use super::util::*;
}