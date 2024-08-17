use crate::ns::*;

pub struct Database {
    pub(crate) arena: EntityArena,
    node_mapping: NodeAssignment<Entity>,
    node_invalidation_mapping: NodeAssignment<()>,
    project_path: Option<String>,
    config_constants: SharedMap<String, String>,
    config_constants_result: SharedMap<String, Entity>,
    env_cache: RefCell<Option<Rc<HashMap<String, String>>>>,

    unused_things: Rc<RefCell<Vec<Entity>>>,

    pub(crate) explicit_namespaces: RefCell<HashMap<String, Entity>>,
    pub(crate) user_namespaces: RefCell<HashMap<String, Entity>>,
    pub(crate) qnames: RefCell<HashMap<Entity, HashMap<String, QName>>>,

    invalidation_entity: Entity,
    unresolved_entity: Entity,
    pub(crate) top_level_package: Entity,
    as3_vec_package: RefCell<Option<Entity>>,
    mxmlextrema_utils_package_name: Vec<String>,
    mxmlextrema_utils_package: RefCell<Option<Entity>>,
    any_type: Entity,
    void_type: Entity,
    object_type: RefCell<Option<Entity>>,
    boolean_type: RefCell<Option<Entity>>,
    number_type: RefCell<Option<Entity>>,
    int_type: RefCell<Option<Entity>>,
    uint_type: RefCell<Option<Entity>>,
    float_type: RefCell<Option<Entity>>,
    string_type: RefCell<Option<Entity>>,
    array_type: RefCell<Option<Entity>>,
    namespace_type: RefCell<Option<Entity>>,
    function_type: RefCell<Option<Entity>>,
    class_type: RefCell<Option<Entity>>,
    xml_type: RefCell<Option<Entity>>,
    xml_list_type: RefCell<Option<Entity>>,
    reg_exp_type: RefCell<Option<Entity>>,
    date_type: RefCell<Option<Entity>>,
    promise_type: RefCell<Option<Entity>>,
    vector_type: RefCell<Option<Entity>>,
    proxy_type: RefCell<Option<Entity>>,
    dictionary_type: RefCell<Option<Entity>>,
    byte_array_type: RefCell<Option<Entity>>,
    mxmlextrema_proxy_ns_uri: String,
    mxmlextrema_proxy_ns: RefCell<Option<Entity>>,
    as3_ns: RefCell<Option<Entity>>,
    empty_empty_qname: RefCell<Option<QName>>,
    const_eval_scope: RefCell<Option<Entity>>,

    meta_prop: Entity,
    meta_env_prop: Entity,

    primitive_types: RefCell<Option<Rc<Vec<Entity>>>>,
    non_null_primitive_types: RefCell<Option<Rc<Vec<Entity>>>>,
    numeric_types: RefCell<Option<Rc<Vec<Entity>>>>,
    floating_point_types: RefCell<Option<Rc<Vec<Entity>>>>,
    integer_types: RefCell<Option<Rc<Vec<Entity>>>>,
    pub(crate) types_after_sub: RefCell<HashMap<Entity, Vec<Entity>>>,
    pub(crate) function_types: RefCell<HashMap<usize, Vec<Entity>>>,
    pub(crate) tuple_types: RefCell<HashMap<usize, Vec<Entity>>>,
    pub(crate) nullable_types: RefCell<HashMap<Entity, Entity>>,
    pub(crate) non_nullable_types: RefCell<HashMap<Entity, Entity>>,
    // Slots after indirect type substitution (variable, method, and virtual slots).
    pub(crate) vasub: RefCell<HashMap<Entity, HashMap<SharedArray<Entity>, Vec<Entity>>>>,
    pub(crate) visub: RefCell<HashMap<Entity, HashMap<SharedArray<Entity>, Vec<Entity>>>>,
    pub(crate) mssub: RefCell<HashMap<Entity, HashMap<SharedArray<Entity>, Vec<Entity>>>>,
}

impl Database {
    pub fn new(options: DatabaseOptions) -> Self {
        let arena = EntityArena::new();
        let explicit_namespaces = RefCell::new(HashMap::new());
        let user_namespaces = RefCell::new(HashMap::new());
        let qnames = RefCell::new(HashMap::new());
        let any_type: Entity = AnyType::new(&arena).into();
        let void_type: Entity = VoidType::new(&arena).into();
        let invalidation_entity: Entity = InvalidationEntity::new(&arena).into();
        let unresolved_entity: Entity = UnresolvedEntity::new(&arena).into();
        let top_level_package = Package::new(&arena, "".into());
        let meta_prop: Entity = MetaProperty::new(&arena, &any_type).into();
        let meta_env_prop: Entity = MetaEnvProperty::new(&arena, &any_type).into();
        let host = Self {
            arena,
            node_mapping: NodeAssignment::new(),
            node_invalidation_mapping: NodeAssignment::new(),
            project_path: options.project_path.clone(),
            config_constants: SharedMap::new(),
            config_constants_result: SharedMap::new(),
            env_cache: RefCell::new(None),

            explicit_namespaces,
            user_namespaces,
            qnames,
            top_level_package: top_level_package.clone().into(),
            as3_vec_package: RefCell::new(None),
            mxmlextrema_utils_package_name: options.mxmlextrema_utils_package_name,
            mxmlextrema_utils_package: RefCell::new(None),
            invalidation_entity,
            unresolved_entity,

            unused_things: Rc::new(RefCell::new(vec![])),

            meta_prop,
            meta_env_prop,

            any_type,
            void_type,
            object_type: RefCell::new(None),
            boolean_type: RefCell::new(None),
            number_type: RefCell::new(None),
            int_type: RefCell::new(None),
            uint_type: RefCell::new(None),
            float_type: RefCell::new(None),
            string_type: RefCell::new(None),
            array_type: RefCell::new(None),
            namespace_type: RefCell::new(None),
            function_type: RefCell::new(None),
            class_type: RefCell::new(None),
            xml_type: RefCell::new(None),
            xml_list_type: RefCell::new(None),
            reg_exp_type: RefCell::new(None),
            date_type: RefCell::new(None),
            promise_type: RefCell::new(None),
            vector_type: RefCell::new(None),
            proxy_type: RefCell::new(None),
            dictionary_type: RefCell::new(None),
            byte_array_type: RefCell::new(None),
            mxmlextrema_proxy_ns_uri: options.mxmlextrema_proxy_ns_uri,
            mxmlextrema_proxy_ns: RefCell::new(None),
            as3_ns: RefCell::new(None),
            empty_empty_qname: RefCell::new(None),
            const_eval_scope: RefCell::new(None),

            primitive_types: RefCell::new(None),
            non_null_primitive_types: RefCell::new(None),
            numeric_types: RefCell::new(None),
            floating_point_types: RefCell::new(None),
            integer_types: RefCell::new(None),
            types_after_sub: RefCell::new(HashMap::new()),
            function_types: RefCell::new(HashMap::new()),
            tuple_types: RefCell::new(HashMap::new()),
            nullable_types: RefCell::new(HashMap::new()),
            non_nullable_types: RefCell::new(HashMap::new()),
            vasub: RefCell::new(HashMap::new()),
            visub: RefCell::new(HashMap::new()),
            mssub: RefCell::new(HashMap::new()),
        };

        // Initialize top level namespaces
        top_level_package.set_public_ns(Some(host.factory().create_public_ns(Some(top_level_package.clone().into()))));
        top_level_package.set_internal_ns(Some(host.factory().create_internal_ns(Some(top_level_package.clone().into()))));

        // Initialize the proxy namespace ("flash_proxy" compliant)
        host.mxmlextrema_proxy_ns.replace(Some(host.factory().create_user_ns(host.mxmlextrema_proxy_ns_uri.clone())));

        // Initialize the "AS3" namespace
        host.as3_ns.replace(Some(host.factory().create_user_ns(options.as3_ns_uri)));

        // Initialize empty-empty QName
        host.empty_empty_qname.replace(Some(host.factory().create_qname(&host.factory().create_user_ns("".into()), "".into())));

        // Initialize the scope for constant evaluation
        let const_eval_scope = host.factory().create_scope();
        const_eval_scope.import_list().push(host.factory().create_package_wildcard_import(&host.top_level_package(), None));
        const_eval_scope.open_ns_set().push(host.as3_ns());
        host.const_eval_scope.replace(Some(const_eval_scope));

        host
    }

    #[inline(always)]
    pub fn factory(&self) -> Factory {
        Factory(self)
    }

    /// Mapping from a node to something in the semantic model.
    #[inline(always)]
    pub fn node_mapping(&self) -> &NodeAssignment<Entity> {
        &self.node_mapping
    }

    /// Mapping from a node to an unit indicating invalidation.
    #[inline(always)]
    pub fn node_invalidation_mapping(&self) -> &NodeAssignment<()> {
        &self.node_invalidation_mapping
    }

    pub fn lazy_node_mapping<T>(&self, node: &Rc<T>, init: impl FnOnce() -> Entity) -> Entity
        where NodeAssignment<Entity>: NodeAssignmentMethod<T, Entity>
    {
        if let Some(m) = self.node_mapping().get(node) {
            m
        } else {
            let entity = init();
            self.node_mapping().set(node, Some(entity.clone()));
            entity
        }
    }

    /// The mapping of configuration constants used for
    /// conditional compilation.
    #[inline(always)]
    pub fn config_constants(&self) -> SharedMap<String, String> {
        self.config_constants.clone()
    }

    pub fn clear_config_constants(&self) {
        self.config_constants.clone().clear();
        self.config_constants_result.clone().clear();
    }

    /// The mapping of configuration constants to their verification
    /// result.
    #[inline(always)]
    pub fn config_constants_result(&self) -> SharedMap<String, Entity> {
        self.config_constants_result.clone()
    }

    pub fn empty_empty_qname(&self) -> QName {
        self.empty_empty_qname.borrow().as_ref().unwrap().clone()
    }

    /// Default scope used for the evaluation of configuration constants.
    pub fn const_eval_scope(&self) -> Entity {
        self.const_eval_scope.borrow().as_ref().unwrap().clone()
    }

    pub fn top_level_package(&self) -> Entity {
        self.top_level_package.clone()
    }

    pub fn as3_vec_package(&self) -> Entity {
        if let Some(p) = self.as3_vec_package.borrow().as_ref() {
            return p.clone();
        }
        let p = self.factory().create_package(["__AS3__", "vec"]);
        self.as3_vec_package.replace(Some(p.clone()));
        p
    }

    pub fn mxmlextrema_utils_package(&self) -> Entity {
        if let Some(p) = self.mxmlextrema_utils_package.borrow().as_ref() {
            return p.clone();
        }
        let p = self.factory().create_package(self.mxmlextrema_utils_package_name.iter().map(|s| s.as_str()));
        self.mxmlextrema_utils_package.replace(Some(p.clone()));
        p
    }

    pub fn invalidation_entity(&self) -> Entity {
        self.invalidation_entity.clone()
    }

    pub fn unresolved_entity(&self) -> Entity {
        self.unresolved_entity.clone()
    }

    pub fn any_type(&self) -> Entity {
        self.any_type.clone()
    }

    pub fn void_type(&self) -> Entity {
        self.void_type.clone()
    }

    pub fn meta_property(&self) -> Entity {
        self.meta_prop.clone()
    }

    pub fn meta_env_property(&self) -> Entity {
        self.meta_env_prop.clone()
    }

    global_lookup!(object_type, "Object");
    global_lookup!(boolean_type, "Boolean");
    global_lookup!(number_type, "Number");
    global_lookup!(int_type, "int");
    global_lookup!(uint_type, "uint");
    global_lookup!(float_type, "float");
    global_lookup!(string_type, "String");
    global_lookup!(array_type, "Array");
    global_lookup!(namespace_type, "Namespace");
    global_lookup!(function_type, "Function");
    global_lookup!(class_type, "Class");
    global_lookup!(xml_type, "XML");
    global_lookup!(xml_list_type, "XMLList");
    global_lookup!(reg_exp_type, "RegExp");
    global_lookup!(date_type, "Date");
    global_lookup!(promise_type, "Promise");

    pub fn array_type_of_any(&self) -> Result<Entity, DeferError> {
        let origin = self.array_type().defer()?;
        Ok(self.factory().create_type_after_substitution(&origin, &shared_array![self.any_type()]))
    }

    pub fn promise_type_of_any(&self) -> Result<Entity, DeferError> {
        let origin = self.promise_type().defer()?;
        Ok(self.factory().create_type_after_substitution(&origin, &shared_array![self.any_type()]))
    }

    pub fn vector_type_of_any(&self) -> Result<Entity, DeferError> {
        let origin = self.vector_type().defer()?;
        Ok(self.factory().create_type_after_substitution(&origin, &shared_array![self.any_type()]))
    }

    /// Retrieves `__AS3__.vec.Vector`, a possibly unresolved thing.
    pub fn vector_type(&self) -> Entity {
        if let Some(r) = self.vector_type.borrow().as_ref() {
            return r.clone();
        }
        let pckg = self.as3_vec_package();
        if let Some(r) = pckg.properties(self).get(&self.factory().create_qname(&pckg.public_ns().unwrap().into(), "Vector".to_owned())) {
            self.vector_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved_entity()
        }
    }

    /// Retrieves `mxmlextrema.utils.Proxy`, a possibly unresolved thing.
    pub fn proxy_type(&self) -> Entity {
        if let Some(r) = self.proxy_type.borrow().as_ref() {
            return r.clone();
        }
        let pckg = self.mxmlextrema_utils_package();
        if let Some(r) = pckg.properties(self).get(&self.factory().create_qname(&pckg.public_ns().unwrap().into(), "Proxy".to_owned())) {
            self.proxy_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved_entity()
        }
    }

    /// Retrieves `mxmlextrema.utils.Dictionary`, a possibly unresolved thing.
    pub fn dictionary_type(&self) -> Entity {
        if let Some(r) = self.dictionary_type.borrow().as_ref() {
            return r.clone();
        }
        let pckg = self.mxmlextrema_utils_package();
        if let Some(r) = pckg.properties(self).get(&self.factory().create_qname(&pckg.public_ns().unwrap().into(), "Dictionary".to_owned())) {
            self.dictionary_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved_entity()
        }
    }

    /// Retrieves `mxmlextrema.utils.ByteArray`, a possibly unresolved thing.
    pub fn byte_array_type(&self) -> Entity {
        if let Some(r) = self.byte_array_type.borrow().as_ref() {
            return r.clone();
        }
        let pckg = self.mxmlextrema_utils_package();
        if let Some(r) = pckg.properties(self).get(&self.factory().create_qname(&pckg.public_ns().unwrap().into(), "ByteArray".to_owned())) {
            self.byte_array_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved_entity()
        }
    }

    /// The `mxmlextrema.utils.mxmlextrema_proxy` namespace.
    pub fn mxmlextrema_proxy_ns(&self) -> Entity {
        self.mxmlextrema_proxy_ns.borrow().as_ref().unwrap().clone()
    }

    /// The `AS3` namespace.
    pub fn as3_ns(&self) -> Entity {
        self.as3_ns.borrow().as_ref().unwrap().clone()
    }

    /// Returns the set (`void`, `String`, `Boolean`, `Number`, `int`, `uint`, `float`).
    pub fn primitive_types(&self) -> Result<Rc<Vec<Entity>>, DeferError> {
        if let Some(r) = self.primitive_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.void_type(),
            self.string_type().defer()?,
            self.boolean_type().defer()?,
            self.number_type().defer()?,
            self.int_type().defer()?,
            self.uint_type().defer()?,
            self.float_type().defer()?,
        ]);
        self.primitive_types.replace(Some(r.clone()));
        Ok(r)
    }

    /// Returns the set (`Boolean`, `Number`, `int`, `uint`, `float`).
    pub fn non_null_primitive_types(&self) -> Result<Rc<Vec<Entity>>, DeferError> {
        if let Some(r) = self.non_null_primitive_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.boolean_type().defer()?,
            self.number_type().defer()?,
            self.int_type().defer()?,
            self.uint_type().defer()?,
            self.float_type().defer()?,
        ]);
        self.non_null_primitive_types.replace(Some(r.clone()));
        Ok(r)
    }

    pub fn numeric_types(&self) -> Result<Rc<Vec<Entity>>, DeferError> {
        if let Some(r) = self.numeric_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.number_type().defer()?,
            self.int_type().defer()?,
            self.uint_type().defer()?,
            self.float_type().defer()?,
        ]);
        self.numeric_types.replace(Some(r.clone()));
        Ok(r)
    }

    pub fn floating_point_types(&self) -> Result<Rc<Vec<Entity>>, DeferError> {
        if let Some(r) = self.floating_point_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.number_type().defer()?,
            self.float_type().defer()?,
        ]);
        self.floating_point_types.replace(Some(r.clone()));
        Ok(r)
    }

    pub fn integer_types(&self) -> Result<Rc<Vec<Entity>>, DeferError> {
        if let Some(r) = self.integer_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.int_type().defer()?,
            self.uint_type().defer()?,
        ]);
        self.integer_types.replace(Some(r.clone()));
        Ok(r)
    }

    /// Preloads environment variables from the main project's `.env` file
    /// using the DotEnv file format.
    pub fn env(&self) -> Rc<HashMap<String, String>> {
        if let Some(env) = self.env_cache.borrow().as_ref() {
            return env.clone();
        }
        let mut r = HashMap::<String, String>::new();
        if let Some(project_path) = self.project_path.as_ref() {
            if let Ok(iterator) = dotenvy::from_path_iter(project_path) {
                for item in iterator {
                    if let Ok((key, value)) = item {
                        r.insert(key, value);
                    }
                }
            }
        }
        let r = Rc::new(r);
        self.env_cache.replace(Some(r.clone()));
        r
    }

    pub(crate) fn unused_things(&self) -> std::cell::Ref<Vec<Entity>> {
        self.unused_things.borrow()
    }

    pub(crate) fn is_unused(&self, entity: &Entity) -> bool {
        self.unused_things.borrow().contains(entity)
    }

    pub(crate) fn add_unused_thing(&self, thing: &Entity) {
        self.unused_things.borrow_mut().push(thing.clone());
    }

    pub(crate) fn remove_unused_thing(&self, thing: &Entity) {
        let mut i = 0usize;
        let mut things = self.unused_things.borrow_mut();
        for t1 in things.iter() {
            if thing == t1 {
                things.remove(i);
                break;
            }
            i += 1;
        }
    }
}

#[derive(Clone)]
pub struct DatabaseOptions {
    /// The directory path of the main project being compiled,
    /// used for the `import.meta.env.EXAMPLE` accessors.
    pub project_path: Option<String>,

    /// The "AS3" namespace URI. Default: `"http://adobe.com/AS3/2006/builtin"`.
    pub as3_ns_uri: String,

    /// The "flash_proxy" compliant namespace's URI. Default: `"http://www.adobe.com/2006/actionscript/flash/proxy"`
    pub mxmlextrema_proxy_ns_uri: String,
    /// The "flash.utils" semi compliant package name. Default: `["flash", "utils"]`
    pub mxmlextrema_utils_package_name: Vec<String>,
}

impl Default for DatabaseOptions {
    fn default() -> Self {
        Self {
            project_path: None,
            as3_ns_uri: "http://adobe.com/AS3/2006/builtin".into(),
            mxmlextrema_proxy_ns_uri: "http://www.adobe.com/2006/actionscript/flash/proxy".into(),
            mxmlextrema_utils_package_name: vec!["flash".into(), "utils".into()],
        }
    }
}

macro global_lookup {
    ($field:ident, $as3name:expr) => {
        /// Retrieves a possibly unresolved thing.
        pub fn $field(&self) -> Entity {
            if let Some(r) = self.$field.borrow().as_ref() {
                return r.clone();
            }
            if let Some(r) = self.top_level_package.properties(self).get(&self.factory().create_qname(&self.top_level_package.public_ns().unwrap().into(), $as3name.to_owned())) {
                self.$field.replace(Some(r.clone()));
                r
            } else {
                self.unresolved_entity()
            }
        }
    },
}