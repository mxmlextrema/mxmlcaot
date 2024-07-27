use crate::ns::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConversionKind {
    /// Implicit conversion.
    FromAny,

    /// Implicit conversion.
    ToAny,

    /// Implicit conversion between number types,
    /// where the base and target are as they are
    /// (not marked nullable or non-nullable).
    BetweenNumber,

    /// Implicit conversion to covariant type.
    /// 
    /// Involved types either both include null or both do not include null.
    ToCovariant,

    /// Explicit conversion from `Object`, `Object!`, or `Object?`, to interface.
    /// 
    /// Involved types either both include null or both do not include null.
    ObjectToItrfc,

    /// Implicit conversion to `Object`, `Object?` or `Object!`.
    /// 
    /// Involved types either both include null or both do not include null.
    ItrfcToObject,

    /// Implicit conversion.
    NonNullableToNullable,

    /// Implicit conversion to `T?` where the `T` is a type
    /// that includes `null` without having been
    /// marked explicitly nullable.
    AsIsToNullable,

    /// Implicit conversion from `T!` to `T` where `T` is a type
    /// that includes `null` without having been
    /// marked explicitly nullable.
    NonNullableToAsIs,

    /// Explicit conversion.
    /// 
    /// Involved types either both include null or both do not include null.
    ToContravariant,

    /// Explicit conversion.
    /// 
    /// Restrictions:
    /// 
    /// * Involved types either both include null or both do not include null.
    /// * Involved element types either both include null or both do not include null.
    ToCovariantVector,

    /// Explicit conversion.
    /// 
    /// Restrictions:
    /// 
    /// * Base type is either as-is, or non-nullable.
    /// * Target type is either as-is, or non-nullable.
    StringToEnum,

    /// Explicit conversion.
    /// 
    /// Restrictions:
    /// 
    /// * Base type is as-is.
    /// * Target type is either as-is, or non-nullable.
    NumberToEnum,

    /// Implicit conversion from `Function` to structural function type.
    /// 
    /// Involved types either both include null or both do not include null.
    FunctionToStructuralFunction,

    /// Explicit conversion, where base type is a type parameter type,
    /// possibly having been marked non-nullable.
    FromTypeParameter,

    /// Explicit conversion where the type arguments to
    /// a parameterized type are changed. This conversion
    /// does not apply to `Vector.<T>`.
    /// 
    /// Involved types either both include null or both do not include null.
    /// 
    /// Involved types are each either a parameterized type without applied types
    /// or an application of a parameterized type.
    ParameterizedTypeAlter,

    /// Explicit type conversion.
    ToString,

    /// Explicit type conversion.
    ToBoolean,

    /// Explicit type conversion.
    ToNumber,

    /// Explicit type conversion.
    ToFloat,

    /// Explicit type conversion.
    ToUint,

    /// Explicit type conversion.
    ToInt,
}

impl ConversionKind {
    pub fn is_implicit(&self) -> bool {
        [
            Self::FromAny,
            Self::ToAny,
            Self::BetweenNumber,
            Self::ToCovariant,
            Self::ItrfcToObject,
            Self::NonNullableToNullable,
            Self::AsIsToNullable,
            Self::NonNullableToAsIs,
            Self::FunctionToStructuralFunction,
        ].contains(self)
    }
}

pub struct ConversionMethods<'a>(pub &'a Database);

impl<'a> ConversionMethods<'a> {
    pub fn constant(&self, value: &Entity, target_type: &Entity) -> Result<Option<Entity>, DeferError> {
        let from_type = value.static_type(self.0);
        if &from_type == target_type {
            return Ok(Some(value.clone()));
        }

        if value.is::<InvalidationEntity>() || from_type.is::<InvalidationEntity>() || target_type.is::<InvalidationEntity>() {
            return Ok(Some(self.0.invalidation_entity()));
        }

        if !value.is::<Constant>() {
            return Ok(None);
        }

        // undefined to type containing undefined or null
        if value.is::<UndefinedConstant>() {
            if target_type.includes_undefined(self.0)? {
                return Ok(Some(self.0.factory().create_undefined_constant(target_type)));
            } else if target_type.includes_null(self.0)? {
                return Ok(Some(self.0.factory().create_null_constant(target_type)));
            }
        }

        // null to type containing undefined or null
        if value.is::<NullConstant>() && (target_type.includes_undefined(self.0)? || target_type.includes_null(self.0)?) {
            return Ok(Some(self.0.factory().create_null_constant(target_type)));
        }

        let object_type = self.0.object_type().defer()?;
        let target_esc_type = target_type.escape_of_nullable_or_non_nullable();

        // Number constant to *, Object or Object!
        if value.is::<NumberConstant>() && (target_type.is::<AnyType>() || target_esc_type == object_type) {
            return Ok(Some(self.0.factory().create_number_constant(value.number_value(), target_type)));
        }

        if value.is::<NumberConstant>() && self.0.numeric_types()?.contains(&target_esc_type) {
            let v = value.number_value().convert_type(target_type, self.0)?;
            return Ok(Some(self.0.factory().create_number_constant(v, target_type)));
        }

        // From T or T! constant to T?, or
        // from T or T? constant to T!
        if (target_type.is::<NullableType>() && target_type.base() == from_type.escape_of_nullable_or_non_nullable())
        || (target_type.is::<NonNullableType>() && target_type.base() == from_type.escape_of_nullable_or_non_nullable()) {
            let new_k = value.clone_constant(self.0);
            new_k.set_static_type(target_type.clone());
            return Ok(Some(new_k));
        }

        Ok(None)
    }

    pub fn implicit(&self, value: &Entity, target_type: &Entity, optional: bool) -> Result<Option<Entity>, DeferError> {
        let from_type = value.static_type(self.0);
        if &from_type == target_type {
            return Ok(Some(value.clone()));
        }

        let kc = self.constant(value, target_type)?;
        if kc.is_some() {
            return Ok(kc);
        }

        // From *
        if from_type.is::<AnyType>() {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::FromAny, optional, target_type)?));
        }

        // To *
        if target_type.is::<AnyType>() {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToAny, optional, target_type)?));
        }

        // Between number types
        if self.0.numeric_types()?.contains(&from_type) && self.0.numeric_types()?.contains(&target_type) {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::BetweenNumber, optional, target_type)?));
        }

        let from_type_esc = from_type.escape_of_nullable_or_non_nullable();
        let target_type_esc = target_type.escape_of_nullable_or_non_nullable();

        if from_type_esc.is_subtype_of(&target_type_esc, self.0)? {
            let both_include_null = from_type.includes_null(self.0)? && target_type.includes_null(self.0)?;
            let both_dont_include_null = !from_type.includes_null(self.0)? && !target_type.includes_null(self.0)?;

            // ToCovariant
            if both_include_null || both_dont_include_null {
                return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToCovariant, optional, target_type)?));
            }
        }

        let object_type = self.0.object_type().defer()?;

        if target_type_esc == object_type && from_type_esc.is_interface_type_possibly_after_sub() {
            let both_include_null = from_type.includes_null(self.0)? && target_type.includes_null(self.0)?;
            let both_dont_include_null = !from_type.includes_null(self.0)? && !target_type.includes_null(self.0)?;

            // ItrfcToObject
            if both_include_null || both_dont_include_null {
                return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ItrfcToObject, optional, target_type)?));
            }
        }

        if target_type.is::<NullableType>() {
            // NonNullableToNullable
            if from_type.is::<NonNullableType>() && from_type.base() == target_type.base() {
                return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::NonNullableToNullable, optional, target_type)?));
            }

            // AsIsToNullable
            if from_type == target_type.base() {
                return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::AsIsToNullable, optional, target_type)?));
            }
        }

        /*
        // NullableToAsIs
        if from_type.is::<NullableType>() && target_type == &from_type.base() && target_type.includes_null(self.0)? {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::NullableToAsIs, optional, target_type)?));
        }
        */

        // NonNullableToAsIs
        if from_type.is::<NonNullableType>() && target_type == &from_type.base() {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::NonNullableToAsIs, optional, target_type)?));
        }

        let function_type = self.0.function_type().defer()?;

        if from_type_esc == function_type && target_type_esc.is::<FunctionType>() {
            let both_include_null = from_type.includes_null(self.0)? && target_type.includes_null(self.0)?;
            let both_dont_include_null = !from_type.includes_null(self.0)? && !target_type.includes_null(self.0)?;

            // FunctionToStructuralFunction
            if both_include_null || both_dont_include_null {
                return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::FunctionToStructuralFunction, optional, target_type)?));
            }
        }

        Ok(None)
    }

    pub fn explicit(&self, value: &Entity, target_type: &Entity, optional: bool) -> Result<Option<Entity>, DeferError> {
        let from_type = value.static_type(self.0);
        if &from_type == target_type {
            return Ok(Some(value.clone()));
        }

        let ic = self.implicit(value, target_type, optional)?;
        if ic.is_some() {
            return Ok(ic);
        }

        let from_type_esc = from_type.escape_of_nullable_or_non_nullable();
        let target_type_esc = target_type.escape_of_nullable_or_non_nullable();
        let object_type = self.0.object_type().defer()?;

        if from_type_esc == object_type && target_type_esc.is_interface_type_possibly_after_sub() {
            let both_include_null = from_type.includes_null(self.0)? && target_type.includes_null(self.0)?;
            let both_dont_include_null = !from_type.includes_null(self.0)? && !target_type.includes_null(self.0)?;

            // ObjectToItrfc
            if both_include_null || both_dont_include_null {
                return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ObjectToItrfc, optional, target_type)?));
            }
        }

        if from_type_esc.is_ascending_type_of(&target_type_esc, self.0)? {
            let both_include_null = from_type.includes_null(self.0)? && target_type.includes_null(self.0)?;
            let both_dont_include_null = !from_type.includes_null(self.0)? && !target_type.includes_null(self.0)?;

            // ToContravariant
            if both_include_null || both_dont_include_null {
                return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToContravariant, optional, target_type)?));
            }
        }

        if let Some(el_subtype) = from_type_esc.vector_element_type(self.0)? {
            if let Some(el_basetype) = target_type_esc.vector_element_type(self.0)? {
                let asc = el_basetype.escape_of_nullable_or_non_nullable().is_ascending_type_of(&el_subtype.escape_of_nullable_or_non_nullable(), self.0)?;

                let both_el_include_null = el_basetype.includes_null(self.0)? && el_subtype.includes_null(self.0)?;
                let both_el_dont_include_null = !el_basetype.includes_null(self.0)? && !el_subtype.includes_null(self.0)?;

                if asc && (both_el_include_null || both_el_dont_include_null) {
                    let both_include_null = from_type.includes_null(self.0)? && target_type.includes_null(self.0)?;
                    let both_dont_include_null = !from_type.includes_null(self.0)? && !target_type.includes_null(self.0)?;

                    // ToCovariantVector
                    if both_include_null || both_dont_include_null {
                        return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToCovariantVector, optional, target_type)?));
                    }
                }
            }
        }

        let string_type = self.0.string_type().defer()?;

        // StringToEnum
        if from_type.escape_of_non_nullable() == string_type && target_type.escape_of_non_nullable().is::<EnumType>() {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::StringToEnum, optional, target_type)?));
        }

        let number_type = self.0.number_type().defer()?;

        // NumberToEnum
        if from_type == number_type && target_type.escape_of_non_nullable().is::<EnumType>() {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::NumberToEnum, optional, target_type)?));
        }

        if target_type == &string_type {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToString, optional, target_type)?));
        }

        if target_type == &number_type {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToNumber, optional, target_type)?));
        }

        let float_type = self.0.float_type().defer()?;

        if target_type == &float_type {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToFloat, optional, target_type)?));
        }

        let uint_type = self.0.uint_type().defer()?;

        if target_type == &uint_type {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToUint, optional, target_type)?));
        }

        let int_type = self.0.int_type().defer()?;

        if target_type == &int_type {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToInt, optional, target_type)?));
        }

        let boolean_type = self.0.boolean_type().defer()?;

        if target_type == &boolean_type {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ToBoolean, optional, target_type)?));
        }

        // FromTypeParameter
        if from_type.escape_of_non_nullable().is::<TypeParameterType>() {
            return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::FromTypeParameter, optional, target_type)?));
        }

        if from_type_esc.is_parameterized_type_or_type_after_sub() && target_type_esc.is_parameterized_type_or_type_after_sub() {
            let from_origin = from_type_esc.origin_or_parameterized_type_identity().unwrap();
            let target_origin = target_type_esc.origin_or_parameterized_type_identity().unwrap();
            let vector_type = self.0.vector_type().defer()?;

            let both_include_null = from_type.includes_null(self.0)? && target_type.includes_null(self.0)?;
            let both_dont_include_null = !from_type.includes_null(self.0)? && !target_type.includes_null(self.0)?;

            // ParameterizedTypeAlter
            if from_origin == target_origin && from_origin != vector_type
            && (both_include_null || both_dont_include_null)
            {
                return Ok(Some(self.0.factory().create_conversion_value(value, ConversionKind::ParameterizedTypeAlter, optional, target_type)?));
            }
        }

        Ok(None)
    }
}