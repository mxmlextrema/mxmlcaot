use crate::ns::*;

/// Operation for verifying the implementation of an interface over a class.
pub struct InterfaceImplement<'a>(pub &'a Database);

impl<'a> InterfaceImplement<'a> {
    pub fn verify(&self, implementor: &Entity, interface: &Entity) -> Result<Vec<InterfaceImplementationLog>, DeferError> {
        let mut interfaces = interface.all_ascending_types(self.0);
        interfaces.push(interface.clone());

        let mut log: Vec<InterfaceImplementationLog> = vec![];

        for interface in interfaces {
            interface.defer()?;

            let implementor_prototype = implementor.prototype(self.0);

            for (name, item) in interface.prototype(self.0).borrow().iter() {
                let local_name = name.local_name();
                let implementor_item = implementor_prototype.get_in_any_public_ns(&local_name).ok().unwrap_or(None);

                if implementor_item.is_none() {
                    if item.is::<VirtualSlot>() {
                        if item.getter(self.0).is_some() {
                            log.push(InterfaceImplementationLog::GetterNotImplemented { name: local_name.clone() });
                        }
                        if item.setter(self.0).is_some() {
                            log.push(InterfaceImplementationLog::SetterNotImplemented { name: local_name.clone() });
                        }
                    } else {
                        log.push(InterfaceImplementationLog::MethodNotImplemented { name: local_name.clone() });
                    }
                // Verify accessors
                } else if item.is::<VirtualSlot>() {
                    let implementor_item = implementor_item.unwrap();
                    if !implementor_item.is::<VirtualSlot>() {
                        log.push(InterfaceImplementationLog::PropertyMustBeVirtual { name: local_name.clone() });
                    } else {
                        // Getter
                        if implementor_item.getter(self.0).is_none() {
                            if item.getter(self.0).is_some() {
                                log.push(InterfaceImplementationLog::GetterNotImplemented { name: local_name.clone() });
                            }
                        } else if item.getter(self.0).is_some() && item.getter(self.0).unwrap().signature(self.0) != implementor_item.getter(self.0).unwrap().signature(self.0) {
                            let expected_signature = item.getter(self.0).unwrap().signature(self.0);
                            expected_signature.defer()?;

                            let actual_signature = implementor_item.getter(self.0).unwrap().signature(self.0);
                            actual_signature.defer()?;

                            log.push(InterfaceImplementationLog::IncompatibleGetterSignature {
                                name: local_name.clone(), expected_signature,
                            });
                        }

                        // Setter
                        if implementor_item.setter(self.0).is_none() {
                            if item.setter(self.0).is_some() {
                                log.push(InterfaceImplementationLog::SetterNotImplemented { name: local_name.clone() });
                            }
                        } else if item.setter(self.0).is_some() && item.setter(self.0).unwrap().signature(self.0) != implementor_item.setter(self.0).unwrap().signature(self.0) {
                            let expected_signature = item.setter(self.0).unwrap().signature(self.0);
                            expected_signature.defer()?;

                            let actual_signature = implementor_item.setter(self.0).unwrap().signature(self.0);
                            actual_signature.defer()?;

                            log.push(InterfaceImplementationLog::IncompatibleSetterSignature {
                                name: local_name.clone(), expected_signature,
                            });
                        }
                    }
                // Verify regular method
                } else {
                    let implementor_item = implementor_item.unwrap();
                    if !implementor_item.is::<MethodSlot>() {
                        log.push(InterfaceImplementationLog::PropertyMustBeMethod { name: local_name.clone() });
                    }

                    let expected_signature = item.signature(self.0);
                    expected_signature.defer()?;

                    let actual_signature = implementor_item.signature(self.0);
                    actual_signature.defer()?;

                    if expected_signature != actual_signature {
                        log.push(InterfaceImplementationLog::IncompatibleMethodSignature {
                            name: local_name.clone(), expected_signature,
                        });
                    }
                }
            }
        }

        Ok(log)
    }
}

/// The log result of verifying interface implementations.
pub enum InterfaceImplementationLog {
    MethodNotImplemented { name: String },
    GetterNotImplemented { name: String },
    SetterNotImplemented { name: String },
    PropertyMustBeMethod { name: String },
    PropertyMustBeVirtual { name: String },
    IncompatibleMethodSignature { name: String, expected_signature: Entity },
    IncompatibleGetterSignature { name: String, expected_signature: Entity },
    IncompatibleSetterSignature { name: String, expected_signature: Entity },
}