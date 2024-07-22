use crate::ns::*;

/// Type substitution.
///
/// In the present, the Compiler codebase is able to perform type substitution on a limited
/// set of things and panics for unsupported things:
///
/// * Types
///   * Classes and interfaces are not substituted by `TypeSubstitution`, but may be in a future version.
/// * `UnresolvedThingy` is returned as is
/// * Variable slots
/// * Virtual slots
/// * Method slots
/// 
/// In the future, type substitution may expand to other things if necessary.
pub struct TypeSubstitution<'a>(pub &'a Database);

impl<'a> TypeSubstitution<'a> {
    pub fn exec(&mut self, thing: &Thingy, type_params: &SharedArray<Thingy>, substitute_types: &SharedArray<Thingy>) -> Thingy {
        if thing.is::<UnresolvedThingy>() || thing.is::<InvalidationThingy>() {
            return thing.clone();
        } else if thing.is::<Type>() {
            if thing.is::<FunctionType>() {
                let result_type = thing.result_type().type_substitution(self.0, type_params, substitute_types);
                let mut params: Vec<Rc<SemanticFunctionTypeParameter>> = Vec::new();
                for param in thing.params().iter() {
                    params.push(Rc::new(param.type_substitution(self.0, type_params, substitute_types)));
                }
                return self.0.factory().create_function_type(params, result_type);
            } else if thing.is::<NullableType>() {
                let base = &thing.base().type_substitution(self.0, type_params, substitute_types);
                return self.0.factory().create_nullable_type(base);
            } else if thing.is::<NonNullableType>() {
                let base = &thing.base().type_substitution(self.0, type_params, substitute_types);
                return self.0.factory().create_non_nullable_type(base);
            } else if thing.is::<TupleType>() {
                let el: Vec<Thingy> = thing.element_types().iter().map(|t| t.type_substitution(self.0, type_params, substitute_types)).collect();
                return self.0.factory().create_tuple_type(el);
            } else if thing.is::<TypeAfterSubstitution>() {
                let new_substitute_types: SharedArray<Thingy> = thing.substitute_types().iter().map(|t| t.type_substitution(self.0, type_params, substitute_types)).collect();
                return self.0.factory().create_type_after_substitution(&thing.origin(), &new_substitute_types);
            } else if thing.is::<TypeParameterType>() {
                let i = type_params.index_of(&thing);
                if let Some(i) = i {
                    return substitute_types.get(i).unwrap();
                }
            }
            return thing.clone();
        } else if thing.is::<VariableSlot>() {
            self.0.factory().create_variable_slot_after_substitution(thing, type_params, substitute_types)
        } else if thing.is::<VirtualSlot>() {
            self.0.factory().create_virtual_slot_after_substitution(thing, type_params, substitute_types)
        } else if thing.is::<MethodSlot>() {
            self.0.factory().create_method_slot_after_substitution(thing, type_params, substitute_types)
        } else {
            panic!()
        }
    }
}