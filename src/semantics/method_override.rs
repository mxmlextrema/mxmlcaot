use crate::ns::*;

/// Operations for listing "not" overriden abstract methods and for
/// overriding a method.
pub struct MethodOverride<'a>(pub &'a Database);

impl<'a> MethodOverride<'a> {
    /// Returns a listing of abstract methods that were not overriden.
    /// The resulting list may include method slots which are getters or setters
    /// from a virtual slot.
    pub fn abstract_methods_not_overriden(&mut self, class: &Entity, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
        let base_class = class.extends_class(self.0);
        if base_class.is_none() {
            return Ok(vec![]);
        }
        let base_class = base_class.unwrap();
        if base_class.is::<UnresolvedEntity>() {
            return Err(DeferError(None));
        }
        if &base_class == class {
            return Ok(vec![]);
        }
        let mut r: Vec<Entity> = vec![];
        for (name, prop) in base_class.prototype(self.0).borrow().iter() {
            // Regular method
            if prop.is::<MethodSlot>() {
                if prop.is_abstract() {
                    let prop2 = if name.namespace().is::<SystemNamespace>() {
                        class.prototype(self.0).get_in_system_ns_kind_in_ns_set(ns_set, name.namespace().system_ns_kind().unwrap(), &name.local_name()).ok().unwrap_or(None)
                    } else {
                        class.prototype(self.0).get(name)
                    };
                    if prop2.is_none() || !prop2.unwrap().is::<MethodSlot>() {
                        r.push(prop.clone());
                    }
                }
            }
            // Accessor
            else if prop.is::<VirtualSlot>() {
                if let Some(getter) = prop.getter(self.0) {
                    if prop.not_overriden_abstract_getter(&getter, class, self.0) {
                        r.push(getter.clone());
                    }
                }
                if let Some(setter) = prop.setter(self.0) {
                    if prop.not_overriden_abstract_setter(&setter, class, self.0) {
                        r.push(setter.clone());
                    }
                }
            }
        }
        Ok(r)
    }

    pub fn override_method(&mut self, method: &Entity, ns_set: &SharedArray<Entity>) -> Result<(), MethodOverrideError> {
        let name = method.name();
        let class = method.parent().unwrap();
        assert!(class.is::<ClassType>() || class.is::<EnumType>());

        let base_type = class.extends_class(self.0);
        if base_type.is_none() {
            return Err(MethodOverrideError::MustOverrideAMethod);
        }
        let base_type = base_type.unwrap();
        if base_type == class {
            return Ok(());
        }
        let base_method = self.lookup_method(&name, &base_type, ns_set)?;
        if base_method.is_none() {
            return Err(MethodOverrideError::MustOverrideAMethod);
        }
        let mut base_method = base_method.unwrap();
        
        let virtual_property = method.of_virtual_slot(self.0);
        if let Some(virtual_property) = virtual_property {
            let is_getter = Some(method.clone()) == virtual_property.getter(self.0);
            if is_getter {
                // Overriding a getter
                if !(base_method.is::<VirtualSlot>() && base_method.getter(self.0).is_some()) {
                    return Err(MethodOverrideError::MustOverrideAMethod);
                }
                base_method = base_method.getter(self.0).unwrap();
            } else {
                // Overriding a setter
                if !(base_method.is::<VirtualSlot>() && base_method.setter(self.0).is_some()) {
                    return Err(MethodOverrideError::MustOverrideAMethod);
                }
                base_method = base_method.setter(self.0).unwrap();
            }
        // Overriding a regular method
        } else if !base_method.is::<MethodSlot>() {
            return Err(MethodOverrideError::MustOverrideAMethod);
        }

        // Retrieve base type method's signature. Defer.
        let base_signature = base_method.signature(self.0);
        base_signature.defer().map_err(|_| MethodOverrideError::Defer)?;

        // Retrieve subtype method's signature. Defer.
        let subtype_signature = method.signature(self.0);
        subtype_signature.defer().map_err(|_| MethodOverrideError::Defer)?;

        if !self.compatible_override(&base_signature, &subtype_signature) {
            return Err(MethodOverrideError::IncompatibleOverride {
                expected_signature: base_signature,
                actual_signature: subtype_signature,
            });
        }

        if base_method.is_final() {
            return Err(MethodOverrideError::OverridingFinalMethod);
        }

        base_method.overriden_by(self.0).push(method.clone());
        method.set_overrides_method(Some(base_method));
        Ok(())
    }

    fn lookup_method(&mut self, name: &QName, base_type: &Entity, ns_set: &SharedArray<Entity>) -> Result<Option<Entity>, MethodOverrideError> {
        for class in base_type.descending_class_hierarchy(self.0).collect::<Vec<_>>() {
            // Defer
            class.defer().map_err(|_| MethodOverrideError::Defer)?;

            let prop = if name.namespace().is::<SystemNamespace>() {
                class.prototype(self.0).get_in_system_ns_kind_in_ns_set(ns_set, name.namespace().system_ns_kind().unwrap(), &name.local_name()).ok().unwrap_or(None)
            } else {
                class.prototype(self.0).get(name)
            };

            if let Some(prop) = prop {
                // Defer
                prop.property_static_type(self.0).defer().map_err(|_| MethodOverrideError::Defer)?;

                if prop.is::<VirtualSlot>() {
                    if let Some(getter) = prop.getter(self.0) {
                        // Defer
                        getter.signature(self.0).defer().map_err(|_| MethodOverrideError::Defer)?;
                    }

                    if let Some(setter) = prop.setter(self.0) {
                        // Defer
                        setter.signature(self.0).defer().map_err(|_| MethodOverrideError::Defer)?;
                    }
                }

                return Ok(Some(prop));
            }
        }
        Ok(None)
    }

    fn compatible_override(&mut self, base_signature: &Entity, subtype_signature: &Entity) -> bool {
        base_signature == subtype_signature
    }
}