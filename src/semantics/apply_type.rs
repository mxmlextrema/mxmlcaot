use crate::ns::*;

/// Operation for applying types to a parameterized type across several entities.
pub struct ApplyType<'a>(pub &'a Database);

impl<'a> ApplyType<'a> {
    pub fn exec(&mut self, thing: &Entity, type_params: &SharedArray<Entity>, substitute_types: &SharedArray<Entity>) -> Entity {
        if thing.is::<UnresolvedEntity>() || thing.is::<InvalidationEntity>() {
            return thing.clone();
        } else if thing.is::<Type>() {
            if thing.is::<FunctionType>() {
                let result_type = thing.result_type().apply_type(self.0, type_params, substitute_types);
                let mut params: Vec<Rc<SemanticFunctionTypeParameter>> = Vec::new();
                for param in thing.params().iter() {
                    params.push(Rc::new(param.apply_type(self.0, type_params, substitute_types)));
                }
                return self.0.factory().create_function_type(params, result_type);
            } else if thing.is::<NullableType>() {
                let base = &thing.base().apply_type(self.0, type_params, substitute_types);
                return self.0.factory().create_nullable_type(base);
            } else if thing.is::<NonNullableType>() {
                let base = &thing.base().apply_type(self.0, type_params, substitute_types);
                return self.0.factory().create_non_nullable_type(base);
            } else if thing.is::<TupleType>() {
                let el: Vec<Entity> = thing.element_types().iter().map(|t| t.apply_type(self.0, type_params, substitute_types)).collect();
                return self.0.factory().create_tuple_type(el);
            } else if thing.is::<TypeAfterSubstitution>() {
                let new_substitute_types: SharedArray<Entity> = thing.substitute_types().iter().map(|t| t.apply_type(self.0, type_params, substitute_types)).collect();
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