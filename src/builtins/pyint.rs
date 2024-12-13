use crate::builtins::pyobjects::PyPointer;
use crate::builtins::object::{py_object};
use crate::builtins::pyobjects::*;
use crate::builtins::pyobjects::PyInternalFunction::{BivariateFunc, InitFunc, NewFunc, UnaryFunc};

pub fn expect_int(pyobj: PyPointer<PyObject>) -> i64 {
    match **pyobj.borrow() {
        PyObject::Int(value) => value,
        _ => panic!("Expected int"),
    }
}

pub fn expect_set_int(pyobj: PyPointer<PyObject>, new_value: i64){
    match **pyobj.borrow() {
        PyObject::Int(value) => {value + new_value},
        _ => panic!("Expected int"),
    };
}

pub fn int__new__(_pyclass: PyPointer<PyClass>, pyargs: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {  // error handling
    let value = pyargs.get(0).unwrap();
    
    let new_value = match **value.borrow() {  // cast value
        PyObject::Int(ref value) => value.clone(),  // copy the value
        PyObject::Float(ref value) => *value as i64,
        PyObject::Str(ref value) => value.parse::<i64>().unwrap(),
        _ => panic!("Expected int, str, or float"), // TODO make python error
    };
    
    let pyself = PyPointer::new(PyObject::Int(new_value));  // idk how to do inheritance with this
    pyself
}

pub fn int__init__(_pyself: PyPointer<PyObject>, _pyargs: Vec<PyPointer<PyObject>>) {
}

pub fn int__add__(pyself: PyPointer<PyObject>, other: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let self_value = expect_int(pyself);
    let other_value = expect_int(other);  // TODO make this work for other types (float)
    
    PyPointer::new(PyObject::Int(self_value + other_value))
}

pub fn int__pow__(pyself: PyPointer<PyObject>, other: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let self_value = expect_int(pyself);
    let other_value = expect_int(other);  // TODO make this work for other types (float)

    PyPointer::new(PyObject::Int(self_value.pow(other_value as u32)))
}

pub fn int__repr__(pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let value = expect_int(pyself);
    PyPointer::new(PyObject::Str(value.to_string()))
}
pub const py_int: PyClass = PyClass::Internal {
    name_func: || "int".to_string(),
    super_classes_func: || vec![PyPointer::new(py_object)],
    
    methods: PyMagicMethods {
        __new__: Some(NewFunc(&(int__new__ as NewFuncType))),
        __init__: Some(InitFunc(&(int__init__ as InitFuncType))),
        
        __repr__: Some(UnaryFunc(&(int__repr__ as UnaryFuncType))),
        __add__: Some(BivariateFunc(&(int__add__ as BivariateFuncType))),
        __pow__: Some(BivariateFunc(&(int__pow__ as BivariateFuncType))),
        
        ..py_magic_methods_defaults()
    }
};

