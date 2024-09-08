use crate::ns::*;
use bitflags::bitflags;
use hydroperfox_smodel::smodel;

smodel! {
    type Arena = EntityArena;

    /// Semantic data type representing one of several ActionScript 3 elements.
    pub struct Entity {
        pub fn defer(&self) -> Result<Entity, DeferError> {
            if self.is::<UnresolvedEntity>() {
                Err(DeferError(None))
            } else {
                Ok(self.clone())
            }
        }

        pub fn location(&self) -> Option<Location> {
            panic!();
        }

        pub fn set_location(&self, loc: Option<Location>) {
            panic!();
        }

        pub fn qualifier(&self) -> Option<Entity> {
            panic!();
        }

        pub fn key(&self) -> Entity {
            panic!();
        }

        /// Returns the static type of a property, whether for a type, variable, virtual or method slot or namespace,
        /// or act as identity of a value's static type.
        /// Possibly `UnresolvedEntity`.
        pub fn property_static_type(&self, host: &Database) -> Entity {
            panic!();
        }

        pub fn is_dynamic_or_inherits_dynamic(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub fn tuple_index(&self) -> usize {
            0
        }

        pub fn system_ns_kind(&self) -> Option<SystemNamespaceKind> {
            None
        }

        pub fn asdoc(&self) -> Option<Rc<Asdoc>> {
            None
        }

        pub fn set_asdoc(&self, asdoc: Option<Rc<Asdoc>>) {}

        pub fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            panic!();
        }

        pub fn is_external(&self) -> bool {
            false
        }

        pub fn set_is_external(&self, value: bool) {
            panic!();
        }

        /// Indicates whether a variable slot is optional for an object initializer
        /// applied to an `[Options]` class.
        pub fn is_opt_variable_for_options_class(&self, host: &Database) -> Result<bool, DeferError> {
            let st = self.static_type(host).defer()?;
            Ok(st.includes_null(host)? || st.includes_undefined(host)?)
        }

        /// Escapes out of a nullable type layer.
        pub fn escape_of_nullable(&self) -> Entity {
            self.clone()
        }

        /// Escapes out of a non nullable type layer.
        pub fn escape_of_non_nullable(&self) -> Entity {
            self.clone()
        }

        /// Escapes out of a nullable or non nullable type layer.
        pub fn escape_of_nullable_or_non_nullable(&self) -> Entity {
            self.clone()
        }

        pub fn object(&self) -> Entity {
            panic!();
        }

        pub fn uri(&self) -> String {
            "".into()
        }

        pub fn open_ns_set(&self) -> SharedArray<Entity> {
            panic!();
        }

        pub fn import_list(&self) -> SharedArray<Entity> {
            panic!();
        }

        pub fn is_class_or_equivalent(&self) -> bool {
            self.is_class_type_possibly_after_sub()
            || self.is::<EnumType>()
            || self.is::<TupleType>()
            || self.is::<FunctionType>()
        }

        pub fn local_name(&self) -> String {
            "".into()
        }

        pub fn class(&self) -> Entity {
            panic!();
        }

        pub fn interface(&self) -> Entity {
            panic!();
        }

        pub fn package(&self) -> Entity {
            panic!();
        }

        pub fn of_method(&self) -> Entity {
            panic!();
        }
        
        pub fn search_activation(&self) -> Option<Entity> {
            for scope in self.descending_scope_hierarchy() {
                if scope.is::<Activation>() {
                    return Some(scope);
                }
            }
            return None
        }

        pub fn this(&self) -> Option<Entity> {
            panic!();
        }
        
        pub fn set_this(&self, this: Option<Entity>) {
            panic!();
        }
        
        pub fn property_has_capture(&self, property: &Entity) -> bool {
            panic!();
        }
        
        pub fn set_property_has_capture(&self, property: &Entity, value: bool) {
            panic!();
        }

        pub fn concat_open_ns_set_of_scope_chain(&self) -> SharedArray<Entity> {
            let mut open_ns_set = SharedArray::new();
            open_ns_set.extend(self.open_ns_set().iter());
            let mut p = self.parent();
            while let Some(p1) = p {
                open_ns_set.extend(p1.open_ns_set().iter());
                p = p1.parent();
            }
            open_ns_set
        }

        pub fn referenced_type(&self) -> Entity {
            panic!();
        }

        pub fn referenced_ns(&self) -> Entity {
            panic!();
        }

        pub fn shorthand_resolution(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_shorthand_resolution(&self, value: Option<Entity>) {
            panic!();
        }

        pub fn field_slot(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_field_slot(&self, value: Option<Entity>) {
            panic!();
        }

        pub fn is_entity_after_substitution(&self) -> bool {
            self.is::<TypeAfterSubstitution>() ||
            self.is::<VariableSlotAfterSubstitution>() ||
            self.is::<VirtualSlotAfterSubstitution>() ||
            self.is::<MethodSlotAfterSubstitution>()
        }

        pub fn parent(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_parent(&self, p: Option<Entity>) {
            panic!();
        }

        pub fn package_concats(&self) -> SharedArray<Entity> {
            panic!();
        }

        pub fn public_ns(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_public_ns(&self, ns: Option<Entity>) {
            panic!();
        }

        pub fn private_ns(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_private_ns(&self, ns: Option<Entity>) {
            panic!();
        }

        pub fn protected_ns(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_protected_ns(&self, ns: Option<Entity>) {
            panic!();
        }

        pub fn static_protected_ns(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_static_protected_ns(&self, ns: Option<Entity>) {
            panic!();
        }

        pub fn internal_ns(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_internal_ns(&self, ns: Option<Entity>) {
            panic!();
        }

        pub fn is_public_ns(&self) -> bool {
            false
        }

        pub fn is_private_ns(&self) -> bool {
            false
        }

        pub fn is_protected_ns(&self) -> bool {
            false
        }

        pub fn is_internal_ns(&self) -> bool {
            false
        }

        pub fn is_static_protected_ns(&self) -> bool {
            false
        }

        pub fn is_parameterized_type_or_type_after_sub(&self) -> bool {
            if self.is::<ClassType>() || self.is::<InterfaceType>() {
                self.type_params().is_some()
            } else {
                self.is::<TypeAfterSubstitution>()
            }
        }

        pub fn is_package_self_referential(&self, pckg: &Entity) -> bool {
            if self == pckg {
                return true;
            }
            let mut p = self.parent();
            while let Some(p1) = p {
                if &p1 == pckg {
                    return true;
                }
                p = p1.parent();
            }
            false
        }

        pub fn is_global_initialization(&self) -> bool {
            panic!();
        }

        pub fn set_is_global_initialization(&self, value: bool) {
            panic!();
        }

        pub fn is_package_initialization(&self) -> bool {
            panic!();
        }

        pub fn set_is_package_initialization(&self, value: bool) {
            panic!();
        }

        pub fn number_value(&self) -> Number {
            panic!();
        }

        pub fn string_value(&self) -> String {
            panic!();
        }

        pub fn boolean_value(&self) -> bool {
            panic!();
        }

        pub fn type_default_value(&self, host: &Database) -> Result<Option<Entity>, DeferError> {
            panic!();
        }

        pub fn conversion_kind(&self) -> ConversionKind {
            panic!();
        }

        pub fn conversion_is_opt(&self) -> bool {
            panic!();
        }

        pub fn conversion_target(&self) -> Entity {
            panic!();
        }

        /// Returns whether a type is a class, whether
        /// original or after substitution.
        pub fn is_class_type_possibly_after_sub(&self) -> bool {
            false
        }

        /// Returns whether a type is an interface, whether
        /// original or after substitution.
        pub fn is_interface_type_possibly_after_sub(&self) -> bool {
            false
        }

        /// Event mapping from `[Event(name="eventName", type="T")]` meta-data.
        pub fn events(&self) -> SharedMap<String, Event> {
            panic!();
        }

        pub fn bindable_event(&self) -> Option<String> {
            panic!();
        }

        pub fn set_bindable_event(&self, name: Option<String>) {
            panic!();
        }

        pub fn scope(&self) -> Entity {
            panic!();
        }

        pub fn getter(&self, host: &Database) -> Option<Entity> {
            panic!();
        }

        pub fn set_getter(&self, m: Option<Entity>) {
            panic!();
        }

        pub fn setter(&self, host: &Database) -> Option<Entity> {
            panic!();
        }

        pub fn set_setter(&self, m: Option<Entity>) {
            panic!();
        }

        pub fn static_type(&self, host: &Database) -> Entity {
            panic!();
        }

        pub fn set_static_type(&self, value: Entity) {
            panic!();
        }

        pub fn clone_constant(&self, host: &Database) -> Entity {
            panic!();
        }

        pub fn is_abstract(&self) -> bool {
            false
        }

        pub fn set_is_abstract(&self, value: bool) {
        }

        pub fn is_native(&self) -> bool {
            false
        }

        pub fn set_is_native(&self, value: bool) {
        }

        pub fn is_final(&self) -> bool {
            false
        }

        pub fn set_is_final(&self, value: bool) {
        }

        pub fn is_dynamic(&self) -> bool {
            false
        }

        pub fn set_is_dynamic(&self, value: bool) {
        }

        pub fn is_options_class(&self) -> bool {
            false
        }

        pub fn set_is_options_class(&self, value: bool) {
        }

        pub fn is_static(&self) -> bool {
            false
        }

        pub fn set_is_static(&self, value: bool) {
        }

        pub fn is_overriding(&self) -> bool {
            false
        }

        pub fn set_is_overriding(&self, value: bool) {
        }

        pub fn is_async(&self) -> bool {
            false
        }

        pub fn set_is_async(&self, value: bool) {
        }

        pub fn is_generator(&self) -> bool {
            false
        }

        pub fn set_is_generator(&self, value: bool) {
        }

        pub fn signature(&self, host: &Database) -> Entity {
            panic!();
        }

        pub fn set_signature(&self, signature: &Entity) {
            panic!();
        }

        /// If a type is `[T]`, returns `T`, either as an origin type parameter
        /// or as a substitute type.
        pub fn array_element_type(&self, host: &Database) -> Result<Option<Entity>, DeferError> {
            let array_type = host.array_type().defer()?;
            if self == &array_type {
                Ok(Some(array_type.type_params().unwrap().get(0).unwrap()))
            } else if self.type_after_sub_has_origin(&array_type) {
                Ok(Some(self.substitute_types().get(0).unwrap()))
            } else {
                Ok(None)
            }
        }

        /// If a type is `Vector.<T>`, returns `T`, either as an origin type parameter
        /// or as a substitute type.
        pub fn vector_element_type(&self, host: &Database) -> Result<Option<Entity>, DeferError> {
            let vec_type = host.vector_type().defer()?;
            if self == &vec_type {
                Ok(Some(vec_type.type_params().unwrap().get(0).unwrap()))
            } else if self.type_after_sub_has_origin(&vec_type) {
                Ok(Some(self.substitute_types().get(0).unwrap()))
            } else {
                Ok(None)
            }
        }

        /// If a type is `Promise.<T>`, returns `T`, either as an origin type parameter
        /// or as a substitute type.
        pub fn promise_result_type(&self, host: &Database) -> Result<Option<Entity>, DeferError> {
            let promise_type = host.promise_type().defer()?;
            if self == &promise_type {
                Ok(Some(promise_type.type_params().unwrap().get(0).unwrap()))
            } else if self.type_after_sub_has_origin(&promise_type) {
                Ok(Some(self.substitute_types().get(0).unwrap()))
            } else {
                Ok(None)
            }
        }

        pub fn type_after_sub_has_origin(&self, origin: &Entity) -> bool {
            self.is::<TypeAfterSubstitution>() && &self.origin() == origin
        }

        pub fn is_type_or_type_after_sub_has_origin(&self, type_or_origin: &Entity) -> bool {
            self == type_or_origin || self.type_after_sub_has_origin(type_or_origin)
        }

        pub fn origin_or_parameterized_type_identity(&self) -> Option<Entity> {
            if self.is::<TypeAfterSubstitution>() {
                Some(self.origin())
            } else if self.type_params().is_some() {
                Some(self.clone())
            } else {
                None
            }
        }

        /// Iterator over a descending class hierarchy.
        pub fn descending_class_hierarchy<'a>(&self, host: &'a Database) -> DescendingClassHierarchy<'a> {
            DescendingClassHierarchy(Some(self.clone()), host, self.clone())
        }

        /// Iterator over a descending scope hierarchy.
        pub fn descending_scope_hierarchy(&self) -> DescendingScopeHierarchy {
            DescendingScopeHierarchy(Some(self.clone()))
        }

        /// Iterator over a descending definition hierarchy.
        pub fn descending_definition_hierarchy(&self) -> DescendingDefinitionHierarchy {
            DescendingDefinitionHierarchy(Some(self.clone()))
        }

        pub fn is_namespace_or_ns_constant(&self) -> bool {
            false
        }

        pub fn wrap_property_reference(&self, host: &Database) -> Result<Entity, DeferError> {
            if self.is::<Value>() {
                return Ok(self.clone());
            }
            if self.is::<Type>() && (
                self.is::<VoidType>() ||
                self.is::<AnyType>() ||
                self.is::<FunctionType>() ||
                self.is::<TupleType>() ||
                self.is::<NullableType>() ||
                self.is::<NonNullableType>() ||
                self.is::<TypeAfterSubstitution>()) {
                return host.factory().create_type_constant(self);
            }
            if self.is::<Namespace>() {
                return host.factory().create_namespace_constant(self);
            }
            if self.is::<InvalidationEntity>() {
                return Ok(self.clone());
            }
            let parent = self.parent().unwrap();
            if parent.is::<ClassType>() || parent.is::<EnumType>() {
                return host.factory().create_static_reference_value(&parent, self);
            }
            if parent.is::<Package>() {
                return host.factory().create_package_reference_value(&parent, self);
            }
            assert!(parent.is::<Scope>());
            return host.factory().create_scope_reference_value(&parent, self);
        }

        pub fn activation(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_activation(&self, activation: Option<Entity>) {
            panic!();
        }

        pub fn of_virtual_slot(&self, host: &Database) -> Option<Entity> {
            panic!();
        }

        pub fn set_of_virtual_slot(&self, virtual_slot: Option<Entity>) {
            panic!();
        }

        pub fn overriden_by(&self, host: &Database) -> SharedArray<Entity> {
            panic!();
        }

        pub fn overrides_method(&self, host: &Database) -> Option<Entity> {
            panic!();
        }

        pub fn set_overrides_method(&self, method: Option<Entity>) {
            panic!();
        }

        pub fn is_constructor(&self) -> bool {
            false
        }

        pub fn set_is_constructor(&self, value: bool) {
        }

        pub fn constructor_method(&self, host: &Database) -> Option<Entity> {
            panic!();
        }

        pub fn set_constructor_method(&self, m: Option<Entity>) {}

        pub fn known_subclasses(&self) -> SharedArray<Entity> {
            panic!();
        }

        /// Includes possibly unresolved.
        pub fn implements(&self, host: &Database) -> SharedArray<Entity> {
            panic!();
        }

        /// Possibly unresolved.
        pub fn extends_class(&self, host: &Database) -> Option<Entity> {
            None
        }

        pub fn set_extends_class(&self, entity: Option<Entity>) {
            panic!();
        }

        pub fn prototype(&self, host: &Database) -> Names {
            Names::new()
        }

        pub fn properties(&self, host: &Database) -> Names {
            Names::new()
        }

        pub fn subpackages(&self) -> SharedMap<String, Entity> {
            panic!();
        }

        pub fn alias_of(&self) -> Entity {
            panic!();
        }

        pub fn set_alias_of(&self, value: &Entity) {
            panic!();
        }

        pub fn resolve_alias(&self) -> Entity {
            self.clone()
        }

        pub fn property(&self) -> Entity {
            panic!();
        }

        pub fn set_property(&self, value: &Entity) {
            panic!();
        }

        pub fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            panic!();
        }

        pub fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            panic!();
        }

        pub fn name(&self) -> QName {
            panic!();
        }

        pub fn fully_qualified_name(&self) -> String {
            self.fully_qualified_name_list().join(".").replace("__AS3__.vec.Vector", "Vector")
        }
    
        pub fn fully_qualified_name_list(&self) -> Vec<String> {
            let mut r: Vec<String> = vec![];
            let mut p = Some(self.clone());
            while let Some(p1) = p {
                let name = if p1.is::<Package>() {
                    p1.local_name()
                } else {
                    p1.name().to_string()
                };
                if !name.is_empty() {
                    r.insert(0, name);
                }
                p = p1.parent();
            }
            r
        }

        pub fn available_definitions_in_package(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            Ok(vec![])
        }

        pub fn available_static_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            Ok(vec![])
        }

        pub fn available_prototype_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            Ok(vec![])
        }

        pub fn type_params(&self) -> Option<SharedArray<Entity>> {
            None
        }

        pub fn set_type_params(&self, list: Option<SharedArray<Entity>>) {
        }

        pub fn enum_member_number_mapping(&self) -> SharedMap<String, Number> {
            panic!();
        }

        pub fn enum_member_slot_mapping(&self) -> SharedMap<String, Entity> {
            panic!();
        }

        pub fn known_implementors(&self) -> SharedArray<Entity> {
            panic!();
        }

        /// Includes possibly unresolved.
        pub fn extends_interfaces(&self, host: &Database) -> SharedArray<Entity> {
            panic!();
        }

        pub fn origin(&self) -> Entity {
            panic!();
        }

        pub fn substitute_types(&self) -> SharedArray<Entity> {
            panic!();
        }

        pub fn indirect_type_params(&self) -> SharedArray<Entity> {
            panic!();
        }

        pub fn indirect_substitute_types(&self) -> SharedArray<Entity> {
            panic!();
        }

        pub fn element_types(&self) -> SharedArray<Entity> {
            panic!();
        }

        pub fn params(&self) -> SharedArray<Rc<SemanticFunctionTypeParameter>> {
            panic!();
        }

        pub fn result_type(&self) -> Entity {
            panic!();
        }

        pub fn base(&self) -> Entity {
            panic!();
        }

        /// Performs type substitution.
        pub fn apply_type(&self, host: &Database, type_params: &SharedArray<Entity>, substitute_types: &SharedArray<Entity>) -> Entity {
            ApplyType(host).exec(self, type_params, substitute_types)
        }

        pub fn read_only(&self, host: &Database) -> bool {
            true
        }

        pub fn set_read_only(&self, value: bool) {
            panic!();
        }

        pub fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub fn set_write_only(&self, value: bool) {
            panic!();
        }

        pub fn deletable(&self, host: &Database) -> bool {
            false
        }

        pub fn var_constant(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_var_constant(&self, k: Option<Entity>) {
            panic!();
        }

        pub fn is_ascending_type_of(&self, possibly_subtype: &Entity, host: &Database) -> Result<bool, DeferError> {
            possibly_subtype.is_subtype_of(self, host)
        }
    
        pub fn is_subtype_of(&self, possibly_ascending_type: &Entity, host: &Database) -> Result<bool, DeferError> {
            if possibly_ascending_type.is::<AnyType>() {
                return Ok(true);
            }
            for t in self.all_ascending_types(host) {
                // Defer if unresolved
                t.defer()?;

                if &t == possibly_ascending_type {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    
        pub fn is_equals_or_subtype_of(&self, other: &Entity, host: &Database) -> Result<bool, DeferError> {
            Ok(self == other || self.is_subtype_of(other, host)?)
        }
    
        /// Returns all ascending types of a type in ascending type order,
        /// each possibly unresolved.
        pub fn all_ascending_types(&self, host: &Database) -> Vec<Entity> {
            self.all_ascending_types_non_circular(host, self)
        }

        fn all_ascending_types_non_circular(&self, host: &Database, descending_most: &Entity) -> Vec<Entity> {
            let mut r = vec![];
            let mut r2 = vec![];
            for type_thing in self.direct_ascending_types(host) {
                if !type_thing.is::<UnresolvedEntity>() {
                    if &type_thing != descending_most {
                        for type1 in type_thing.all_ascending_types(host) {
                            if !r.contains(&type1) && &type1 != descending_most {
                                r.push(type1.clone());
                            }
                        }
                    }
                }
                if !r.contains(&type_thing) && &type_thing != descending_most {
                    r2.push(type_thing.clone());
                }
            }
            r.extend(r2);
            r
        }
    
        /// Returns direct ascending types of a type, each possibly unresolved.
        pub fn direct_ascending_types(&self, host: &Database) -> Vec<Entity> {
            if self.is::<ClassType>() {
                let mut r: Vec<Entity> = self.implements(host).iter().collect();
                if let Some(ascending_class) = self.extends_class(host) {
                    r.push(ascending_class);
                }
                return r;
            } else if self.is::<EnumType>() {
                return vec![self.extends_class(host).unwrap()];
            } else if self.is::<InterfaceType>() {
                return self.extends_interfaces(host).iter().collect();
            } else if self.is::<FunctionType>() {
                return vec![host.function_type()];
            } else if self.is::<TupleType>() {
                return vec![host.array_type_of_any().unwrap_or(host.unresolved_entity())];
            } else if self.is::<InvalidationEntity>() {
                return vec![];
            }
            return vec![];
        }
    
        pub(crate) fn not_overriden_abstract_getter(&self, getter_from_base_class: &Entity, subclass: &Entity, host: &Database) -> bool {
            if getter_from_base_class.is_abstract() {
                let name = &getter_from_base_class.name();
                let prop2 = if name.namespace().is::<SystemNamespace>() {
                    subclass.prototype(host).get_in_system_ns_kind(name.namespace().system_ns_kind().unwrap(), &name.local_name()).ok().unwrap_or(None)
                } else {
                    subclass.prototype(host).get(name)
                };
                prop2.is_none() || !prop2.clone().unwrap().is::<VirtualSlot>() || prop2.unwrap().getter(host).is_none()
            } else {
                false
            }
        }
    
        pub(crate) fn not_overriden_abstract_setter(&self, setter_from_base_class: &Entity, subclass: &Entity, host: &Database) -> bool {
            if setter_from_base_class.is_abstract() {
                let name = &setter_from_base_class.name();
                let prop2 = if name.namespace().is::<SystemNamespace>() {
                    subclass.prototype(host).get_in_system_ns_kind(name.namespace().system_ns_kind().unwrap(), &name.local_name()).ok().unwrap_or(None)
                } else {
                    subclass.prototype(host).get(name)
                };
                prop2.is_none() || !prop2.clone().unwrap().is::<VirtualSlot>() || prop2.unwrap().setter(host).is_none()
            } else {
                false
            }
        }

        pub fn is_comparison_between_unrelated_types(&self, other: &Entity, host: &Database) -> Result<bool, DeferError> {
            let left = self.escape_of_nullable_or_non_nullable();
            let right = other.escape_of_nullable_or_non_nullable();

            if left == right || [left.clone(), right.clone()].contains(&host.any_type()) {
                return Ok(false);
            }

            let primitive_types = host.primitive_types()?;

            if primitive_types.contains(&left) || primitive_types.contains(&right) {
                return Ok(false);
            }

            if !(left.is_ascending_type_of(&right, host)? || left.is_subtype_of(&right, host)?) {
                return Ok(true);
            }

            Ok(false)
        }

        pub fn expect_type(&self) -> Result<Entity, TypeExpectError> {
            if let Some(t) = self.as_type() {
                Ok(t)
            } else {
                Err(TypeExpectError())
            }
        }

        pub fn as_type(&self) -> Option<Entity> {
            if self.is::<TypeConstant>() {
                return Some(self.referenced_type());
            }
            if self.is::<FixtureReferenceValue>() {
                return self.property().as_type();
            }
            if self.is::<Type>() { Some(self.clone()) } else { None }
        }
        
        pub fn fixture_reference_value_equals(&self, other: &Entity) -> bool {
            if other.is::<FixtureReferenceValue>() {
                if self.is::<FixtureReferenceValue>() {
                    self.property() == other.property()
                } else {
                    self == &other.property()
                }
            } else if self.is::<FixtureReferenceValue>() {
                &self.property() == other
            } else {
                self == other
            }
        }

        pub fn control_flow_graph(&self) -> ControlFlowGraph {
            panic!();
        }

        /// Lookups property in an object.
        pub fn lookup_in_object(&self, host: &Database, open_ns_set: &SharedArray<Entity>, qual: Option<Entity>, key: &PropertyLookupKey, calling: bool) -> Result<Option<Entity>, PropertyLookupError> {
            PropertyLookup(host).lookup_in_object(self, open_ns_set, qual, key, calling)
        }

        /// Lookups property in the scope chain.
        pub fn lookup_in_scope_chain(&self, host: &Database, qual: Option<Entity>, key: &PropertyLookupKey) -> Result<Option<Entity>, PropertyLookupError> {
            PropertyLookup(host).lookup_in_scope_chain(self, qual, key)
        }

        pub fn search_system_ns_in_scope_chain(&self, ns: SystemNamespaceKind) -> Option<Entity> {
            let mut scope = Some(self.clone());
            while let Some(scope1) = scope {
                if scope1.is::<PackageScope>() || scope1.is::<Activation>() {
                    if ns == SystemNamespaceKind::Public && scope1.public_ns().is_some() {
                        return scope1.public_ns();
                    }
                    if ns == SystemNamespaceKind::Internal && scope1.internal_ns().is_some() {
                        return scope1.internal_ns();
                    }
                } else if scope1.is::<ClassScope>() {
                    if ns == SystemNamespaceKind::Private {
                        return scope1.private_ns();
                    }
                    if ns == SystemNamespaceKind::Protected {
                        return scope1.protected_ns();
                    }
                    if ns == SystemNamespaceKind::StaticProtected {
                        return scope1.static_protected_ns();
                    }
                } else if scope1.is::<EnumScope>() {
                    if ns == SystemNamespaceKind::Private {
                        return scope1.private_ns();
                    }
                }
                scope = scope1.parent();
            }
            None
        }

        pub fn field_reference(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_field_reference(&self, value: Option<Entity>) {
            panic!();
        }

        pub fn var_slot(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_var_slot(&self, value: Option<Entity>) {
            panic!();
        }

        pub fn target_reference(&self) -> Option<Entity> {
            panic!();
        }

        pub fn set_target_reference(&self, value: Option<Entity>) {
            panic!();
        }

        pub fn search_hoist_scope(&self) -> Entity {
            let mut scope = Some(self.clone());
            while let Some(scope1) = scope {
                if scope1.is::<FixtureScope>() || scope1.is::<Activation>() {
                    return scope1;
                }
                scope = scope1.parent();
            }
            scope.unwrap()
        }

        pub fn is_empty_package(&self, host: &Database) -> bool {
            if self.properties(host).length() != 0 {
                return false;
            }
            for pckg in self.package_concats().iter() {
                if !pckg.is_empty_package(host) {
                    return false;
                }
            }
            true
        }

        pub fn is_empty_package_recursive(&self, host: &Database) -> bool {
            if !self.is_empty_package(host) {
                return false;
            }
            for (_, pckg) in self.subpackages().borrow().iter() {
                if !pckg.is_empty_package_recursive(host) {
                    return false;
                }
            }
            true
        }

        pub fn list_packages_recursively(&self) -> Vec<Entity> {
            let mut r: Vec<Entity> = vec![self.clone()];
            for (_, pckg) in self.subpackages().borrow().iter() {
                r.extend(pckg.list_packages_recursively());
            }
            r
        }

        fn to_string_1(&self) -> String {
            "".into()
        }
    }

    pub struct UnresolvedEntity: Entity {
        pub(crate) fn UnresolvedEntity() {
            super();
        }
    }

    /// Entity used to indicate that an entity is invalidated.
    pub struct InvalidationEntity: Entity {
        pub(crate) fn InvalidationEntity() {
            super();
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &Database) -> Entity {
            self.clone().into()
        }

        pub override fn static_type(&self, host: &Database) -> Entity {
            self.clone().into()
        }

        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            "[unknown]".into()
        }
    }

    /// Either a system, user, or explicit namespace. The *user* and *explicit*
    /// namespaces are redundant in terms of functionality.
    pub struct Namespace: Entity {
        pub(crate) fn Namespace() {
            super();
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &Database) -> Entity {
            host.namespace_type()
        }

        pub override fn is_namespace_or_ns_constant(&self) -> bool {
            true
        }
    }

    /// A public, private, protected, internal, or static-protected namespace.
    pub struct SystemNamespace: Namespace {
        let m_kind: SystemNamespaceKind = SystemNamespaceKind::Public;
        let ref m_parent: Option<Entity> = None;

        pub(crate) fn SystemNamespace(kind: SystemNamespaceKind, parent: Option<Entity>) {
            super();
            self.set_m_kind(kind);
            self.set_m_parent(parent);
        }

        pub override fn system_ns_kind(&self) -> Option<SystemNamespaceKind> {
            Some(self.m_kind())
        }

        pub override fn is_public_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::Public
        }

        pub override fn is_private_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::Private
        }

        pub override fn is_protected_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::Protected
        }

        pub override fn is_internal_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::Internal
        }

        pub override fn is_static_protected_ns(&self) -> bool {
            self.m_kind() == SystemNamespaceKind::StaticProtected
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }

        override fn to_string_1(&self) -> String {
            self.m_kind().to_string()
        }
    }

    /// In the AVM2, this is equivalent to a `CONSTANT_Namespace` namespace.
    /// In terms of functionality, this is redundant to `ExplicitNamespace`.
    pub struct UserNamespace: Namespace {
        let ref m_uri: String = "".into();

        pub(crate) fn UserNamespace(uri: String) {
            super();
            self.set_m_uri(uri);
        }

        pub override fn uri(&self) -> String {
            self.m_uri()
        }

        override fn to_string_1(&self) -> String {
            self.m_uri()
        }
    }

    /// In the AVM2, this is equivalent to a `CONSTANT_ExplicitNamespace` namespace.
    /// In terms of functionality, this is redundant to `UserNamespace`.
    pub struct ExplicitNamespace: Namespace {
        let ref m_uri: String = "".into();

        pub(crate) fn ExplicitNamespace(uri: String) {
            super();
            self.set_m_uri(uri);
        }

        pub override fn uri(&self) -> String {
            self.m_uri()
        }

        override fn to_string_1(&self) -> String {
            self.m_uri()
        }
    }

    /// A package consists of a local name, two namespaces, `public` and `internal`,
    /// and a mapping of subpackages.
    pub struct Package: Entity {
        let ref m_name: String = "".into();
        let ref m_parent: Option<Entity> = None;
        let ref m_public_ns: Option<Entity> = None;
        let ref m_internal_ns: Option<Entity> = None;
        let ref m_properties: Names = Names::new();
        let ref m_subpackages: SharedMap<String, Entity> = SharedMap::new();
        let ref m_package_concats: SharedArray<Entity> = SharedArray::new();
        let ref m_asdoc: Option<Rc<Asdoc>> = None;

        pub(crate) fn Package(name: String) {
            super();
            self.set_m_name(name);
        }

        /// The local name of the package. For the top-level package
        /// this is the empty string.
        pub override fn local_name(&self) -> String {
            self.m_name()
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Entity>) {
            self.set_m_parent(p);
        }

        /// Concatenated packages.
        pub override fn package_concats(&self) -> SharedArray<Entity> {
            self.m_package_concats()
        }

        pub override fn public_ns(&self) -> Option<Entity> {
            self.m_public_ns()
        }

        pub override fn set_public_ns(&self, ns: Option<Entity>) {
            self.set_m_public_ns(ns);
        }

        pub override fn internal_ns(&self) -> Option<Entity> {
            self.m_internal_ns()
        }

        pub override fn set_internal_ns(&self, ns: Option<Entity>) {
            self.set_m_internal_ns(ns);
        }

        pub override fn properties(&self, host: &Database) -> Names {
            self.m_properties()
        }

        pub override fn subpackages(&self) -> SharedMap<String, Entity> {
            self.m_subpackages()
        }

        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<Asdoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn available_definitions_in_package(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            let mut r: Vec<Entity> = vec![];
            for (name, ent) in self.properties(host).borrow().iter() {
                if name.accessible_from_ns_set(host, ns_set) {
                    r.push(ent.clone());
                }
            }
            Ok(r)
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    /// An alias resulting from either an `import` directive, or a `type` definition,
    /// or a `public += com.p.n;` directive, for example.
    pub struct Alias: Entity {
        let ref m_name: Option<QName> = None;
        let ref m_alias_of: Option<Entity> = None;
        let ref m_parent: Option<Entity> = None;
        let ref m_location: Option<Location> = None;

        pub(crate) fn Alias(name: QName, alias_of: Entity) {
            super();
            self.set_m_name(Some(name));
            self.set_m_alias_of(Some(alias_of));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn alias_of(&self) -> Entity {
            self.m_alias_of().unwrap()
        }

        pub override fn set_alias_of(&self, value: &Entity) {
            self.set_m_alias_of(Some(value.clone()));
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        pub override fn resolve_alias(&self) -> Entity {
            self.alias_of().resolve_alias()
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Entity>) {
            self.set_m_parent(p);
        }

        override fn to_string_1(&self) -> String {
            self.alias_of().to_string_1()
        }
    }

    /// One of several types.
    pub struct Type: Entity {
        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(false)
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &Database) -> Entity {
            host.class_type()
        }

        pub override fn type_default_value(&self, host: &Database) -> Result<Option<Entity>, DeferError> {
            if self.includes_undefined(host)? {
                Ok(Some(host.factory().create_undefined_constant(self)))
            } else if self.includes_null(host)? {
                Ok(Some(host.factory().create_null_constant(self)))
            } else if host.numeric_types()?.contains(self) {
                if host.floating_point_types()?.contains(self) {
                    let v = Number::nan(self, host);
                    return Ok(Some(host.factory().create_number_constant(v, self)));
                }
                let v = Number::zero(self, host);
                Ok(Some(host.factory().create_number_constant(v, self)))
            } else if <Type as Into<Entity>>::into(self.clone()) == host.boolean_type().defer()? {
                Ok(Some(host.factory().create_boolean_constant(false, self)))
            } else {
                Ok(None)
            }
        }
    }

    /// The `*` type.
    pub struct AnyType : Type {
        pub(crate) fn AnyType() {
            super();
        }

        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            "*".into()
        }
    }

    /// The `void` type.
    pub struct VoidType : Type {
        pub(crate) fn VoidType() {
            super();
        }

        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(false)
        }

        override fn to_string_1(&self) -> String {
            "void".into()
        }
    }

    /// A `class` definitino, possibly parameterized.
    pub struct ClassType: Type {
        let ref m_name: Option<QName> = None;
        let m_flags: ClassTypeFlags = ClassTypeFlags::empty();
        let ref m_type_params: Option<SharedArray<Entity>> = None;
        let ref m_extends_class: Option<Entity> = None;
        let ref m_implements: SharedArray<Entity> = SharedArray::new();
        let ref m_known_subclasses: SharedArray<Entity> = SharedArray::new();
        let ref m_constructor_method: Option<Entity> = None;
        let ref m_parent: Option<Entity> = None;
        let ref m_private_ns: Option<Entity> = None;
        let ref m_protected_ns: Option<Entity> = None;
        let ref m_static_protected_ns: Option<Entity> = None;
        let ref m_properties: Names = Names::new();
        let ref m_prototype: Names = Names::new();
        let ref m_events: SharedMap<String, Event> = SharedMap::new();
        let ref m_asdoc: Option<Rc<Asdoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_location: Option<Location> = None;

        pub(crate) fn ClassType(name: QName) {
            super();
            self.set_m_name(Some(name));
        }

        pub override fn is_class_type_possibly_after_sub(&self) -> bool {
            true
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        #[inheritdoc]
        pub override fn events(&self) -> SharedMap<String, Event> {
            self.m_events()
        }

        pub override fn private_ns(&self) -> Option<Entity> {
            self.m_private_ns()
        }

        pub override fn set_private_ns(&self, ns: Option<Entity>) {
            self.set_m_private_ns(ns);
        }

        pub override fn protected_ns(&self) -> Option<Entity> {
            self.m_protected_ns()
        }

        pub override fn set_protected_ns(&self, ns: Option<Entity>) {
            self.set_m_protected_ns(ns);
        }

        pub override fn static_protected_ns(&self) -> Option<Entity> {
            self.m_static_protected_ns()
        }

        pub override fn set_static_protected_ns(&self, ns: Option<Entity>) {
            self.set_m_static_protected_ns(ns);
        }

        pub override fn type_params(&self) -> Option<SharedArray<Entity>> {
            self.m_type_params()
        }

        pub override fn set_type_params(&self, list: Option<SharedArray<Entity>>) {
            self.set_m_type_params(list);
        }

        pub override fn is_abstract(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_ABSTRACT)
        }

        pub override fn set_is_abstract(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_ABSTRACT, value);
            self.set_m_flags(v);
        }

        pub override fn is_final(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_FINAL)
        }

        pub override fn set_is_final(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_FINAL, value);
            self.set_m_flags(v);
        }

        pub override fn is_static(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_STATIC)
        }

        pub override fn set_is_static(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_STATIC, value);
            self.set_m_flags(v);
        }

        pub override fn is_dynamic(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_DYNAMIC)
        }

        pub override fn set_is_dynamic(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_DYNAMIC, value);
            self.set_m_flags(v);
        }

        pub override fn is_dynamic_or_inherits_dynamic(&self, host: &Database) -> Result<bool, DeferError> {
            if self.is_dynamic() {
                return Ok(true);
            }
            if let Some(cb) = self.extends_class(host) {
                cb.is_dynamic_or_inherits_dynamic(host)
            } else {
                Ok(false)
            }
        }

        /// Whether the class is an `[Options]` class.
        pub override fn is_options_class(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_OPTIONS_CLASS)
        }

        pub override fn set_is_options_class(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_OPTIONS_CLASS, value);
            self.set_m_flags(v);
        }

        pub override fn known_subclasses(&self) -> SharedArray<Entity> {
            self.m_known_subclasses()
        }

        #[inheritdoc]
        pub override fn implements(&self, host: &Database) -> SharedArray<Entity> {
            self.m_implements()
        }

        #[inheritdoc]
        pub override fn extends_class(&self, host: &Database) -> Option<Entity> {
            self.m_extends_class()
        }

        pub override fn set_extends_class(&self, entity: Option<Entity>) {
            self.set_m_extends_class(entity);
        }

        pub override fn properties(&self, host: &Database) -> Names {
            self.m_properties()
        }

        pub override fn prototype(&self, host: &Database) -> Names {
            self.m_prototype()
        }

        pub override fn constructor_method(&self, host: &Database) -> Option<Entity> {
            self.m_constructor_method()
        }

        pub override fn set_constructor_method(&self, m: Option<Entity>) {
            self.set_m_constructor_method(m);
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Entity>) {
            self.set_m_parent(p);
        }

        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<Asdoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.m_metadata()
        }

        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(!host.non_null_primitive_types()?.contains(&self.clone().into()))
        }

        pub override fn available_static_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            let mut r: Vec<Entity> = vec![];
            let mut c: Entity = self.clone().into();
            loop {
                for (name, ent) in c.properties(host).borrow().iter() {
                    if name.accessible_from_ns_set(host, ns_set) {
                        r.push(ent.clone());
                    }
                }
                if let Some(c1) = c.extends_class(host) {
                    c = c1.defer()?;
                } else {
                    break;
                }
            }
            Ok(r)
        }

        pub override fn available_prototype_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            let mut r: Vec<Entity> = vec![];
            let mut c: Entity = self.clone().into();
            loop {
                for (name, ent) in c.prototype(host).borrow().iter() {
                    if name.accessible_from_ns_set(host, ns_set) {
                        r.push(ent.clone());
                    }
                }
                if let Some(c1) = c.extends_class(host) {
                    c = c1.defer()?;
                } else {
                    break;
                }
            }
            Ok(r)
        }

        pub override fn is_external(&self) -> bool {
            self.m_flags().contains(ClassTypeFlags::IS_EXTERNAL)
        }

        pub override fn set_is_external(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(ClassTypeFlags::IS_EXTERNAL, value);
            self.set_m_flags(v);
        }

        override fn to_string_1(&self) -> String {
            let name_1 = self.fully_qualified_name();
            let mut p = String::new();
            if let Some(type_params) = self.type_params() {
                p = ".<".to_owned() + &type_params.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ") + ">";
            }
            name_1 + &p
        }
    }

    /// A `enum` definition.
    pub struct EnumType: Type {
        let ref m_name: Option<QName> = None;
        let ref m_parent: Option<Entity> = None;
        let ref m_private_ns: Option<Entity> = None;
        let ref m_properties: Names = Names::new();
        let ref m_prototype: Names = Names::new();
        let ref m_number_mapping: SharedMap<String, Number> = SharedMap::new();
        let ref m_slot_mapping: SharedMap<String, Entity> = SharedMap::new();
        let ref m_asdoc: Option<Rc<Asdoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_location: Option<Location> = None;

        pub(crate) fn EnumType(name: QName) {
            super();
            self.set_m_name(Some(name));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        /// Mapping from member's String to Number.
        pub override fn enum_member_number_mapping(&self) -> SharedMap<String, Number> {
            self.m_number_mapping()
        }

        /// Mapping from member's String to the static variable slot
        /// used in `properties()`.
        pub override fn enum_member_slot_mapping(&self) -> SharedMap<String, Entity> {
            self.m_slot_mapping()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        pub override fn private_ns(&self) -> Option<Entity> {
            self.m_private_ns()
        }

        pub override fn set_private_ns(&self, ns: Option<Entity>) {
            self.set_m_private_ns(ns);
        }

        pub override fn is_abstract(&self) -> bool {
            false
        }

        pub override fn is_final(&self) -> bool {
            true
        }

        pub override fn is_dynamic(&self) -> bool {
            false
        }

        pub override fn is_options_class(&self) -> bool {
            false
        }

        #[inheritdoc]
        pub override fn extends_class(&self, host: &Database) -> Option<Entity> {
            Some(host.object_type())
        }

        pub override fn properties(&self, host: &Database) -> Names {
            self.m_properties()
        }

        pub override fn prototype(&self, host: &Database) -> Names {
            self.m_prototype()
        }

        pub override fn available_static_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            let mut r: Vec<Entity> = vec![];
            let mut c: Entity = self.clone().into();
            loop {
                for (name, ent) in c.properties(host).borrow().iter() {
                    if name.accessible_from_ns_set(host, ns_set) {
                        r.push(ent.clone());
                    }
                }
                if let Some(c1) = c.extends_class(host) {
                    c = c1.defer()?;
                } else {
                    break;
                }
            }
            Ok(r)
        }

        pub override fn available_prototype_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            let mut r: Vec<Entity> = vec![];
            let mut c: Entity = self.clone().into();
            loop {
                for (name, ent) in c.prototype(host).borrow().iter() {
                    if name.accessible_from_ns_set(host, ns_set) {
                        r.push(ent.clone());
                    }
                }
                if let Some(c1) = c.extends_class(host) {
                    c = c1.defer()?;
                } else {
                    break;
                }
            }
            Ok(r)
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Entity>) {
            self.set_m_parent(p);
        }

        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<Asdoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.m_metadata()
        }

        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    /// An `interface` definition, possibly parameterized.
    pub struct InterfaceType: Type {
        let ref m_name: Option<QName> = None;
        let ref m_type_params: Option<SharedArray<Entity>> = None;
        let ref m_events: SharedMap<String, Event> = SharedMap::new();
        let ref m_extends_interfaces: SharedArray<Entity> = SharedArray::new();
        let ref m_known_implementors: SharedArray<Entity> = SharedArray::new();
        let ref m_parent: Option<Entity> = None;
        let ref m_prototype: Names = Names::new();
        let ref m_asdoc: Option<Rc<Asdoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_location: Option<Location> = None;
        let m_external: bool = false;

        pub(crate) fn InterfaceType(name: QName) {
            super();
            self.set_m_name(Some(name));
        }

        pub override fn is_interface_type_possibly_after_sub(&self) -> bool {
            true
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        pub override fn type_params(&self) -> Option<SharedArray<Entity>> {
            self.m_type_params()
        }

        pub override fn set_type_params(&self, list: Option<SharedArray<Entity>>) {
            self.set_m_type_params(list);
        }

        #[inheritdoc]
        pub override fn events(&self) -> SharedMap<String, Event> {
            self.m_events()
        }

        pub override fn known_implementors(&self) -> SharedArray<Entity> {
            self.m_known_implementors()
        }

        #[inheritdoc]
        pub override fn extends_interfaces(&self, host: &Database) -> SharedArray<Entity> {
            self.m_extends_interfaces()
        }

        pub override fn prototype(&self, host: &Database) -> Names {
            self.m_prototype()
        }

        pub override fn available_prototype_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            let mut r: Vec<Entity> = vec![];
            let mut typelist = self.all_ascending_types(host);
            typelist.push(self.clone().into());
            for itrfc in typelist.iter() {
                for (_, ent) in itrfc.prototype(host).borrow().iter() {
                    r.push(ent.clone());
                }
            }
            Ok(r)
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Entity>) {
            self.set_m_parent(p);
        }

        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<Asdoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.m_metadata()
        }

        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn is_external(&self) -> bool {
            self.m_external()
        }

        pub override fn set_is_external(&self, value: bool) {
            self.set_m_external(value);
        }

        override fn to_string_1(&self) -> String {
            let name_1 = self.fully_qualified_name();
            let mut p = String::new();
            if let Some(type_params) = self.type_params() {
                p = ".<".to_owned() + &type_params.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ") + ">";
            }
            name_1 + &p
        }
    }

    /// Type after substitution, whose origin is either
    /// a class or an interface. Other types, after substitution,
    /// such as structural types, are represented by their
    /// same type with substitution in compound parts.
    pub struct TypeAfterSubstitution: Type {
        let ref m_origin: Option<Entity> = None;
        let ref m_substitute_types: SharedArray<Entity> = SharedArray::new();
        let ref m_extends_class: Option<Entity> = None;
        let ref m_implements: Option<SharedArray<Entity>> = None;
        let ref m_extends_interfaces: Option<SharedArray<Entity>> = None;
        let ref m_properties: Option<Names> = None;
        let ref m_prototype: Option<Names> = None;
        let ref m_constructor_method: Option<Entity> = None;

        pub(crate) fn TypeAfterSubstitution(origin: Entity, substitute_types: SharedArray<Entity>) {
            super();
            self.set_m_origin(Some(origin));
            self.set_m_substitute_types(substitute_types);
        }

        pub override fn is_class_type_possibly_after_sub(&self) -> bool {
            self.origin().is_class_type_possibly_after_sub()
        }

        pub override fn is_interface_type_possibly_after_sub(&self) -> bool {
            self.origin().is_interface_type_possibly_after_sub()
        }

        pub override fn origin(&self) -> Entity {
            self.m_origin().unwrap()
        }

        pub override fn substitute_types(&self) -> SharedArray<Entity> {
            self.m_substitute_types()
        }
        
        pub override fn name(&self) -> QName {
            self.origin().name()
        }

        pub override fn events(&self) -> SharedMap<String, Event> {
            self.origin().events()
        }

        pub override fn is_abstract(&self) -> bool {
            self.origin().is_abstract()
        }

        pub override fn is_static(&self) -> bool {
            self.origin().is_static()
        }

        pub override fn is_final(&self) -> bool {
            self.origin().is_final()
        }

        pub override fn is_dynamic(&self) -> bool {
            self.origin().is_dynamic()
        }

        pub override fn is_dynamic_or_inherits_dynamic(&self, host: &Database) -> Result<bool, DeferError> {
            self.origin().is_dynamic_or_inherits_dynamic(host)
        }

        pub override fn is_options_class(&self) -> bool {
            self.origin().is_options_class()
        }

        #[inheritdoc]
        pub override fn extends_class(&self, host: &Database) -> Option<Entity> {
            if let Some(r) = self.m_extends_class() {
                return Some(r.clone());
            }
            let origin = self.origin();
            let r = origin.extends_class(host);
            if r.is_none() {
                return None;
            }
            let r = r.unwrap();
            if r.is::<UnresolvedEntity>() {
                return Some(r.clone());
            }
            let r = ApplyType(host).exec(&r, &origin.type_params().unwrap(), &self.m_substitute_types());
            self.set_m_extends_class(Some(r.clone()));
            Some(r)
        }

        #[inheritdoc]
        pub override fn implements(&self, host: &Database) -> SharedArray<Entity> {
            if let Some(r) = self.m_implements() {
                return r;
            }
            let origin = self.origin();
            let r: SharedArray<Entity> = origin.implements(host).iter().map(|t| ApplyType(host).exec(&t, &origin.type_params().unwrap(), &self.m_substitute_types())).collect();
            self.set_m_implements(Some(r.clone()));
            r
        }

        #[inheritdoc]
        pub override fn extends_interfaces(&self, host: &Database) -> SharedArray<Entity> {
            if let Some(r) = self.m_extends_interfaces() {
                return r;
            }
            let origin = self.origin();
            let r: SharedArray<Entity> = origin.extends_interfaces(host).iter().map(|t| ApplyType(host).exec(&t, &origin.type_params().unwrap(), &self.m_substitute_types())).collect();
            self.set_m_extends_interfaces(Some(r.clone()));
            r
        }

        pub override fn prototype(&self, host: &Database) -> Names {
            if let Some(r) = self.m_prototype() {
                return r;
            }
            let origin = self.origin();
            let mut r = Names::new();
            for (name, entity) in origin.prototype(host).borrow().iter() {
                let entity = ApplyType(host).exec(&entity, &origin.type_params().unwrap(), &self.m_substitute_types());
                r.set(name.clone(), entity)
            }
            self.set_m_prototype(Some(r.clone()));
            r
        }

        pub override fn properties(&self, host: &Database) -> Names {
            if let Some(r) = self.m_properties() {
                return r;
            }
            let origin = self.origin();
            let mut r = Names::new();
            for (name, entity) in origin.properties(host).borrow().iter() {
                let entity = ApplyType(host).exec(&entity, &origin.type_params().unwrap(), &self.m_substitute_types());
                r.set(name.clone(), entity)
            }
            self.set_m_properties(Some(r.clone()));
            r
        }

        pub override fn available_static_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            if self.is_interface_type_possibly_after_sub() {
                return Ok(vec![]);
            }
            let mut r: Vec<Entity> = vec![];
            let mut c: Entity = self.clone().into();
            loop {
                for (name, ent) in c.properties(host).borrow().iter() {
                    if name.accessible_from_ns_set(host, ns_set) {
                        r.push(ent.clone());
                    }
                }
                if let Some(c1) = c.extends_class(host) {
                    c = c1.defer()?;
                } else {
                    break;
                }
            }
            Ok(r)
        }

        pub override fn available_prototype_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            let mut r: Vec<Entity> = vec![];
            if self.is_interface_type_possibly_after_sub() {
                let mut typelist = self.all_ascending_types(host);
                typelist.push(self.clone().into());
                for itrfc in typelist.iter() {
                    for (_, ent) in itrfc.prototype(host).borrow().iter() {
                        r.push(ent.clone());
                    }
                }
                return Ok(r);
            }
            let mut c: Entity = self.clone().into();
            loop {
                for (name, ent) in c.prototype(host).borrow().iter() {
                    if name.accessible_from_ns_set(host, ns_set) {
                        r.push(ent.clone());
                    }
                }
                if let Some(c1) = c.extends_class(host) {
                    c = c1.defer()?;
                } else {
                    break;
                }
            }
            Ok(r)
        }

        pub override fn constructor_method(&self, host: &Database) -> Option<Entity> {
            if let Some(r) = self.m_constructor_method() {
                return Some(r.clone());
            }
            let origin = self.origin();
            let r = origin.constructor_method(host);
            if r.is_none() {
                return None;
            }
            let r = r.unwrap();
            let r = ApplyType(host).exec(&r, &origin.type_params().unwrap(), &self.m_substitute_types());
            self.set_m_constructor_method(Some(r.clone()));
            Some(r)
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.origin().parent()
        }

        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.origin().asdoc()
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.origin().metadata()
        }

        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn location(&self) -> Option<Location> {
            None
        }

        override fn to_string_1(&self) -> String {
            let name_1 = self.fully_qualified_name();
            let a = self.m_substitute_types();
            let p = ".<".to_owned() + &a.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", ") + ">";
            name_1 + &p
        }
    }

    /// Tuple type. The tuple type is equivalent to
    /// `Array` with type safety for its element types.
    pub struct TupleType: Type {
        let ref m_elements: SharedArray<Entity> = SharedArray::new();

        pub(crate) fn TupleType(elements: SharedArray<Entity>) {
            super();
            self.set_m_elements(elements);
        }
        
        pub override fn element_types(&self) -> SharedArray<Entity> {
            self.m_elements()
        }

        pub override fn is_abstract(&self) -> bool {
            false
        }

        pub override fn is_final(&self) -> bool {
            true
        }

        pub override fn is_dynamic(&self) -> bool {
            true
        }

        pub override fn is_dynamic_or_inherits_dynamic(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(self.is_dynamic())
        }

        pub override fn is_options_class(&self) -> bool {
            false
        }

        #[inheritdoc]
        pub override fn extends_class(&self, host: &Database) -> Option<Entity> {
            Some(host.array_type_of_any().unwrap_or(host.unresolved_entity()))
        }

        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        pub override fn available_static_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            self.extends_class(host).map(|c| c.available_static_definitions(host, ns_set)).unwrap_or(Ok(vec![]))
        }

        pub override fn available_prototype_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            self.extends_class(host).map(|c| c.available_prototype_definitions(host, ns_set)).unwrap_or(Ok(vec![]))
        }

        override fn to_string_1(&self) -> String {
            format!("[{}]", self.element_types().iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", "))
        }
    }

    /// Structural function type. This type is equivalent to `Function`
    /// with type safety.
    pub struct FunctionType: Type {
        let ref m_params: SharedArray<Rc<SemanticFunctionTypeParameter>> = SharedArray::new();
        let ref m_result_type: Option<Entity> = None;

        pub(crate) fn FunctionType(params: SharedArray<Rc<SemanticFunctionTypeParameter>>, result_type: Entity) {
            super();
            self.set_m_params(params);
            self.set_m_result_type(Some(result_type));
        }
        
        pub override fn params(&self) -> SharedArray<Rc<SemanticFunctionTypeParameter>> {
            self.m_params()
        }

        pub override fn result_type(&self) -> Entity {
            self.m_result_type().unwrap()
        }

        pub override fn is_abstract(&self) -> bool {
            false
        }

        pub override fn is_final(&self) -> bool {
            true
        }

        pub override fn is_dynamic(&self) -> bool {
            false
        }

        pub override fn is_dynamic_or_inherits_dynamic(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(self.is_dynamic())
        }

        pub override fn is_options_class(&self) -> bool {
            false
        }

        pub override fn available_static_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            self.extends_class(host).map(|c| c.available_static_definitions(host, ns_set)).unwrap_or(Ok(vec![]))
        }

        pub override fn available_prototype_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            self.extends_class(host).map(|c| c.available_prototype_definitions(host, ns_set)).unwrap_or(Ok(vec![]))
        }

        #[inheritdoc]
        pub override fn extends_class(&self, host: &Database) -> Option<Entity> {
            Some(host.function_type())
        }

        pub override fn includes_undefined(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self, host: &Database) -> Result<bool, DeferError> {
            Ok(true)
        }

        override fn to_string_1(&self) -> String {
            let mut p = Vec::<String>::new();
            for p1 in self.params().iter() {
                match p1.kind {
                    ParameterKind::Required => {
                        p.push(p1.static_type.to_string());
                    },
                    ParameterKind::Optional => {
                        p.push(p1.static_type.to_string() + &"=".to_owned());
                    },
                    ParameterKind::Rest => {
                        p.push("...".to_owned() + &p1.static_type.to_string());
                    },
                }
            }
            format!("function({}) : {}", p.join(", "), self.result_type().to_string())
        }
    }

    /// The nullable type `T?`. It is equivalent to either
    /// `T` or `*` (for all primitive types but String).
    pub struct NullableType: Type {
        let ref m_base: Option<Entity> = None;

        pub(crate) fn NullableType(base: Entity) {
            super();
            self.set_m_base(Some(base));
        }

        /// The type that is made nullable.
        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn includes_undefined(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self) -> Result<bool, DeferError> {
            Ok(true)
        }

        #[inheritdoc]
        pub override fn escape_of_nullable(&self) -> Entity {
            self.base()
        }

        pub override fn escape_of_nullable_or_non_nullable(&self) -> Entity {
            self.base()
        }

        pub override fn available_static_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            self.base().available_static_definitions(host, ns_set)
        }

        pub override fn available_prototype_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            self.base().available_prototype_definitions(host, ns_set)
        }

        override fn to_string_1(&self) -> String {
            if let Ok(ft) = self.base().to::<FunctionType>() {
                format!("?{}", ft.to_string())
            } else {
                format!("{}?", self.base().to_string())
            }
        }
    }

    /// The non-nullable type `T!`.
    pub struct NonNullableType: Type {
        let ref m_base: Option<Entity> = None;

        pub(crate) fn NonNullableType(base: Entity) {
            super();
            self.set_m_base(Some(base));
        }

        /// The type that is made non-nullable.
        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn includes_undefined(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        #[inheritdoc]
        pub override fn escape_of_non_nullable(&self) -> Entity {
            self.base()
        }

        pub override fn escape_of_nullable_or_non_nullable(&self) -> Entity {
            self.base()
        }

        pub override fn available_static_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            self.base().available_static_definitions(host, ns_set)
        }

        pub override fn available_prototype_definitions(&self, host: &Database, ns_set: &SharedArray<Entity>) -> Result<Vec<Entity>, DeferError> {
            self.base().available_prototype_definitions(host, ns_set)
        }

        override fn to_string_1(&self) -> String {
            if let Ok(ft) = self.base().to::<FunctionType>() {
                format!("({})!", ft.to_string())
            } else {
                format!("{}!", self.base().to_string())
            }
        }
    }

    /// A type corresponding to a type parameter from a class or interface.
    pub struct TypeParameterType: Type {
        let ref m_name: Option<QName> = None;
        let ref m_location: Option<Location> = None;

        pub(crate) fn TypeParameterType(name: QName) {
            super();
            self.set_m_name(Some(name));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        pub override fn includes_undefined(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        pub override fn includes_null(&self) -> Result<bool, DeferError> {
            Ok(false)
        }

        override fn to_string_1(&self) -> String {
            self.name().to_string()
        }
    }

    /// Either an **original** variable slot, or a variable slot after substitution.
    pub struct VariableSlot: Entity {
        fn VariableSlot() {
            super();
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &Database) -> Entity {
            self.static_type(host)
        }
    }

    pub struct OriginalVariableSlot: VariableSlot {
        let ref m_name: Option<QName> = None;
        let ref m_location: Option<Location> = None;
        let ref m_asdoc: Option<Rc<Asdoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_constant: Option<Entity> = None;
        let ref m_static_type: Option<Entity> = None;
        let ref m_parent: Option<Entity> = None;
        let m_flags: VariableSlotFlags = VariableSlotFlags::empty();
        let ref m_bindable_event: Option<String> = None;

        pub(crate) fn OriginalVariableSlot(name: &QName, read_only: bool, static_type: &Entity) {
            super();
            self.set_m_name(Some(name.clone()));
            self.set_read_only(read_only);
            self.set_m_static_type(Some(static_type.clone()));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        /// The constant initially assigned to that variable slot.
        pub override fn var_constant(&self) -> Option<Entity> {
            self.m_constant()
        }

        /// The constant initially assigned to that variable slot.
        pub override fn set_var_constant(&self, k: Option<Entity>) {
            self.set_m_constant(k);
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            self.m_flags().contains(VariableSlotFlags::READ_ONLY)
        }

        pub override fn set_read_only(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(VariableSlotFlags::READ_ONLY, value);
            self.set_m_flags(v);
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn static_type(&self, host: &Database) -> Entity {
            self.m_static_type().unwrap()
        }

        pub override fn set_static_type(&self, value: Entity) {
            self.set_m_static_type(Some(value));
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        /// The event name indicated by a `[Bindable]` meta-data tag.
        pub override fn bindable_event(&self) -> Option<String> {
            self.m_bindable_event()
        }

        pub override fn set_bindable_event(&self, name: Option<String>) {
            self.set_m_bindable_event(name);
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Entity>) {
            self.set_m_parent(p);
        }

        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<Asdoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.m_metadata()
        }

        pub override fn is_external(&self) -> bool {
            self.m_flags().contains(VariableSlotFlags::IS_EXTERNAL)
        }

        pub override fn set_is_external(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(VariableSlotFlags::IS_EXTERNAL, value);
            self.set_m_flags(v);
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    /// Variable slot after indirect substitution.
    pub struct VariableSlotAfterSubstitution: VariableSlot {
        let ref m_origin: Option<Entity> = None;
        let ref m_indirect_type_params: SharedArray<Entity> = SharedArray::new();
        let ref m_indirect_substitute_types: SharedArray<Entity> = SharedArray::new();
        let ref m_static_type: Option<Entity> = None;

        pub(crate) fn VariableSlotAfterSubstitution(origin: &Entity, indirect_type_params: &SharedArray<Entity>, indirect_substitute_types: &SharedArray<Entity>) {
            super();
            self.set_m_origin(Some(origin.clone()));
            self.set_m_indirect_type_params(indirect_type_params.clone());
            self.set_m_indirect_substitute_types(indirect_substitute_types.clone());
        }

        pub override fn origin(&self) -> Entity {
            self.m_origin().unwrap()
        }

        pub override fn indirect_type_params(&self) -> SharedArray<Entity> {
            self.m_indirect_type_params()
        }

        pub override fn indirect_substitute_types(&self) -> SharedArray<Entity> {
            self.m_indirect_substitute_types()
        }

        pub override fn name(&self) -> QName {
            self.origin().name()
        }

        /// The constant initially assigned to that variable slot.
        pub override fn var_constant(&self) -> Option<Entity> {
            None
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            self.origin().read_only(host)
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn static_type(&self, host: &Database) -> Entity {
            if let Some(r) = self.m_static_type() {
                return r.clone();
            }
            let r = self.origin().static_type(host);
            if r.is::<UnresolvedEntity>() {
                return r.clone();
            }
            let r = ApplyType(host).exec(&r, &self.m_indirect_type_params(), &self.m_indirect_substitute_types());
            self.set_m_static_type(Some(r.clone()));
            r
        }

        pub override fn location(&self) -> Option<Location> {
            None
        }

        /// The event name indicated by a `[Bindable]` meta-data tag.
        pub override fn bindable_event(&self) -> Option<String> {
            self.origin().bindable_event()
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.origin().parent()
        }

        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.origin().asdoc()
        }

        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.origin().metadata()
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    /// Either an **original** virtual slot, or a virtual slot after substitution.
    pub struct VirtualSlot: Entity {
        fn VirtualSlot() {
            super();
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &Database) -> Entity {
            self.static_type(host)
        }
    }

    pub struct OriginalVirtualSlot: VirtualSlot {
        let ref m_name: Option<QName> = None;
        let ref m_location: Option<Location> = None;
        let ref m_asdoc: Option<Rc<Asdoc>> = None;
        let ref m_getter: Option<Entity> = None;
        let ref m_setter: Option<Entity> = None;
        let ref m_static_type: Option<Entity> = None;
        let ref m_parent: Option<Entity> = None;
        let m_flags: VirtualSlotFlags = VirtualSlotFlags::empty();
        let ref m_bindable_event: Option<String> = None;

        pub(crate) fn OriginalVirtualSlot(name: &QName) {
            super();
            self.set_m_name(Some(name.clone()));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn getter(&self, host: &Database) -> Option<Entity> {
            self.m_getter()
        }

        pub override fn set_getter(&self, m: Option<Entity>) {
            self.set_m_getter(m);
        }

        pub override fn setter(&self, host: &Database) -> Option<Entity> {
            self.m_setter()
        }

        pub override fn set_setter(&self, m: Option<Entity>) {
            self.set_m_setter(m);
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            self.setter(host).is_none()
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            self.getter(host).is_none()
        }

        pub override fn static_type(&self, host: &Database) -> Entity {
            if let Some(r) = self.m_static_type() {
                return r.clone();
            }

            let mut deduced_type: Option<Entity> = None;

            // Deduce type from getter
            if let Some(getter) = self.m_getter() {
                let signature: Entity = getter.signature(host);
                if !signature.is::<UnresolvedEntity>() {
                    deduced_type = Some(signature.result_type());
                }
            }

            // Deduce type from setter
            if let Some(setter) = self.m_setter() {
                let signature: Entity = setter.signature(host);
                if !signature.is::<UnresolvedEntity>() {
                    deduced_type = Some(signature.params().get(0).unwrap().static_type.clone());
                }
            }

            if deduced_type.is_none() {
                return host.unresolved_entity();
            }

            self.set_m_static_type(deduced_type.clone());
            deduced_type.unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        /// The event name indicated by a `[Bindable]` meta-data tag.
        pub override fn bindable_event(&self) -> Option<String> {
            self.m_bindable_event()
        }

        pub override fn set_bindable_event(&self, name: Option<String>) {
            self.set_m_bindable_event(name);
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Entity>) {
            self.set_m_parent(p);
        }

        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.m_asdoc()
        }

        pub override fn set_asdoc(&self, asdoc: Option<Rc<Asdoc>>) {
            self.set_m_asdoc(asdoc);
        }

        pub override fn is_external(&self) -> bool {
            self.m_flags().contains(VirtualSlotFlags::IS_EXTERNAL)
        }

        pub override fn set_is_external(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(VirtualSlotFlags::IS_EXTERNAL, value);
            self.set_m_flags(v);
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    pub struct VirtualSlotAfterSubstitution: VirtualSlot {
        let ref m_origin: Option<Entity> = None;
        let ref m_indirect_type_params: SharedArray<Entity> = SharedArray::new();
        let ref m_indirect_substitute_types: SharedArray<Entity> = SharedArray::new();
        let ref m_getter: Option<Entity> = None;
        let ref m_setter: Option<Entity> = None;
        let ref m_static_type: Option<Entity> = None;

        pub(crate) fn VirtualSlotAfterSubstitution(origin: &Entity, indirect_type_params: &SharedArray<Entity>, indirect_substitute_types: &SharedArray<Entity>) {
            super();
            self.set_m_origin(Some(origin.clone()));
            self.set_m_indirect_type_params(indirect_type_params.clone());
            self.set_m_indirect_substitute_types(indirect_substitute_types.clone());
        }

        pub override fn location(&self) -> Option<Location> {
            None
        }

        pub override fn origin(&self) -> Entity {
            self.m_origin().unwrap()
        }

        pub override fn indirect_type_params(&self) -> SharedArray<Entity> {
            self.m_indirect_type_params()
        }

        pub override fn indirect_substitute_types(&self) -> SharedArray<Entity> {
            self.m_indirect_substitute_types()
        }

        pub override fn name(&self) -> QName {
            self.origin().name()
        }

        pub override fn getter(&self, host: &Database) -> Option<Entity> {
            if let Some(r) = self.m_getter() {
                return Some(r);
            }
            let r = self.origin().getter(host);
            if r.is_none() {
                return r;
            }
            let r = ApplyType(host).exec(&r.unwrap(), &self.indirect_type_params(), &self.indirect_substitute_types());
            self.set_m_getter(Some(r.clone()));
            Some(r)
        }

        pub override fn setter(&self, host: &Database) -> Option<Entity> {
            if let Some(r) = self.m_setter() {
                return Some(r);
            }
            let r = self.origin().setter(host);
            if r.is_none() {
                return r;
            }
            let r = ApplyType(host).exec(&r.unwrap(), &self.indirect_type_params(), &self.indirect_substitute_types());
            self.set_m_setter(Some(r.clone()));
            Some(r)
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            self.origin().read_only(host)
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            self.origin().write_only(host)
        }

        pub override fn static_type(&self, host: &Database) -> Entity {
            if let Some(r) = self.m_static_type() {
                return r;
            }
            let r = self.origin().static_type(host);
            if r.is::<UnresolvedEntity>() {
                return r;
            }
            let r = ApplyType(host).exec(&r, &self.indirect_type_params(), &self.indirect_substitute_types());
            self.set_m_static_type(Some(r.clone()));
            r
        }

        pub override fn bindable_event(&self) -> Option<String> {
            self.origin().bindable_event()
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.origin().parent()
        }

        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.origin().asdoc()
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    /// Either an **original** method slot, or a method slot after substitution.
    pub struct MethodSlot: Entity {
        fn MethodSlot() {
            super();
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &Database) -> Entity {
            host.function_type()
        }
    }

    pub struct OriginalMethodSlot: MethodSlot {
        let ref m_name: Option<QName> = None;
        let ref m_location: Option<Location> = None;
        let ref m_asdoc: Option<Rc<Asdoc>> = None;
        let ref m_metadata: SharedArray<Rc<Metadata>> = SharedArray::new();
        let ref m_activation: Option<Entity> = None;
        let ref m_signature: Option<Entity> = None;
        let ref m_parent: Option<Entity> = None;
        let ref m_of_virtual_slot: Option<Entity> = None;
        let ref m_overriden_by: SharedArray<Entity> = SharedArray::new();
        let ref m_overrides_method: Option<Entity> = None;
        let m_flags: MethodSlotFlags = MethodSlotFlags::empty();

        pub(crate) fn OriginalMethodSlot(name: &QName, signature: &Entity) {
            super();
            self.set_m_name(Some(name.clone()));
            self.set_m_signature(Some(signature.clone()));
        }

        pub override fn name(&self) -> QName {
            self.m_name().unwrap()
        }

        pub override fn is_final(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_FINAL)
        }
    
        pub override fn set_is_final(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_FINAL, value);
            self.set_m_flags(v);
        }

        pub override fn is_static(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_STATIC)
        }
    
        pub override fn set_is_static(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_STATIC, value);
            self.set_m_flags(v);
        }

        pub override fn is_abstract(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_ABSTRACT)
        }
    
        pub override fn set_is_abstract(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_ABSTRACT, value);
            self.set_m_flags(v);
        }

        pub override fn is_native(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_NATIVE)
        }
    
        pub override fn set_is_native(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_NATIVE, value);
            self.set_m_flags(v);
        }

        pub override fn is_overriding(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_OVERRIDING)
        }
    
        pub override fn set_is_overriding(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_OVERRIDING, value);
            self.set_m_flags(v);
        }

        pub override fn is_async(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_ASYNC)
        }
    
        pub override fn set_is_async(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_ASYNC, value);
            self.set_m_flags(v);
        }

        pub override fn is_generator(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_GENERATOR)
        }
    
        pub override fn set_is_generator(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_GENERATOR, value);
            self.set_m_flags(v);
        }

        pub override fn is_constructor(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_CONSTRUCTOR)
        }
    
        pub override fn set_is_constructor(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_CONSTRUCTOR, value);
            self.set_m_flags(v);
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }
    
        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }
    
        pub override fn set_parent(&self, p: Option<Entity>) {
            self.set_m_parent(p);
        }
    
        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.m_asdoc()
        }
    
        pub override fn set_asdoc(&self, asdoc: Option<Rc<Asdoc>>) {
            self.set_m_asdoc(asdoc);
        }
    
        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.m_metadata()
        }

        pub override fn signature(&self, host: &Database) -> Entity {
            self.m_signature().unwrap()
        }

        pub override fn set_signature(&self, signature: &Entity) {
            self.set_m_signature(Some(signature.clone()));
        }

        pub override fn activation(&self) -> Option<Entity> {
            self.m_activation()
        }

        pub override fn set_activation(&self, activation: Option<Entity>) {
            self.set_m_activation(activation);
        }

        pub override fn of_virtual_slot(&self, host: &Database) -> Option<Entity> {
            self.m_of_virtual_slot()
        }

        pub override fn set_of_virtual_slot(&self, virtual_slot: Option<Entity>) {
            self.set_m_of_virtual_slot(virtual_slot);
        }

        pub override fn overriden_by(&self, host: &Database) -> SharedArray<Entity> {
            self.m_overriden_by()
        }

        pub override fn overrides_method(&self, host: &Database) -> Option<Entity> {
            self.m_overrides_method()
        }

        pub override fn set_overrides_method(&self, method: Option<Entity>) {
            self.set_m_overrides_method(method);
        }

        pub override fn is_external(&self) -> bool {
            self.m_flags().contains(MethodSlotFlags::IS_EXTERNAL)
        }

        pub override fn set_is_external(&self, value: bool) {
            let mut v = self.m_flags();
            v.set(MethodSlotFlags::IS_EXTERNAL, value);
            self.set_m_flags(v);
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    pub struct MethodSlotAfterSubstitution: MethodSlot {
        let ref m_origin: Option<Entity> = None;
        let ref m_indirect_type_params: SharedArray<Entity> = SharedArray::new();
        let ref m_indirect_substitute_types: SharedArray<Entity> = SharedArray::new();
        let ref m_signature: Option<Entity> = None;
        let ref m_of_virtual_slot: Option<Entity> = None;
        let ref m_overriden_by: Option<SharedArray<Entity>> = None;
        let ref m_overrides_method: Option<Entity> = None;
        let m_is_overriding: bool = false;

        pub fn MethodSlotAfterSubstitution(origin: &Entity, indirect_type_params: &SharedArray<Entity>, indirect_substitute_types: &SharedArray<Entity>) {
            super();
            self.set_m_origin(Some(origin.clone()));
            self.set_m_indirect_type_params(indirect_type_params.clone());
            self.set_m_indirect_substitute_types(indirect_substitute_types.clone());
        }

        pub override fn origin(&self) -> Entity {
            self.m_origin().unwrap()
        }

        pub override fn indirect_type_params(&self) -> SharedArray<Entity> {
            self.m_indirect_type_params()
        }

        pub override fn indirect_substitute_types(&self) -> SharedArray<Entity> {
            self.m_indirect_substitute_types()
        }

        pub override fn name(&self) -> QName {
            self.origin().name()
        }

        pub override fn is_final(&self) -> bool {
            self.origin().is_final()
        }

        pub override fn is_static(&self) -> bool {
            self.origin().is_static()
        }

        pub override fn is_abstract(&self) -> bool {
            self.origin().is_abstract()
        }

        pub override fn is_native(&self) -> bool {
            self.origin().is_native()
        }

        pub override fn is_overriding(&self) -> bool {
            self.m_is_overriding()
        }

        pub override fn set_is_overriding(&self, value: bool) {
            self.set_m_is_overriding(value);
        }

        pub override fn is_async(&self) -> bool {
            self.origin().is_async()
        }

        pub override fn is_generator(&self) -> bool {
            self.origin().is_generator()
        }

        pub override fn is_constructor(&self) -> bool {
            self.origin().is_constructor()
        }

        pub override fn location(&self) -> Option<Location> {
            None
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.origin().parent()
        }
    
        pub override fn asdoc(&self) -> Option<Rc<Asdoc>> {
            self.origin().asdoc()
        }
    
        pub override fn metadata(&self) -> SharedArray<Rc<Metadata>> {
            self.origin().metadata()
        }

        pub override fn signature(&self, host: &Database) -> Entity {
            if let Some(r) = self.m_signature() {
                return r;
            }
            let r = self.origin().signature(host);
            if r.is::<UnresolvedEntity>() {
                return r.clone();
            }
            let r = ApplyType(host).exec(&r, &self.m_indirect_type_params(), &self.m_indirect_substitute_types());
            self.set_m_signature(Some(r.clone()));
            r
        }

        pub override fn of_virtual_slot(&self, host: &Database) -> Option<Entity> {
            if let Some(r) = self.m_of_virtual_slot() {
                return Some(r);
            }
            let r = self.origin().of_virtual_slot(host);
            if r.is_none() {
                return None;
            }
            let r = ApplyType(host).exec(&r.unwrap(), &self.m_indirect_type_params(), &self.m_indirect_substitute_types());
            self.set_m_of_virtual_slot(Some(r.clone()));
            Some(r)
        }

        pub override fn overriden_by(&self, host: &Database) -> SharedArray<Entity> {
            if let Some(r) = self.m_overriden_by() {
                return r;
            }
            let r = self.origin().overriden_by(host);
            let r: SharedArray<Entity> = r.iter().map(|r| ApplyType(host).exec(&r, &self.m_indirect_type_params(), &self.indirect_substitute_types())).collect();
            self.set_m_overriden_by(Some(r.clone()));
            r
        }

        pub override fn overrides_method(&self, host: &Database) -> Option<Entity> {
            if let Some(r) = self.m_overrides_method() {
                return Some(r);
            }
            let r = self.origin().overrides_method(host);
            if r.is_none() {
                return None;
            }
            let r = ApplyType(host).exec(&r.unwrap(), &self.m_indirect_type_params(), &self.m_indirect_substitute_types());
            self.set_m_overrides_method(Some(r.clone()));
            Some(r)
        }

        pub override fn set_overrides_method(&self, method: Option<Entity>) {
            self.set_m_overrides_method(method);
        }

        override fn to_string_1(&self) -> String {
            self.fully_qualified_name()
        }
    }

    pub struct Scope: Entity {
        let ref m_parent: Option<Entity> = None;
        let ref m_properties: Names = Names::new();
        let ref m_open_ns_set: SharedArray<Entity> = SharedArray::new();
        let ref m_import_list: SharedArray<Entity> = SharedArray::new();

        pub(crate) fn Scope() {
            super();
        }

        pub override fn parent(&self) -> Option<Entity> {
            self.m_parent()
        }

        pub override fn set_parent(&self, p: Option<Entity>) {
            self.set_m_parent(p);
        }

        pub override fn properties(&self, host: &Database) -> Names {
            self.m_properties()
        }

        pub override fn open_ns_set(&self) -> SharedArray<Entity> {
            self.m_open_ns_set()
        }

        /// List of [`PackagePropertyImport`], [`PackageWildcardImport`], or [`PackageRecursiveImport`].
        pub override fn import_list(&self) -> SharedArray<Entity> {
            self.m_import_list()
        }
    }

    pub struct WithScope: Scope {
        let ref m_object: Option<Entity> = None;

        pub(crate) fn WithScope(object: &Entity) {
            super();
            self.set_m_object(Some(object.clone()));
        }

        pub override fn object(&self) -> Entity {
            self.m_object().unwrap()
        }
    }

    pub struct FilterScope: Scope {
        let ref m_base: Option<Entity> = None;

        pub(crate) fn FilterScope(base: &Entity) {
            super();
            self.set_m_base(Some(base.clone()));
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }
    }

    pub struct Activation: Scope {
        let m_kind: u8 = DEFAULT_ACTIVATION;
        let ref m_method: Option<Entity> = None;
        let ref m_this: Option<Entity> = None;
        let ref m_property_has_capture: Option<SharedArray<Entity>> = None;
        let ref m_cfg: ControlFlowGraph = ControlFlowGraph::new();
        let ref m_public_ns: Option<Entity> = None;
        let ref m_internal_ns: Option<Entity> = None;

        pub(crate) fn Activation(of_method: &Entity) {
            super();
            self.set_m_method(Some(of_method.clone()));
        }

        pub override fn of_method(&self) -> Entity {
            self.m_method().unwrap()
        }

        /// An optional `ThisObject` value.
        pub override fn this(&self) -> Option<Entity> {
            self.m_this()
        }

        /// Sets a `ThisObject` value.
        pub override fn set_this(&self, this: Option<Entity>) {
            self.set_m_this(this);
        }

        /// Indicates whether an activation's property has been captured
        /// by a subsequent activation. Properties include, for example, the range from the
        /// activation to an inner most block scope.
        pub override fn property_has_capture(&self, property: &Entity) -> bool {
            if let Some(set) = self.m_property_has_capture() {
                set.includes(property)
            } else {
                false
            }
        }

        pub override fn set_property_has_capture(&self, property: &Entity, value: bool) {
            if let Some(mut set) = self.m_property_has_capture() {
                if value {
                    if !set.includes(property) {
                        set.push(property.clone());
                    }
                } else {
                    let i = set.index_of(property);
                    if let Some(i) = i {
                        set.remove(i);
                    }
                }
            } else if value {
                self.set_m_property_has_capture(Some(shared_array![property.clone()]));
            }
        }

        pub override fn control_flow_graph(&self) -> ControlFlowGraph {
            self.m_cfg()
        }

        pub override fn is_global_initialization(&self) -> bool {
            self.m_kind() == GLOBAL_INIT_ACTIVATION
        }

        pub override fn set_is_global_initialization(&self, value: bool) {
            self.set_m_kind(if value { GLOBAL_INIT_ACTIVATION } else { DEFAULT_ACTIVATION });
        }

        pub override fn is_package_initialization(&self) -> bool {
            self.m_kind() == PACKAGE_INIT_ACTIVATION
        }

        pub override fn set_is_package_initialization(&self, value: bool) {
            self.set_m_kind(if value { PACKAGE_INIT_ACTIVATION } else { DEFAULT_ACTIVATION });
        }

        pub override fn public_ns(&self) -> Option<Entity> {
            self.m_public_ns()
        }

        pub override fn set_public_ns(&self, ns: Option<Entity>) {
            self.set_m_public_ns(ns);
        }

        pub override fn internal_ns(&self) -> Option<Entity> {
            self.m_internal_ns()
        }

        pub override fn set_internal_ns(&self, ns: Option<Entity>) {
            self.set_m_internal_ns(ns);
        }
    }

    pub struct FixtureScope: Scope {
        pub(crate) fn FixtureScope() {
            super();
        }
    }

    pub struct ClassScope: FixtureScope {
        let ref m_class: Option<Entity> = None;

        pub(crate) fn ClassScope(class: &Entity) {
            super();
            self.set_m_class(Some(class.clone()));
        }

        pub override fn class(&self) -> Entity {
            self.m_class().unwrap()
        }
    }

    pub struct EnumScope: FixtureScope {
        let ref m_class: Option<Entity> = None;

        pub(crate) fn EnumScope(class: &Entity) {
            super();
            self.set_m_class(Some(class.clone()));
        }

        pub override fn class(&self) -> Entity {
            self.m_class().unwrap()
        }
    }

    pub struct InterfaceScope: FixtureScope {
        let ref m_itrfc: Option<Entity> = None;

        pub(crate) fn InterfaceScope(itrfc: &Entity) {
            super();
            self.set_m_itrfc(Some(itrfc.clone()));
        }

        pub override fn interface(&self) -> Entity {
            self.m_itrfc().unwrap()
        }
    }

    pub struct PackageScope: FixtureScope {
        let ref m_pckg: Option<Entity> = None;

        pub(crate) fn PackageScope(pckg: &Entity) {
            super();
            self.set_m_pckg(Some(pckg.clone()));
        }

        pub override fn package(&self) -> Entity {
            self.m_pckg().unwrap()
        }
    }

    pub struct Value: Entity {
        let ref m_static_type: Option<Entity> = None;

        pub(crate) fn Value(static_type: &Entity) {
            super();
            self.set_m_static_type(Some(static_type.clone()));
        }

        pub override fn static_type(&self, host: &Database) -> Entity {
            self.m_static_type().unwrap()
        }

        pub override fn set_static_type(&self, value: Entity) {
            self.set_m_static_type(Some(value));
        }

        #[inheritdoc]
        pub override fn property_static_type(&self, host: &Database) -> Entity {
            self.static_type(host)
        }
    }

    pub struct PackagePropertyImport: Value {
        let ref m_property: Option<Entity> = None;
        let ref m_location: Option<Location> = None;

        pub(crate) fn PackagePropertyImport(property: &Entity, location: Option<Location>, static_type: &Entity) {
            super(static_type);
            self.set_m_property(Some(property.clone()));
            self.set_m_location(location);
        }

        pub override fn property(&self) -> Entity {
            self.m_property().unwrap()
        }

        pub override fn set_property(&self, value: &Entity) {
            self.set_m_property(Some(value.clone()));
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }
    }

    pub struct PackageWildcardImport: Value {
        let ref m_package: Option<Entity> = None;
        let ref m_location: Option<Location> = None;

        pub(crate) fn PackageWildcardImport(package: &Entity, location: Option<Location>, static_type: &Entity) {
            super(static_type);
            self.set_m_package(Some(package.clone()));
            self.set_m_location(location);
        }

        pub override fn package(&self) -> Entity {
            self.m_package().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }
    }

    pub struct PackageRecursiveImport: Value {
        let ref m_package: Option<Entity> = None;
        let ref m_location: Option<Location> = None;

        pub(crate) fn PackageRecursiveImport(package: &Entity, location: Option<Location>, static_type: &Entity) {
            super(static_type);
            self.set_m_package(Some(package.clone()));
            self.set_m_location(location);
        }

        pub override fn package(&self) -> Entity {
            self.m_package().unwrap()
        }

        pub override fn location(&self) -> Option<Location> {
            self.m_location()
        }

        pub override fn set_location(&self, loc: Option<Location>) {
            self.set_m_location(loc);
        }
    }

    pub struct Constant: Value {
        pub(crate) fn Constant(static_type: &Entity) {
            super(static_type);
        }
    }

    /// Constant with possible types being `*` or `Object`.
    pub struct UndefinedConstant: Constant {
        pub(crate) fn UndefinedConstant(static_type: &Entity) {
            super(static_type);
        }

        pub override fn clone_constant(&self, host: &Database) -> Entity {
            host.factory().create_undefined_constant(&self.static_type(host))
        }
    }

    pub struct NullConstant: Constant {
        pub(crate) fn NullConstant(static_type: &Entity) {
            super(static_type);
        }

        pub override fn clone_constant(&self, host: &Database) -> Entity {
            host.factory().create_null_constant(&self.static_type(host))
        }
    }

    pub struct NamespaceConstant: Constant {
        let ref m_ns: Option<Entity> = None;

        pub(crate) fn NamespaceConstant(referenced_ns: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_ns(Some(referenced_ns.clone()));
        }

        pub override fn referenced_ns(&self) -> Entity {
            self.m_ns().unwrap()
        }

        pub override fn is_namespace_or_ns_constant(&self) -> bool {
            true
        }

        pub override fn clone_constant(&self, host: &Database) -> Entity {
            host.factory().create_namespace_constant_with_static_type(&self.referenced_ns(), &self.static_type(host))
        }
    }

    pub struct TypeConstant: Constant {
        let ref m_type: Option<Entity> = None;

        pub(crate) fn TypeConstant(referenced_type: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_type(Some(referenced_type.clone()));
        }

        pub override fn referenced_type(&self) -> Entity {
            self.m_type().unwrap()
        }

        pub override fn clone_constant(&self, host: &Database) -> Entity {
            host.factory().create_type_constant_with_static_type(&self.referenced_type(), &self.static_type(host))
        }
    }

    pub struct NumberConstant: Constant {
        let ref m_value: Number = Number::Int(0);

        pub(crate) fn NumberConstant(value: Number, static_type: &Entity) {
            super(static_type);
            self.set_m_value(value);
        }

        pub override fn number_value(&self) -> Number {
            self.m_value()
        }

        pub override fn clone_constant(&self, host: &Database) -> Entity {
            host.factory().create_number_constant(self.m_value(), &self.static_type(host))
        }
    }

    pub struct StringConstant: Constant {
        let ref m_value: String = String::new();

        pub(crate) fn StringConstant(value: String, static_type: &Entity) {
            super(static_type);
            self.set_m_value(value);
        }

        pub override fn string_value(&self) -> String {
            self.m_value()
        }

        pub override fn clone_constant(&self, host: &Database) -> Entity {
            host.factory().create_string_constant(self.m_value(), &self.static_type(host))
        }
    }

    pub struct BooleanConstant: Constant {
        let m_value: bool = true;

        pub(crate) fn BooleanConstant(value: bool, static_type: &Entity) {
            super(static_type);
            self.set_m_value(value);
        }

        pub override fn boolean_value(&self) -> bool {
            self.m_value()
        }

        pub override fn clone_constant(&self, host: &Database) -> Entity {
            host.factory().create_boolean_constant(self.m_value(), &self.static_type(host))
        }
    }

    pub struct ThisObject: Value {
        pub(crate) fn ThisObject(static_type: &Entity) {
            super(static_type);
        }
    }

    /// The `import.meta` value.
    pub struct MetaProperty: Value {
        pub(crate) fn MetaProperty(static_type: &Entity) {
            super(static_type);
        }
    }

    /// The `import.meta.env` value.
    pub struct MetaEnvProperty: Value {
        pub(crate) fn MetaEnvProperty(static_type: &Entity) {
            super(static_type);
        }
    }

    pub struct ReferenceValue: Value {
        pub(crate) fn ReferenceValue(static_type: &Entity) {
            super(static_type);
        }
    }

    /// Possibly uses attribute.
    pub struct XmlReferenceValue: ReferenceValue {
        let ref m_base: Option<Entity> = None;
        let ref m_qual: Option<Entity> = None;
        let ref m_key: Option<Entity> = None;

        pub(crate) fn XmlReferenceValue(base: &Entity, qualifier: Option<Entity>, key: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_qual(qualifier);
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn qualifier(&self) -> Option<Entity> {
            self.m_qual()
        }

        pub override fn key(&self) -> Entity {
            self.m_key().unwrap()
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn deletable(&self, host: &Database) -> bool {
            true
        }
    }

    /// Possibly uses attribute.
    pub struct DynamicReferenceValue: ReferenceValue {
        let ref m_base: Option<Entity> = None;
        let ref m_qual: Option<Entity> = None;
        let ref m_key: Option<Entity> = None;

        pub(crate) fn DynamicReferenceValue(base: &Entity, qualifier: Option<Entity>, key: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_qual(qualifier);
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn qualifier(&self) -> Option<Entity> {
            self.m_qual()
        }

        pub override fn key(&self) -> Entity {
            self.m_key().unwrap()
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn deletable(&self, host: &Database) -> bool {
            true
        }
    }

    pub struct FixtureReferenceValue: ReferenceValue {
        let ref m_base: Option<Entity> = None;
        let ref m_property: Option<Entity> = None;

        pub(crate) fn FixtureReferenceValue(base: &Entity, property: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_property(Some(property.clone()));
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn property(&self) -> Entity {
            self.m_property().unwrap()
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            self.property().read_only(host)
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            self.property().write_only(host)
        }

        pub override fn deletable(&self, host: &Database) -> bool {
            false
        }
    }

    pub struct StaticReferenceValue: FixtureReferenceValue {
        pub(crate) fn StaticReferenceValue(base: &Entity, property: &Entity, static_type: &Entity) {
            super(base, property, static_type);
        }
    }

    /// A dynamic reference whose base is a class object itself.
    pub struct StaticDynamicReferenceValue: ReferenceValue {
        let ref m_base: Option<Entity> = None;
        let ref m_qual: Option<Entity> = None;
        let ref m_key: Option<Entity> = None;

        pub(crate) fn StaticDynamicReferenceValue(base: &Entity, qualifier: Option<Entity>, key: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_qual(qualifier);
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn qualifier(&self) -> Option<Entity> {
            self.m_qual()
        }

        pub override fn key(&self) -> Entity {
            self.m_key().unwrap()
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn deletable(&self, host: &Database) -> bool {
            false
        }
    }

    /// Instance reference value in a possibly non nullable base.
    pub struct InstanceReferenceValue: FixtureReferenceValue {
        pub(crate) fn InstanceReferenceValue(base: &Entity, property: &Entity, static_type: &Entity) {
            super(base, property, static_type);
        }
    }

    /// Tuple reference value in a possibly non nullable base.
    pub struct TupleReferenceValue: ReferenceValue {
        let ref m_base: Option<Entity> = None;
        let ref m_index: usize = 0;

        pub(crate) fn TupleReferenceValue(base: &Entity, index: usize, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_index(index);
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn tuple_index(&self) -> usize {
            self.m_index()
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn deletable(&self, host: &Database) -> bool {
            false
        }
    }

    pub struct ScopeReferenceValue: FixtureReferenceValue {
        pub(crate) fn ScopeReferenceValue(base: &Entity, property: &Entity, static_type: &Entity) {
            super(base, property, static_type);
        }
    }

    /// Possibly uses attribute.
    pub struct DynamicScopeReferenceValue: ReferenceValue {
        let ref m_base: Option<Entity> = None;
        let ref m_qual: Option<Entity> = None;
        let ref m_key: Option<Entity> = None;

        pub(crate) fn DynamicScopeReferenceValue(base: &Entity, qualifier: Option<Entity>, key: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_qual(qualifier);
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn qualifier(&self) -> Option<Entity> {
            self.m_qual()
        }

        pub override fn key(&self) -> Entity {
            self.m_key().unwrap()
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn deletable(&self, host: &Database) -> bool {
            true
        }
    }

    pub struct PackageReferenceValue: FixtureReferenceValue {
        pub(crate) fn PackageReferenceValue(base: &Entity, property: &Entity, static_type: &Entity) {
            super(base, property, static_type);
        }
    }

    /// Array element reference value with a possibly non-nullable base.
    /// The key is assumed to be of the `Number` data type.
    pub struct ArrayElementReferenceValue: ReferenceValue {
        let ref m_base: Option<Entity> = None;
        let ref m_key: Option<Entity> = None;

        pub(crate) fn ArrayElementReferenceValue(base: &Entity, key: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn key(&self) -> Entity {
            self.m_key().unwrap()
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn deletable(&self, host: &Database) -> bool {
            true
        }
    }

    /// Vector element reference value with a possibly non-nullable base.
    /// The key is assumed to be of the `Number` data type.
    pub struct VectorElementReferenceValue: ReferenceValue {
        let ref m_base: Option<Entity> = None;
        let ref m_key: Option<Entity> = None;

        pub(crate) fn VectorElementReferenceValue(base: &Entity, key: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn key(&self) -> Entity {
            self.m_key().unwrap()
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn deletable(&self, host: &Database) -> bool {
            false
        }
    }

    /// Byte array element reference value with a possibly non-nullable base.
    /// The key is assumed to be of the `Number` data type.
    pub struct ByteArrayElementReferenceValue: ReferenceValue {
        let ref m_base: Option<Entity> = None;
        let ref m_key: Option<Entity> = None;

        pub(crate) fn ByteArrayElementReferenceValue(base: &Entity, key: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_key(Some(key.clone()));
        }

        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn key(&self) -> Entity {
            self.m_key().unwrap()
        }

        pub override fn read_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn write_only(&self, host: &Database) -> bool {
            false
        }

        pub override fn deletable(&self, host: &Database) -> bool {
            false
        }
    }

    /// Represents the resulting value of a conversion, whether implicit or explicit.
    pub struct ConversionValue: Value {
        let ref m_base: Option<Entity> = None;
        let m_kind: ConversionKind = ConversionKind::BetweenNumber;
        let m_opt: bool = true;
        let ref m_target: Option<Entity> = None;

        pub(crate) fn ConversionValue(base: &Entity, variant: ConversionKind, opt: bool, target: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
            self.set_m_kind(variant);
            self.set_m_opt(opt);
            self.set_m_target(Some(target.clone()));
        }

        /// Original value.
        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }

        pub override fn conversion_kind(&self) -> ConversionKind {
            self.m_kind()
        }

        /// Indicates whether the conversion has been performed by the `as` operator
        /// (rather than `T(v)` or implicit conversion) and the resulting type
        /// has been either escaped out of non nullable or made nullable.
        pub override fn conversion_is_opt(&self) -> bool {
            self.m_opt()
        }

        pub override fn conversion_target(&self) -> Entity {
            self.m_target().unwrap()
        }
    }

    /// Non-null assertion value as part of fields from object destructuring.
    pub struct NonNullValue: Value {
        let ref m_base: Option<Entity> = None;

        pub(crate) fn NonNullValue(base: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_base(Some(base.clone()));
        }

        /// Original value.
        pub override fn base(&self) -> Entity {
            self.m_base().unwrap()
        }
    }

    /// Represents the direct value of a `function` expression, holding back its activation.
    pub struct LambdaObject: Value {
        let ref m_activation: Option<Entity> = None;

        pub(crate) fn LambdaObject(activation: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_activation(Some(activation.clone()));
        }

        // Returns a `Some(activation)` value.
        pub override fn activation(&self) -> Option<Entity> {
            self.m_activation()
        }
    }

    /// Represents the direct value of a filter expression, holding back its scope.
    pub struct FilterValue: Value {
        let ref m_scope: Option<Entity> = None;

        pub(crate) fn FilterValue(scope: &Entity, static_type: &Entity) {
            super(static_type);
            self.set_m_scope(Some(scope.clone()));
        }

        pub override fn scope(&self) -> Entity {
            self.m_scope().unwrap()
        }
    }

    /// Resolutions of a field in an object initializer, including shorthand resolution
    /// and the field slot. This is only assigned to a field where applicable.
    pub struct FieldResolution: Entity {
        let ref m_shorthand_resolution: Option<Entity> = None;
        let ref m_field_slot: Option<Entity> = None;

        pub(crate) fn FieldResolution() {
            super();
        }

        pub override fn shorthand_resolution(&self) -> Option<Entity> {
            self.m_shorthand_resolution()
        }

        pub override fn set_shorthand_resolution(&self, value: Option<Entity>) {
            self.set_m_shorthand_resolution(value);
        }

        pub override fn field_slot(&self) -> Option<Entity> {
            self.m_field_slot()
        }

        pub override fn set_field_slot(&self, value: Option<Entity>) {
            self.set_m_field_slot(value);
        }
    }

    /// Resolutions of a field in a declarative object destructuring pattern.
    pub struct DeclarativeFieldDestructuringResolution: Entity {
        let ref m_field_reference: Option<Entity> = None;
        let ref m_var_slot: Option<Entity> = None;

        pub(crate) fn DeclarativeFieldDestructuringResolution() {
            super();
        }

        /// Reference value, non-null value, or constant.
        pub override fn field_reference(&self) -> Option<Entity> {
            self.m_field_reference()
        }

        /// Reference value, non-null value, or constant.
        pub override fn set_field_reference(&self, value: Option<Entity>) {
            self.set_m_field_reference(value);
        }

        /// For fields without subpatterns, indicates
        /// the assigned variable slot.
        pub override fn var_slot(&self) -> Option<Entity> {
            self.m_var_slot()
        }

        /// For fields without subpatterns, indicates
        /// the assigned variable slot.
        pub override fn set_var_slot(&self, value: Option<Entity>) {
            self.set_m_var_slot(value);
        }
    }

    /// Resolutions of a field in an assignment object destructuring pattern.
    pub struct AssignmentFieldDestructuringResolution: Entity {
        let ref m_field_reference: Option<Entity> = None;
        let ref m_target_reference: Option<Entity> = None;

        pub(crate) fn AssignmentFieldDestructuringResolution() {
            super();
        }

        /// Reference value, non-null value, or constant.
        pub override fn field_reference(&self) -> Option<Entity> {
            self.m_field_reference()
        }

        /// Reference value, non-null value, or constant.
        pub override fn set_field_reference(&self, value: Option<Entity>) {
            self.set_m_field_reference(value);
        }

        /// For fields without subpatterns, indicates
        /// the target reference value.
        pub override fn target_reference(&self) -> Option<Entity> {
            self.m_target_reference()
        }

        /// For fields without subpatterns, indicates
        /// the target reference value.
        pub override fn set_target_reference(&self, value: Option<Entity>) {
            self.set_m_target_reference(value);
        }
    }
}

impl ToString for Entity {
    fn to_string(&self) -> String {
        self.to_string_1()
    }
}

impl DiagnosticArgument for Entity {}

#[derive(Copy, Clone, PartialEq)]
pub enum SystemNamespaceKind {
    Public,
    Private,
    Protected,
    Internal,
    StaticProtected,
}

impl ToString for SystemNamespaceKind {
    fn to_string(&self) -> String {
        match self {
            Self::Public => "public".into(),
            Self::Private => "private".into(),
            Self::Protected => "protected".into(),
            Self::Internal => "internal".into(),
            Self::StaticProtected => "static protected".into(),
        }
    }
}

/// A qualified name in ActionScript 3 consisting of
/// a namespace and a local name.
/// 
/// This structure is not intended for E4X, but for representing
/// ActionScript 3 property names.
/// 
/// # Representation
/// 
/// `QName` in this codebase is a type managed by reference counting.
/// Calling `.clone()` in it will clone by reference, not by content.
#[derive(Clone)]
pub struct QName(pub(crate) Rc<QName1>);

impl QName {
    pub fn in_public_or_protected_ns(&self) -> bool {
        let ns = self.namespace();
        ns.is_public_ns() || ns.is_protected_ns()
    }

    pub fn namespace(&self) -> Entity {
        self.0.m_namespace.clone()
    }

    pub fn local_name(&self) -> String {
        self.0.m_local_name.clone()
    }

    pub fn matches_in_ns_set_or_any_public_ns(&self, host: &Database, ns_set: &SharedArray<Entity>, local_name: &str) -> bool {
        self.accessible_from_ns_set(host, ns_set) && self.local_name() == local_name
    }

    pub fn accessible_from_ns_set(&self, host: &Database, ns_set: &SharedArray<Entity>) -> bool {
        let ns1 = self.namespace();
        if !(ns1.is_public_ns() || ns1 == host.as3_ns()) {
            let found_ns = ns_set.iter().find(|ns2| &ns1 == ns2).is_some();
            if !found_ns {
                return false;
            }
        }
        true
    }
}

impl std::hash::Hash for QName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state)
    }
}

impl PartialEq for QName {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for QName {}

pub(crate) struct QName1 {
    pub(crate) m_namespace: Entity,
    pub(crate) m_local_name: String,
}

impl ToString for QName {
    fn to_string(&self) -> String {
        let q = self.namespace();
        let lname = self.local_name();
        if q.is::<SystemNamespace>() {
            return lname;
        }
        format!("{}::{lname}", q.uri())
    }
}

impl DiagnosticArgument for QName {}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct ClassTypeFlags: u16 {
        const IS_FINAL         = 0b00000001;
        const IS_STATIC        = 0b00000010;
        const IS_ABSTRACT      = 0b00000100;
        const IS_DYNAMIC       = 0b00001000;
        const IS_OPTIONS_CLASS = 0b00010000;
        const IS_EXTERNAL      = 0b00100000;
    }
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct VariableSlotFlags: u16 {
        const READ_ONLY     = 0b00000001;
        const IS_EXTERNAL   = 0b00000010;
    }
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct VirtualSlotFlags: u16 {
        const IS_EXTERNAL   = 0b00000001;
    }
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct MethodSlotFlags: u16 {
        const IS_FINAL          = 0b000000001;
        const IS_STATIC         = 0b000000010;
        const IS_ABSTRACT       = 0b000000100;
        const IS_OVERRIDING     = 0b000010000;
        const IS_ASYNC          = 0b000100000;
        const IS_GENERATOR      = 0b001000000;
        const IS_CONSTRUCTOR    = 0b010000000;
        const IS_EXTERNAL       = 0b100000000;
        const IS_NATIVE         = 0b1000000000;
    }
}

/// Parameter belonging to a function type in the semantic model.
pub struct SemanticFunctionTypeParameter {
    pub kind: ParameterKind,
    /// Static type of the parameter. It is never `UnresolvedEntity`
    /// as function types are only created after all compound types
    /// are resolved.
    pub static_type: Entity,
}

impl SemanticFunctionTypeParameter {
    /// Performs type substitution.
    pub fn apply_type(&self, host: &Database, type_params: &SharedArray<Entity>, substitute_types: &SharedArray<Entity>) -> Self {
        Self {
            kind: self.kind,
            static_type: ApplyType(host).exec(&self.static_type, type_params, substitute_types),
        }
    }
}

#[derive(Clone)]
pub struct ControlFlowGraph(Rc<ControlFlowGraph1>);

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self(Rc::new(ControlFlowGraph1 {
            blocks: SharedArray::new(),
            edges: SharedArray::new(),
        }))
    }

    pub fn blocks(&self) -> SharedArray<ControlFlowBlock> {
        self.0.blocks.clone()
    }

    pub fn edges(&self) -> SharedArray<ControlFlowEdge> {
        self.0.edges.clone()
    }
}

impl std::hash::Hash for ControlFlowGraph {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state)
    }
}

impl PartialEq for ControlFlowGraph {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ControlFlowGraph {}

struct ControlFlowGraph1 {
    blocks: SharedArray<ControlFlowBlock>,
    edges: SharedArray<ControlFlowEdge>,
}

#[derive(Clone)]
pub struct ControlFlowBlock(Rc<Vec<Rc<Directive>>>);

impl ControlFlowBlock {
    pub fn new(lines: Vec<Rc<Directive>>) -> Self {
        Self(Rc::new(lines))
    }

    pub fn lines(&self) -> Rc<Vec<Rc<Directive>>> {
        self.0.clone()
    }
}

impl std::hash::Hash for ControlFlowBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state)
    }
}

impl PartialEq for ControlFlowBlock {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for ControlFlowBlock {}

#[derive(Clone)]
pub struct ControlFlowEdge {
    pub from: ControlFlowBlock,
    pub to: ControlFlowBlock,
}

// The third element is the descending most type.
pub struct DescendingClassHierarchy<'a>(Option<Entity>, &'a Database, Entity);

impl<'a> Iterator for DescendingClassHierarchy<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self.0.clone() {
            if r.is::<UnresolvedEntity>() {
                self.0 = None;
            } else {
                self.0 = r.extends_class(self.1);
                if self.0.as_ref().unwrap() == &self.2 {
                    self.0 = None;
                }
            }
            Some(r)
        } else {
            None
        }
    }
}

pub struct DescendingScopeHierarchy(Option<Entity>);

impl Iterator for DescendingScopeHierarchy {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self.0.clone() {
            self.0 = r.parent();
            Some(r)
        } else {
            None
        }
    }
}

pub struct DescendingDefinitionHierarchy(Option<Entity>);

impl Iterator for DescendingDefinitionHierarchy {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self.0.clone() {
            self.0 = r.parent();
            Some(r)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Event {
    /// Possibly `UnresolvedEntity`.
    pub data_type: Entity,
    /// Variable slot that identifies
    /// the event.
    pub constant: Option<Entity>,
    pub bubbles: Option<bool>,
}

const DEFAULT_ACTIVATION: u8 = 0;
const PACKAGE_INIT_ACTIVATION: u8 = 1;
const GLOBAL_INIT_ACTIVATION: u8 = 2;