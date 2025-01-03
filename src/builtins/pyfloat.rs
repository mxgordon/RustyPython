use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{BivariateFuncType, FuncReturnType, NewFuncType, PyImmutableObject, PyObject, PyPointer, UnaryFuncType};
use crate::builtins::structure::pyobject::PyInternalFunction::{BivariateFunc, NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

pub fn expect_float(pyobj: &PyObject, arena: &mut PyArena) -> Result<f64, PyException> {
    match **pyobj.expect_immutable() {
        PyImmutableObject::Float(value) => {Ok(value)}
        ref value => {
            let message = format!("'{}' object cannot be interpreted as an float", value.get_class(arena).get_name());
            Err(arena.exceptions.type_error.instantiate(message))
        },
    }
}

pub fn float__new__(arena: &mut PyArena, _pyclass: Rc<PyClass>, pyargs: &[PyObject]) -> FuncReturnType {
    let value = pyargs.first();
    
    let new_value;
    
    if let Some(value) = value {
        new_value = match **value.expect_immutable() {
            PyImmutableObject::Int(ref value) => *value as f64,  // copy the value
            PyImmutableObject::Float(ref value) => *value,
            PyImmutableObject::Str(ref value) => value.parse::<f64>().unwrap(),
            ref value => {
                let message = format!("float() argument must be a string, a bytes-like object or a real number, not '{}'", value.get_class(arena).get_name());
                Err(arena.exceptions.type_error.instantiate(message))?
            },
        };
    } else {
        new_value = 0.0;
    }
    
    Ok(PyObject::new_float(new_value))
}

pub fn float__repr__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    Ok(PyObject::new_string(expect_float(pyself, arena)?.to_string()))
}

pub fn float__add__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let self_value = expect_float(pyself, arena)?;
    let other_value = expect_float(other, arena)?;  // TODO make this work for other types (int)

    Ok(PyObject::new_float(self_value + other_value))
}

pub fn get_float_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "float".to_string(),
        super_classes: vec![object_class],
        attributes: AHashMap::new(),
        magic_methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(float__new__ as NewFuncType)))),
            
            __repr__: Some(Rc::new(UnaryFunc(&(float__repr__ as UnaryFuncType)))),
            
            __add__: Some(Rc::new(BivariateFunc(&(float__add__ as BivariateFuncType)))),
            
            ..py_magic_methods_defaults()
        },
    }.create()
}