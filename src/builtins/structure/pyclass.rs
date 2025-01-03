use std::rc::Rc;
use ahash::AHashMap;
use strum::IntoEnumIterator;
use crate::builtins::structure::magic_methods::{PyMagicMethod, PyMagicMethods};
use crate::builtins::structure::pyobject::{PyImmutableObject, PyInternalFunction, PyObject};

#[derive(Debug)]
pub enum PyClass {
    UserDefined {
        name: String,
        attributes: AHashMap<String, PyObject>,
        super_classes: Vec<Rc<PyClass>>
    },
    Internal {
        name: String,
        super_classes: Vec<Rc<PyClass>>,
        magic_methods: PyMagicMethods,
        attributes: AHashMap<String, PyObject>,
    },
    // Exception {exception: PyExceptionType},
}

impl PyClass {  // TODO !automatic caching function that sets the name, superclass, and inheritance functions
    pub fn get_name(&self) -> &String {
        match self {
            PyClass::UserDefined { name, .. } => name,
            PyClass::Internal { name, .. } => name,
        }
    }

    pub fn get_super_classes(&self) -> Vec<Rc<PyClass>> {
        match self {
            PyClass::UserDefined { super_classes, .. } => super_classes.clone(),
            PyClass::Internal { super_classes, .. } => super_classes.clone(),
        }
    }

    pub fn defines_attribute(&self, magic_method: PyMagicMethod) -> bool {
        match self {
            PyClass::UserDefined { attributes, .. } => attributes.contains_key(&magic_method.to_string()),
            PyClass::Internal {
                magic_methods: methods, ..
            } => magic_method.get_method(methods).is_some(),
        }
    }

    pub fn create(mut self) -> Self {
        let attrs = self.get_super_magic_methods();
        self.load_super_magic_methods(attrs);
        self
    }

    pub fn get_super_magic_methods(&self) -> Vec<(PyMagicMethod, Rc<PyInternalFunction>)> {
        let mut methods_to_set = Vec::new();
        if let PyClass::Internal { .. } = self {
            for magic_method_type in PyMagicMethod::iter() {
                if self.defines_attribute(magic_method_type.clone()) {
                    continue;
                }

                if let Some(super_method) = self.search_for_magic_method_internal(magic_method_type.clone()) {
                    methods_to_set.push((magic_method_type, super_method));
                }

            }
        }

        methods_to_set
    }

    pub fn load_super_magic_methods(&mut self, methods_to_set: Vec<(PyMagicMethod, Rc<PyInternalFunction>)>) {
        match self {
            PyClass::Internal { magic_methods: methods, .. } => {
                for (magic_method_type, super_method) in methods_to_set {
                    methods.set_method(magic_method_type, super_method);
                }
            }
            _ => todo!()
        }
    }

    pub fn search_for_magic_method_internal(&self, magic_method: PyMagicMethod) -> Option<Rc<PyInternalFunction>> {
        let search_result = match self {
            PyClass::UserDefined { .. } => {
                panic!("UserDefined classes will not have internal methods")
            },

            PyClass::Internal { magic_methods: methods, .. } => {
                methods.get_method(magic_method.clone())
            }

        };

        if search_result.is_none() {
            for base_class in self.get_super_classes() {
                let attr = base_class.search_for_magic_method_internal(magic_method.clone());

                if attr.is_some() {
                    return attr.clone();
                }
            }
        }

        search_result
    }

    pub fn get_magic_method_internal(&self, magic_method: PyMagicMethod) -> Option<Rc<PyInternalFunction>> {
        match self {
            PyClass::UserDefined { .. } => {
                panic!("UserDefined classes will not have internal methods")
            },

            PyClass::Internal { magic_methods: methods, ..} => {
                let attr = methods.get_method(magic_method.clone());
                attr
            }
        }
    }

    pub fn search_for_magic_method(&self, magic_method: PyMagicMethod) -> Option<PyObject> {
        match self {
            PyClass::UserDefined { attributes, .. } => {
                attributes.get(&magic_method.to_string()).cloned()
            },

            PyClass::Internal { magic_methods: methods, .. } => {
                Some(PyObject::new_immutable(PyImmutableObject::InternalSlot(methods.get_method(magic_method.clone())?)))
            }
        }
    }

    pub fn search_for_method(&self, method_name: &str) -> Option<PyObject> {
        match self {
            PyClass::UserDefined { attributes, .. } => {
                attributes.get(method_name).cloned()
            },

            PyClass::Internal { magic_methods, attributes, .. } => {
                let magic_method = PyMagicMethod::from_string(method_name);
                if let Some(magic_method) = magic_method {
                    let method = magic_methods.get_method(magic_method);

                    if let Some(method) = method {
                        return Some(PyObject::new_immutable(PyImmutableObject::InternalSlot(method)));
                    }
                }

                attributes.get(method_name).cloned()
            }
        }
    }
}