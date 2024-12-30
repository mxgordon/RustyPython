use std::fmt::Debug;
use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{FuncReturnType, PyObject, PyPointer};
use crate::pyarena::PyArena;

#[derive(Debug)]
pub struct PyInstance {
    class: Rc<PyClass>,
    attributes: Option<AHashMap<String, PyPointer<PyObject>>>,
    pub internal: Box<dyn PyInstanceInternal>
}

impl PyInstance {
    pub fn new_empty_attrs(class: Rc<PyClass>, internal: Box<dyn PyInstanceInternal>) -> PyInstance {
        PyInstance {
            class,
            attributes: None,
            internal
        }
    }
    pub fn new_empty(class: Rc<PyClass>) -> PyInstance{
        PyInstance {
            class,
            attributes: None,
            internal: Box::new(EmptyInternal {})
        }
    }
    pub(crate) fn set_field(&mut self, key: String, value: PyPointer<PyObject>, pyarena: &mut PyArena) -> Result<(), PyException> {  // returns if an variable was overwritten (false means a new variable was set)
        if let Some(ref mut attributes) = self.attributes {
            let _result = attributes.insert(key, value);  // TODO this shouldn't be used outside of object methods because it will allow for the setting of new attributes
            return Ok(());
        }

        self.internal.set_field(key, value, pyarena)
    }

    pub(crate) fn get_field(&self, key: &str, pyarena: &mut PyArena) -> Result<PyPointer<PyObject>, PyException> {
        let mut attribute = self.internal.get_field(key, pyarena);

        if attribute.is_none() && let Some(ref attributes) = self.attributes {
            attribute = attributes.get(key).cloned();
        }

        attribute.ok_or_else(|| pyarena.exceptions.attribute_error.instantiate(format!("'{}' object has no attribute `{key}`", self.class.get_name())))
    }

    pub fn get_class(&self) -> &Rc<PyClass> {
        &self.class
    }
}


#[derive(Debug)]
pub struct EmptyInternal {}

pub trait PyInstanceInternal: mopa::Any + Debug {
    fn set_field(&mut self, key: String, value: PyPointer<PyObject>, pyarena: &mut PyArena) -> Result<(), PyException>;  // return field name is successful else exception
    fn get_field(&self, key: &str, pyarena: &mut PyArena) -> Option<PyPointer<PyObject>>;
}

mopafy!(PyInstanceInternal);

impl PyInstanceInternal for EmptyInternal {
    fn set_field(&mut self, key: String, _value: PyPointer<PyObject>, pyarena: &mut PyArena) -> Result<(), PyException> {
        Err(pyarena.exceptions.attribute_error.instantiate(format!("Cannot set {key} for EmptyInternal")))
    }

    fn get_field(&self, _key: &str, _pyarena: &mut PyArena) -> Option<PyPointer<PyObject>> {
        None
    }
}