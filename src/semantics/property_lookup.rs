use crate::ns::*;

/// Methods for looking up a property in an object, package or scope.
pub struct PropertyLookup<'a>(pub &'a Database);

#[derive(Clone)]
pub enum PropertyLookupKey {
    LocalName(String),
    Computed(Entity),
}

impl PropertyLookupKey {
    pub fn computed_or_local_name(&self, host: &Database) -> Result<Entity, DeferError> {
        match self {
            Self::LocalName(s) => {
                let string_type = host.string_type().defer()?;
                Ok(host.factory().create_string_constant(s.clone(), &string_type))
            },
            Self::Computed(s) => Ok(s.clone()),
        }
    }

    pub fn static_type(&self, host: &Database) -> Result<Entity, DeferError> {
        match self {
            Self::LocalName(_) => host.string_type().defer(),
            Self::Computed(s) => s.static_type(host).defer(),
        }
    }

    pub fn local_name(&self) -> Option<String> {
        match self {
            Self::LocalName(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn double_value(&self) -> Result<Option<f64>, DeferError> {
        Ok(match self {
            Self::Computed(d) => {
                if d.is::<NumberConstant>() {
                    Some(d.number_value().force_double())
                } else {
                    None
                }
            },
            _ => None,
        })
    }
}

fn defer(entity: &Entity) -> Result<Entity, PropertyLookupError> {
    if entity.is::<UnresolvedEntity>() {
        Err(PropertyLookupError::Defer)
    } else {
        Ok(entity.clone())
    }
}

fn map_defer_error<T>(result: Result<T, DeferError>) -> Result<T, PropertyLookupError> {
    result.map_err(|_| PropertyLookupError::Defer)
}

impl<'a> PropertyLookup<'a> {
    pub fn lookup_in_object(&self, base: &Entity, open_ns_set: &SharedArray<Entity>, qual: Option<Entity>, key: &PropertyLookupKey, calling: bool) -> Result<Option<Entity>, PropertyLookupError> {
        if base.is::<InvalidationEntity>() {
            return Ok(Some(base.clone()));
        }
        let local_name = key.local_name();
        let double_key = map_defer_error(key.double_value())?;

        // If base is a class
        if base.is_class_or_equivalent() {
            // Key must be a String constant
            let Some(local_name) = local_name else {
                return Ok(None);
            };

            // Qualifier must be a compile-time namespace, otherwise return static dynamic reference.
            if qual.as_ref().map(|q| q.is_namespace_or_ns_constant()).unwrap_or(true) {
                let k = map_defer_error(PropertyLookupKey::LocalName(local_name).computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_static_dynamic_reference_value(base, qual, &k)));
            }

            for class in base.descending_class_hierarchy(self.0).collect::<Vec<_>>() {
                // Defer if unresolved
                defer(&class)?;

                let r = self.get_qname_in_ns_set_or_any_public_ns(&class.properties(self.0), open_ns_set, qual.clone(), &local_name)?;
                if let Some(r) = r {
                    Unused(self.0).mark_used(&r);

                    let r = r.resolve_alias();

                    // Defer if unresolved
                    defer(&r.property_static_type(self.0))?;

                    return Ok(Some(map_defer_error(r.wrap_property_reference(self.0))?));
                }
            }

            return Ok(None);
        }

        // If base is an interface
        if base.is_interface_type_possibly_after_sub() {
            // Key must be a String constant
            let Some(key) = local_name else {
                return Ok(None);
            };

            // Qualifier must be a compile-time namespace, otherwise return static dynamic reference.
            if qual.as_ref().map(|q| q.is_namespace_or_ns_constant()).unwrap_or(true) {
                let k = map_defer_error(PropertyLookupKey::LocalName(key).computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_static_dynamic_reference_value(base, qual, &k)));
            }

            return Ok(None);
        }

        // For a value
        if base.is::<Value>() {
            let base_type = defer(&base.static_type(self.0))?;
            let base_esc_type = base_type.escape_of_non_nullable();

            if base_esc_type.is::<InvalidationEntity>() {
                return Ok(Some(base_esc_type.clone()));
            }

            // If not calling the property and base is a value whose type is one of
            // { XML, XML!, XMLList, XMLList! }, return a XML reference value.
            //
            // If not calling the property and base is a Dictionary, then return a dynamic reference value.
            if !calling {
                if [defer(&self.0.xml_type())?, defer(&self.0.xml_list_type())?].contains(&base_esc_type) {
                    let k = map_defer_error(key.computed_or_local_name(self.0))?;
                    return Ok(Some(self.0.factory().create_xml_reference_value(base, qual, &k)));
                }
                if base_type.escape_of_non_nullable() == map_defer_error(self.0.dictionary_type().defer())? {
                    let k = map_defer_error(key.computed_or_local_name(self.0))?;
                    return Ok(Some(self.0.factory().create_dynamic_reference_value(base, qual, &k)));
                }
            }

            let has_known_ns = qual.as_ref().map(|q| q.is_namespace_or_ns_constant()).unwrap_or(true);

            let Some(local_name) = local_name else {
                // Attempt to index Array
                if let Some(_) = map_defer_error(base_esc_type.array_element_type(self.0))? {
                    let iv: Option<Entity> = map_defer_error(ConversionMethods(self.0).implicit(&map_defer_error(key.computed_or_local_name(self.0))?, &defer(&self.0.number_type())?, false))?;
                    if let Some(iv) = iv {
                        return Ok(Some(map_defer_error(self.0.factory().create_array_element_reference_value(&base, &iv))?));
                    }
                }

                // Attempt to index Vector
                if let Some(_) = map_defer_error(base_esc_type.vector_element_type(self.0))? {
                    let iv: Option<Entity> = map_defer_error(ConversionMethods(self.0).implicit(&map_defer_error(key.computed_or_local_name(self.0))?, &defer(&self.0.number_type())?, false))?;
                    if let Some(iv) = iv {
                        return Ok(Some(map_defer_error(self.0.factory().create_vector_element_reference_value(&base, &iv))?));
                    }
                }

                // Attempt to index ByteArray
                if base_esc_type == map_defer_error(self.0.byte_array_type().defer())? {
                    let iv: Option<Entity> = map_defer_error(ConversionMethods(self.0).implicit(&map_defer_error(key.computed_or_local_name(self.0))?, &defer(&self.0.number_type())?, false))?;
                    if let Some(iv) = iv {
                        return Ok(Some(map_defer_error(self.0.factory().create_byte_array_element_reference_value(&base, &iv))?));
                    }
                }

                // Attempt to index String
                if base_esc_type == map_defer_error(self.0.string_type().defer())? {
                    let iv: Option<Entity> = map_defer_error(ConversionMethods(self.0).implicit(&map_defer_error(key.computed_or_local_name(self.0))?, &defer(&self.0.number_type())?, false))?;
                    if let Some(iv) = iv {
                        return Ok(Some(self.0.factory().create_dynamic_reference_value(&base, None, &iv)));
                    }
                }
                
                // Attempt to index a tuple
                if double_key.is_some() && base_esc_type.is::<TupleType>() {
                    let index: usize = unsafe { double_key.unwrap().to_int_unchecked() };
                    if index >= base_type.element_types().length() {
                        return Ok(None);
                    }
                    return Ok(Some(self.0.factory().create_tuple_reference_value(&base, index)));
                }

                let k = map_defer_error(key.computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_dynamic_reference_value(base, qual, &k)));
            };

            // If base data type is one of { *, Object, Object! }, or
            // if qualifier is not a compile-time control namespace,
            // return a dynamic reference value..
            let any_or_object = [self.0.any_type(), defer(&self.0.object_type())?].contains(&base_esc_type);
            if any_or_object
            || !has_known_ns
            {
                if qual.is_none() {
                    // "import.meta" base
                    if base.is::<MetaProperty>() {
                        if local_name == "env" {
                            return Ok(Some(self.0.meta_env_property()));
                        }
                        return Ok(None);
                    }
    
                    // "import.meta.env" base
                    if base.is::<MetaEnvProperty>() {
                        let ev_dict = self.0.env();
                        if let Some(ev) = ev_dict.get(&local_name) {
                            let string_type = defer(&self.0.string_type())?;
                            return Ok(Some(self.0.factory().create_string_constant(ev.clone(), &string_type)));
                        } else {
                            return Ok(None);
                        }
                    }
                }

                let k = map_defer_error(key.computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_dynamic_reference_value(base, qual, &k)));
            }

            if base_esc_type.is_class_or_equivalent() {
                for class in base_esc_type.descending_class_hierarchy(self.0).collect::<Vec<_>>() {
                    // Defer if unresolved
                    defer(&class)?;

                    let prop = self.get_qname_in_ns_set_or_any_public_ns(&class.prototype(self.0), open_ns_set, qual.clone(), &local_name)?;

                    if let Some(prop) = prop {
                        Unused(self.0).mark_used(&prop);

                        let prop = prop.resolve_alias();

                        // Throw if unresolved
                        defer(&prop.property_static_type(self.0))?;

                        if prop.is_namespace_or_ns_constant() {
                            return Ok(Some(map_defer_error(self.0.factory().create_namespace_constant(&prop))?));
                        }

                        return Ok(Some(map_defer_error(self.0.factory().create_instance_reference_value(&base, &prop))?));
                    }
                }
            } else if base_esc_type.is_interface_type_possibly_after_sub() {
                for itrfc in base_esc_type.all_ascending_types(self.0).iter().rev() {
                    // Defer if unresolved
                    defer(itrfc)?;

                    let prop = self.get_qname_in_ns_set_or_any_public_ns(&itrfc.prototype(self.0), open_ns_set, qual.clone(), &local_name)?;

                    if let Some(prop) = prop {
                        Unused(self.0).mark_used(&prop);

                        // Defer if unresolved
                        defer(&prop.property_static_type(self.0))?;

                        return Ok(Some(map_defer_error(self.0.factory().create_instance_reference_value(&base, &prop))?));
                    }
                }
            }

            // If base data type is a dynamic class or a Proxy then
            // return a dynamic reference value.
            let is_proxy = map_defer_error(base_type.is_equals_or_subtype_of(&map_defer_error(self.0.proxy_type().defer())?, self.0))?;
            if is_proxy || base_type.escape_of_non_nullable().is_dynamic() {
                let k = map_defer_error(key.computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_dynamic_reference_value(base, qual, &k)));
            }

            return Ok(None);
        }

        if base.is::<Package>() {
            // Key must be a local name
            let Some(local_name) = local_name else {
                return Ok(None);
            };

            // Qualifier must be a compile-time namespace.
            if qual.as_ref().map(|q| q.is_namespace_or_ns_constant()).unwrap_or(true) {
                return Ok(None);
            }

            let mut r: Option<Entity> = None;

            let prop = self.get_qname_in_ns_set_or_any_public_ns(&base.properties(self.0), open_ns_set, qual.clone(), &local_name)?;

            if let Some(prop) = prop {
                Unused(self.0).mark_used(&prop);

                let prop = prop.resolve_alias();

                defer(&prop.property_static_type(self.0))?;

                r = Some(map_defer_error(prop.wrap_property_reference(self.0))?);
            // Detect Vector from __AS3__.vec.Vector
            } else if base == &self.0.top_level_package && local_name == "Vector" && qual.as_ref().map(|q| q.is_public_ns()).unwrap_or(true) {
                r = Some(defer(&self.0.vector_type())?);
            }

            for concatp in base.package_concats().iter() {
                let r1 = self.lookup_in_object(&concatp, open_ns_set, qual.clone(), key, calling)?;
                if let Some(r1) = r1 {
                    if r.is_some() {
                        return Err(PropertyLookupError::AmbiguousReference(local_name));
                    }
                    r = Some(r1);
                }
            }

            return Ok(r);
        }

        Ok(None)
    }

    pub fn lookup_in_scope_chain(&self, scope: &Entity, qual: Option<Entity>, key: &PropertyLookupKey) -> Result<Option<Entity>, PropertyLookupError> {
        let open_ns_set = scope.concat_open_ns_set_of_scope_chain();

        // If the key is computed, always return dynamic
        if matches!(key, PropertyLookupKey::Computed(_)) {
            let k = map_defer_error(key.computed_or_local_name(self.0))?;
            return Ok(Some(self.0.factory().create_dynamic_scope_reference_value(scope, qual, &k)));
        }

        // If it's a "with" scope
        if scope.is::<WithScope>() {
            let obj = scope.object();
            let obj_static_type = defer(&obj.static_type(self.0))?;

            if [self.0.any_type(), self.0.xml_type(), self.0.xml_list_type()].contains(&obj_static_type.escape_of_non_nullable()) {
                let k = map_defer_error(key.computed_or_local_name(self.0))?;
                return Ok(Some(self.0.factory().create_dynamic_scope_reference_value(scope, qual, &k)));
            }

            let r = self.lookup_in_object(&obj, &open_ns_set, qual.clone(), key, false)?;
            if let Some(r) = r {
                return Ok(Some(r));
            }
        }

        // If it's a filter operator's scope
        if scope.is::<FilterScope>() {
            let k = map_defer_error(key.computed_or_local_name(self.0))?;
            return Ok(Some(self.0.factory().create_dynamic_scope_reference_value(scope, qual, &k)));
        }

        let local_name = key.local_name();
        let has_known_ns = qual.as_ref().map(|q| q.is_namespace_or_ns_constant()).unwrap_or(true);

        if let Some(qual) = qual.as_ref() {
            if qual.is::<PackageWildcardImport>() {
                let Some(local_name) = local_name else {
                    return Ok(None);
                };
                return self.lookup_in_object(&qual.package(), &open_ns_set, None, &PropertyLookupKey::LocalName(local_name.clone()), false);
            }

            if qual.is::<PackageRecursiveImport>() {
                let Some(local_name) = local_name else {
                    return Ok(None);
                };
                return self.lookup_in_package_recursive(&qual.package(), &open_ns_set, None, &PropertyLookupKey::LocalName(local_name.clone()));
            }
        }

        // Let r be the last assigned lookup success result.
        let mut r: Option<Entity> = None;

        if has_known_ns && local_name.is_some() {
            r = self.get_qname_in_ns_set_or_any_public_ns(&scope.properties(self.0), &open_ns_set, qual.clone(), local_name.as_ref().unwrap())?;
        }

        if let Some(r1) = r.as_ref() {
            Unused(self.0).mark_used(&r1);

            let r1 = r1.resolve_alias();

            defer(&r1.property_static_type(self.0))?;

            r = Some(map_defer_error(r1.wrap_property_reference(self.0))?);
        }

        if scope.is::<Activation>() && scope.this().is_some() && r.is_none() {
            let r1 = self.lookup_in_object(&scope.this().unwrap(), &open_ns_set, qual.clone(), key, false)?;
            if let Some(r1) = r1 {
                if !(r1.is::<DynamicReferenceValue>() || r1.is::<XmlReferenceValue>()) {
                    r = Some(r1);
                }
            }
        }

        // If scope is a class or enum scope and a local name key is specified
        if (scope.is::<ClassScope>() || scope.is::<EnumScope>()) && local_name.is_some() {
            let r1 = self.lookup_in_object(&scope.class(), &open_ns_set, qual.clone(), key, false)?;
            if r1.is_some() {
                if r.is_some() {
                    return Err(PropertyLookupError::AmbiguousReference(local_name.as_ref().unwrap().clone()));
                }
                r = r1;
            }
        }

        let mut amb: Option<Entity>;

        // For a package scope
        if scope.is::<PackageScope>() && has_known_ns && local_name.is_some() {
            amb = self.lookup_in_object(&scope.package(), &open_ns_set, qual.clone(), key, false)?;
            if amb.is_some() {
                if r.is_some() {
                    return Err(PropertyLookupError::AmbiguousReference(local_name.as_ref().unwrap().clone()));
                }
                r = amb;
            }
        }

        if let Some(local_name) = local_name {
            if has_known_ns {
                for import in scope.import_list().iter() {
                    if import.is::<PackageWildcardImport>() {
                        amb = self.lookup_in_object(&import.package(), &open_ns_set, qual.clone(), key, false)?;
                        if let Some(amb) = amb {
                            Unused(self.0).mark_used(&import);
                            if r.is_some() && !r.as_ref().unwrap().fixture_reference_value_equals(&amb) {
                                return Err(PropertyLookupError::AmbiguousReference(local_name));
                            }
                            r = Some(amb);
                        }
                    } else if import.is::<PackageRecursiveImport>() {
                        amb = self.lookup_in_package_recursive(&import.package(), &open_ns_set, qual.clone(), key)?;
                        if let Some(amb) = amb {
                            Unused(self.0).mark_used(&import);
                            if r.is_some() && !r.as_ref().unwrap().fixture_reference_value_equals(&amb) {
                                return Err(PropertyLookupError::AmbiguousReference(local_name));
                            }
                            r = Some(amb);
                        }
                    } else {
                        assert!(import.is::<PackagePropertyImport>());
                        let prop = map_defer_error(import.property().defer())?;
                        if prop.is::<InvalidationEntity>() {
                            continue;
                        }
                        if prop.name().matches_in_ns_set_or_any_public_ns(self.0, &open_ns_set, &local_name) {
                            Unused(self.0).mark_used(&import);

                            if r.is_some() && !r.as_ref().unwrap().fixture_reference_value_equals(&prop) {
                                return Err(PropertyLookupError::AmbiguousReference(local_name));
                            }

                            let prop = prop.resolve_alias();

                            defer(&prop.property_static_type(self.0))?;

                            r = Some(map_defer_error(prop.wrap_property_reference(self.0))?);
                        }
                    }
                }
            }
        }

        if r.is_none() {
            if let Some(parent) = scope.parent() {
                return self.lookup_in_scope_chain(&parent, qual, key);
            }
        }

        Ok(r)
    }

    /// Qualifier is assumed to be a compile-time namespace.
    pub fn get_qname_in_ns_set_or_any_public_ns(&self, mapping: &Names, open_ns_set: &SharedArray<Entity>, qual: Option<Entity>, local_name: &str) -> Result<Option<Entity>, PropertyLookupError> {
        if let Some(qual) = qual {
            if qual.is::<PackageWildcardImport>() || qual.is::<PackageRecursiveImport>() {
                return Ok(None);
            }
            let qual = if qual.is::<NamespaceConstant>() {
                qual.referenced_ns()
            } else {
                qual.clone()
            };
            if let Some(k) = qual.system_ns_kind() {
                if k == SystemNamespaceKind::Public {
                    mapping.get_in_system_ns_kind(k, local_name).map_err(|e| PropertyLookupError::AmbiguousReference(e.0))
                } else {
                    mapping.get_in_system_ns_kind_in_ns_set(open_ns_set, k, local_name).map_err(|e| PropertyLookupError::AmbiguousReference(e.0))
                }
            } else {
                Ok(mapping.get(&self.0.factory().create_qname(&qual, local_name.to_owned())))
            }
        } else {
            mapping.get_in_ns_set_or_any_public_ns(open_ns_set, local_name).map_err(|e| PropertyLookupError::AmbiguousReference(e.0))
        }
    }

    pub fn lookup_in_package_recursive(&self, package: &Entity, open_ns_set: &SharedArray<Entity>, qual: Option<Entity>, local_name: &PropertyLookupKey) -> Result<Option<Entity>, PropertyLookupError> {
        let mut r = self.lookup_in_object(&package, &open_ns_set, qual.clone(), local_name, false)?;

        for (_, subpackage) in package.subpackages().borrow().iter() {
            let r1 = self.lookup_in_package_recursive(subpackage, open_ns_set, qual.clone(), local_name)?;
            if r1.is_some() {
                if r.is_some() {
                    return Err(PropertyLookupError::AmbiguousReference(local_name.local_name().unwrap()));
                }
                r = r1;
            }
        }

        Ok(r)
    }
}