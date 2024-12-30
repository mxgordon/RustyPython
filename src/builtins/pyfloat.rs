use std::collections::HashMap;
use std::rc::Rc;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{BivariateFuncType, FuncReturnType, NewFuncType, PyObject, PyPointer, UnaryFuncType};
use crate::builtins::structure::pyobject::PyInternalFunction::{BivariateFunc, NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

pub fn expect_float_ptr(pyobj: PyPointer<PyObject>, arena: &mut PyArena) -> Result<f64, PyException> {
    match *pyobj.borrow() {
        PyObject::Float(value) => Ok(value),
        ref value => {
            let message = format!("'{}' object cannot be interpreted as an float", value.get_class(arena).get_name());
            Err(arena.exceptions.type_error.instantiate(message))
        },
    }
}

pub fn float__new__(arena: &mut PyArena, _pyclass: Rc<PyClass>, pyargs: Vec<PyPointer<PyObject>>) -> FuncReturnType {
    let value = pyargs.first();
    
    let new_value;
    
    if let Some(value) = value {
        new_value = match *value.borrow() {  // cast value
            PyObject::Int(ref value) => *value as f64,  // copy the value
            PyObject::Float(ref value) => *value,
            PyObject::Str(ref value) => value.parse::<f64>().unwrap(),
            ref value => {
                let message = format!("float() argument must be a string, a bytes-like object or a real number, not '{}'", value.get_class(arena).get_name());
                return Err(arena.exceptions.type_error.instantiate(message)); 
            },
        };
    } else {
        new_value = 0.0;
    }
    
    Ok(PyPointer::new(PyObject::Float(new_value)))
}

pub fn float__repr__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> FuncReturnType {
    Ok(PyPointer::new(PyObject::Str(expect_float_ptr(pyself, arena)?.to_string())))
}

pub fn float__add__(arena: &mut PyArena, pyself: PyPointer<PyObject>, other: PyPointer<PyObject>) -> FuncReturnType {
    let self_value = expect_float_ptr(pyself, arena)?;
    let other_value = expect_float_ptr(other, arena)?;  // TODO make this work for other types (int)

    Ok(PyPointer::new(PyObject::Float(self_value + other_value)))
}

pub fn get_float_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "float".to_string(),
        super_classes: vec![object_class],
        attributes: HashMap::new(),
        magic_methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(float__new__ as NewFuncType)))),
            
            __repr__: Some(Rc::new(UnaryFunc(&(float__repr__ as UnaryFuncType)))),
            
            __add__: Some(Rc::new(BivariateFunc(&(float__add__ as BivariateFuncType)))),
            
            ..py_magic_methods_defaults()
        },
    }.create()
}