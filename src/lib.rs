#![feature(decl_macro)]

pub mod errors;
pub mod semantics;
pub mod util;
pub mod verifier;

/// Unified compiler module.
pub mod ns {
    pub use mxmlextrema_as3parser::ns::*;
    pub use super::errors::*;
    pub use super::semantics::*;
    pub use super::util::*;
    pub use super::verifier::*;
}