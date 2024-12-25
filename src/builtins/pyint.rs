use std::rc::Rc;
use crate::builtins::pyobjects::PyPointer;
use crate::builtins::pyobjects::*;
use crate::builtins::pyobjects::PyInternalFunction::{BivariateFunc, InitFunc, NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

pub fn expect_int(pyobj: &PyObject) -> i64 {
    match pyobj {
        PyObject::Int(value) => *value,
        ref value => panic!("Expected int, got {:?}", value),
    }
}

pub fn expect_int_ptr(pyobj: PyPointer<PyObject>) -> i64 {
    match *pyobj.borrow() {
        PyObject::Int(value) => value,
        ref value => panic!("Expected int, got {:?}", value),
    }
}

pub fn int__new__(_arena: &mut PyArena, _pyclass: Rc<PyClass>, pyargs: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {  // error handling
    let value = pyargs.first().unwrap();
    
    let new_value = match *value.borrow() {  // cast value
        PyObject::Int(ref value) => *value,  // copy the value
        PyObject::Float(ref value) => *value as i64,
        PyObject::Str(ref value) => value.parse::<i64>().unwrap(),
        _ => panic!("Expected int, str, or float"), // TODO make python error
    };
    
    PyPointer::new(PyObject::Int(new_value))  // I don't know how to do inheritance with this
}

pub fn int__init__(_arena: &mut PyArena, _pyself: PyPointer<PyObject>, _pyargs: Vec<PyPointer<PyObject>>) {
}

pub fn int__add__(_arena: &mut PyArena, pyself: PyPointer<PyObject>, other: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let self_value = expect_int_ptr(pyself);
    let other_value = expect_int_ptr(other);  // TODO make this work for other types (float)
    
    PyPointer::new(PyObject::Int(self_value + other_value))
}

pub fn int__pow__(_arena: &mut PyArena, pyself: PyPointer<PyObject>, other: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let self_value = expect_int_ptr(pyself);
    let other_value = expect_int_ptr(other);  // TODO make this work for other types (float)

    PyPointer::new(PyObject::Int(self_value.pow(other_value as u32)))
}

pub fn int__repr__(_arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let value = expect_int_ptr(pyself);
    PyPointer::new(PyObject::Str(value.to_string()))
}
pub fn get_int_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "int".to_string(),
        super_classes: vec![object_class],

        methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(int__new__ as NewFuncType)))),
            __init__: Some(Rc::new(InitFunc(&(int__init__ as InitFuncType)))),

            __repr__: Some(Rc::new(UnaryFunc(&(int__repr__ as UnaryFuncType)))),
            __add__: Some(Rc::new(BivariateFunc(&(int__add__ as BivariateFuncType)))),
            __pow__: Some(Rc::new(BivariateFunc(&(int__pow__ as BivariateFuncType)))),

            ..py_magic_methods_defaults()
        }
    }.create()
}

