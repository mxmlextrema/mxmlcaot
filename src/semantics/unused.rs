use crate::ns::*;

/// Handle unused entities.
pub struct Unused<'a>(pub &'a Database);

impl<'a> Unused<'a> {
    pub fn all(&self) -> std::cell::Ref<Vec<Entity>> {
        self.0.unused_things()
    }

    pub fn is_unused(&self, entity: &Entity) -> bool {
        self.0.is_unused(entity)
    }

    pub fn add(&self, thing: &Entity) {
        self.0.add_unused_thing(thing);
    }

    pub fn add_nominal(&self, thing: &Entity) {
        let name = thing.name();
        if name.in_public_or_protected_ns() || name.local_name().starts_with('_') {
            return;
        }
        self.add(thing);
    }

    pub fn mark_used(&self, property: &Entity) {
        if property.is::<InvalidationEntity>() {
            return;
        }
        let qn = property.name();
        if !qn.in_public_or_protected_ns() {
            if property.is_entity_after_substitution() {
                self.mark_used(&property.origin());
            } else {
                self.0.remove_unused_thing(property);
            }
        }
    }
}