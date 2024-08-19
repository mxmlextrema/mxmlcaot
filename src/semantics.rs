//! Semantic interfaces, including operations, factory and several entities.
//!
//! # Entity type
//! 
//! The `Entity` type represents one of several ActionScript 3 symbols, such as
//! a class, a variable, a scope, or a value.
//! 
//! The `Entity` hierarchy is as follows:
//! 
//! - `Entity`
//!   - `Scope`
//!     - `Activation`
//!     - `FilterScope`
//!     - `FixtureScope`
//!       - `ClassScope`
//!       - `EnumScope`
//!       - `InterfaceScope`
//!       - `PackageScope`
//!     - `WithScope`
//!   - `Alias`
//!   - `Type`
//!     - `AnyType`
//!     - `ClassType`
//!     - `EnumType`
//!     - `FunctionType`
//!     - `InterfaceType`
//!     - `NonNullableType`
//!     - `NullableType`
//!     - `TupleType`
//!     - `TypeAfterSubstitution`
//!     - `TypeParameterType`
//!     - `VoidType`
//!   - `Value`
//!     - `ReferenceValue`
//!       - `ArrayElementReferenceValue`
//!       - `ByteArrayElementReferenceValue`
//!       - `DynamicReferenceValue`
//!       - `DynamicScopeReferenceValue`
//!       - `FixtureReferenceValue`
//!       - `InstanceReferenceValue`
//!       - `PackageReferenceValue`
//!       - `ScopeReferenceValue`
//!       - `StaticDynamicReferenceValue`
//!       - `StaticReferenceValue`
//!       - `TupleReferenceValue`
//!       - `VectorElementReferenceValue`
//!       - `XmlReferenceValue`
//!     - `Constant`
//!       - `BooleanConstant`
//!       - `NamespaceConstant`
//!       - `NullConstant`
//!       - `NumberConstant`
//!       - `StringConstant`
//!       - `TypeConstant`
//!       - `UndefinedConstant`
//!     - `ConversionValue`
//!     - `FilterValue`
//!     - `LambdaObject`
//!     - `MetaEnvProperty`
//!     - `MetaProperty`
//!     - `NonNullValue`
//!     - `PackagePropertyImport`
//!     - `PackageRecursiveImport`
//!     - `PackageWildcardImport`
//!     - `ThisObject`
//!   - `FieldResolution`
//!   - `AssignmentFieldDestructuringResolution`
//!   - `DeclarativeFieldDestructuringResolution`
//!   - `Namespace`
//!     - `ExplicitNamespace`
//!     - `SystemNamespace`
//!     - `UserNamespace`
//!   - `InvalidationEntity`
//!   - `UnresolvedEntity`
//!   - `MethodSlot`
//!     - `MethodSlotAfterSubstitution`
//!     - `OriginalMethodSlot`
//!   - `VariableSlot`
//!     - `OriginalVariableSlot`
//!     - `VariableSlotAfterSubstitution`
//!   - `VirtualSlot`
//!     - `OriginalVirtualSlot`
//!     - `VirtualSlotAfterSubstitution`
//!   - `Package`

mod interface_implement;
pub use interface_implement::*;

mod method_override;
pub use method_override::*;

mod names;
pub use names::*;

mod number;
pub use number::*;

mod property_lookup;
pub use property_lookup::*;

mod database;
pub use database::*;

#[allow(unused_variables)]
mod entity;
pub use entity::*;

mod factory;
pub use factory::*;

mod conversion;
pub use conversion::*;

mod apply_type;
pub use apply_type::*;

mod unused;
pub use unused::*;