#![allow(non_snake_case)]
use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{FuncReturnType, NewFuncType, PyImmutableObject, PyObject, UnaryFuncType};
use crate::builtins::structure::pyobject::PyInternalFunction::{NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

pub fn expect_none(pyobj: &PyObject, arena: &mut PyArena) -> Result<(), PyException> {
    match **pyobj.expect_immutable() {
        PyImmutableObject::None => { Ok(()) }
        ref value => {
            let message = format!("'{}' object cannot be interpreted as a NoneType", value.get_class(arena).get_name());
            Err(arena.exceptions.type_error.instantiate(message))
        },
    }
}

pub fn none__new__(arena: &mut PyArena, _pyclass: Rc<PyClass>, pyargs: &[PyObject]) -> FuncReturnType {  
    assert_eq!(pyargs.len(), 0); // TODO make python error

    Ok(arena.statics.none().clone())
}

pub fn none__repr__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    expect_none(pyself, arena)?;
    Ok(PyObject::new_string("None".to_string()))
}

pub fn get_none_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "NoneType".to_string(),
        super_classes: vec![object_class],
        attributes: AHashMap::new(),
        magic_methods: Box::new(PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(none__new__ as NewFuncType)))),

            __repr__: Some(Rc::new(UnaryFunc(&(none__repr__ as UnaryFuncType)))),

            ..py_magic_methods_defaults()
        })
    }.create()
}

