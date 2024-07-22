use crate::ns::*;

/// Represents a mapping from a qualified name to a thingy
/// (alias, variable slot, class, or any other thing).
/// 
/// `NameMap` is a mutable mapping managed by reference counting.
///
/// # Cloning
/// 
/// The `clone()` method of `NameMap` clones the mapping
/// by reference, and not by content.
/// 
/// Use `clone_content()` for cloning by content.
/// 
/// # Iteration
///
/// Iteration over the entries in the name mapping is
/// performed through the `borrow()` method.
///
/// ```ignore
/// for (qname, thingy) in name_map.borrow().iter() {
///     // Action
/// });
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct NameMap(SharedMap<QName, Thingy>);

impl NameMap {
    pub fn new() -> Self {
        Self(SharedMap::new())
    }

    pub fn get(&self, name: &QName) -> Option<Thingy> {
        self.0.get(name)
    }

    /// Retrieves a thingy matching a local name in a namespace set.
    pub fn get_in_ns_set(&self, ns_set: &SharedArray<Thingy>, local_name: &str) -> Result<Option<Thingy>, AmbiguousReferenceError> {
        let mut r: Option<Thingy> = None;
        for (qname, thingy) in self.borrow().iter() {
            let ns1 = qname.namespace();
            let found_ns = ns_set.iter().find(|ns2| &ns1 == ns2).is_some();
            if !found_ns {
                continue;
            }
            if qname.local_name() == local_name {
                if r.is_some() {
                    return Err(AmbiguousReferenceError(local_name.to_owned()));
                }
                r = Some(thingy.clone());
            }
        }
        Ok(r)
    }

    /// Retrieves a thingy matching a local name in any namespace.
    pub fn get_in_any_ns(&self, local_name: &str) -> Result<Option<Thingy>, AmbiguousReferenceError> {
        let mut r: Option<Thingy> = None;
        for (qname, thingy) in self.borrow().iter() {
            if qname.local_name() == local_name {
                if r.is_some() {
                    return Err(AmbiguousReferenceError(local_name.to_owned()));
                }
                r = Some(thingy.clone());
            }
        }
        Ok(r)
    }

    /// Retrieves a thingy matching a local name in a namespace set or in any `public` namespace.
    pub fn get_in_ns_set_or_any_public_ns(&self, ns_set: &SharedArray<Thingy>, local_name: &str) -> Result<Option<Thingy>, AmbiguousReferenceError> {
        let mut r: Option<Thingy> = None;
        for (qname, thingy) in self.borrow().iter() {
            let ns1 = qname.namespace();
            if !ns1.is_public_ns() {
                let found_ns = ns_set.iter().find(|ns2| &ns1 == ns2).is_some();
                if !found_ns {
                    continue;
                }
            }
            if qname.local_name() == local_name {
                if r.is_some() {
                    return Err(AmbiguousReferenceError(local_name.to_owned()));
                }
                r = Some(thingy.clone());
            }
        }
        Ok(r)
    }

    /// Retrieves a thingy matching a local name in a namespace set where the namespace is a specific
    /// system namespace.
    pub fn get_in_system_ns_kind_in_ns_set(&self, ns_set: &SharedArray<Thingy>, kind: SystemNamespaceKind, local_name: &str) -> Result<Option<Thingy>, AmbiguousReferenceError> {
        let mut r: Option<Thingy> = None;
        for (qname, thingy) in self.borrow().iter() {
            let ns1 = qname.namespace();
            let found_ns = ns_set.iter().find(|ns2| {
                &ns1 == ns2 && ns1.system_ns_kind().map(|k1| kind == k1).unwrap_or(false)
            }).is_some();
            if !found_ns {
                continue;
            }
            if qname.local_name() == local_name {
                if r.is_some() {
                    return Err(AmbiguousReferenceError(local_name.to_owned()));
                }
                r = Some(thingy.clone());
            }
        }
        Ok(r)
    }

    /// Retrieves a thingy matching a local name in any `public` namespace.
    pub fn get_in_any_public_ns(&self, local_name: &str) -> Result<Option<Thingy>, AmbiguousReferenceError> {
        let mut r: Option<Thingy> = None;
        for (qname, thingy) in self.borrow().iter() {
            if qname.local_name() == local_name && qname.namespace().is_public_ns() {
                if r.is_some() {
                    return Err(AmbiguousReferenceError(local_name.to_owned()));
                }
                r = Some(thingy.clone());
            }
        }
        Ok(r)
    }

    /// Retrieves a thingy matching a local name in any `internal` namespace.
    pub fn get_in_any_internal_ns(&self, local_name: &str) -> Result<Option<Thingy>, AmbiguousReferenceError> {
        let mut r: Option<Thingy> = None;
        for (qname, thingy) in self.borrow().iter() {
            if qname.local_name() == local_name && qname.namespace().is_internal_ns() {
                if r.is_some() {
                    return Err(AmbiguousReferenceError(local_name.to_owned()));
                }
                r = Some(thingy.clone());
            }
        }
        Ok(r)
    }

    /// Retrieves a thingy matching a local name in a specific system namespace kind.
    pub fn get_in_system_ns_kind(&self, kind: SystemNamespaceKind, local_name: &str) -> Result<Option<Thingy>, AmbiguousReferenceError> {
        let mut r: Option<Thingy> = None;
        for (qname, thingy) in self.borrow().iter() {
            if qname.local_name() == local_name && qname.namespace().system_ns_kind().map(|k1| kind == k1).unwrap_or(false) {
                if r.is_some() {
                    return Err(AmbiguousReferenceError(local_name.to_owned()));
                }
                r = Some(thingy.clone());
            }
        }
        Ok(r)
    }

    pub fn set(&mut self, name: QName, thing: Thingy) {
        self.0.set(name, thing);
    }

    pub fn delete(&mut self, name: &QName) -> bool {
        self.0.remove(name).is_some()
    }

    /// The `borrow()` method may be used for iteration. Refer to the
    /// [`NameMap`] documentation for an example.
    pub fn borrow(&self) -> std::cell::Ref<HashMap<QName, Thingy>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&mut self) -> std::cell::RefMut<HashMap<QName, Thingy>> {
        self.0.borrow_mut()
    }

    /// Clones this `NameMap` by content, and not by reference.
    pub fn clone_content(&self) -> Self {
        Self(self.0.clone_content())
    }

    pub fn length(&self) -> usize {
        self.0.length()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn has(&self, name: &QName) -> bool {
        self.0.borrow().contains_key(name)
    }
}