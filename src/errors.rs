use std::fmt::Debug;
use crate::ns::*;

/// Error used to indicate that verification must be deferred.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DeferError(pub Option<u32>);

/// Error used to indicate an ambiguous reference to a local name.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AmbiguousReferenceError(pub String);

#[derive(Clone)]
pub enum PropertyLookupError {
    Defer,
    AmbiguousReference(String),
    VoidBase,
    NullableObject {
        nullable_type: Entity,
    },
}

impl Debug for PropertyLookupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PropertyLookupError()")
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TypeExpectError();

#[derive(Clone)]
pub enum MethodOverrideError {
    Defer,
    MustOverrideAMethod,
    IncompatibleOverride {
        expected_signature: Entity,
        actual_signature: Entity,
    },
    OverridingFinalMethod,
}

impl Debug for MethodOverrideError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MethodOverrideError()")
    }
}