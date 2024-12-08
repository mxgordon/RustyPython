use std::sync::Arc;
use lazy_static::lazy_static;
use crate::builtins::object::{expect_class, py_object};
use crate::builtins::pyobjects::*;

lazy_static! {
    pub static ref py_int: Arc<PyClass> = Arc::new(PyClass::new("int", vec![
        ("__new__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::TwoArgs(int__new__))))),
        ("__init__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::TwoArgs(int__init__))))),
        ("__add__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::TwoArgs(int__add__))))),
        ("__pow__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::TwoArgs(int__pow__))))),
        ("__repr__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(int__repr__))))),
        ].into_iter().collect(),
        vec![py_object.clone()]));
}

pub fn expect_int(pyobj: Arc<PyObject>) -> i64 {  //! This lowkey might end up constantly referencing a new pointer to the i64
    match &*pyobj {
        PyObject::Int(value) => value.clone(),
        _ => panic!("Expected int"),
    }
}

pub fn expect_set_int(pyobj: Arc<PyObject>, new_value: i64){
    match &*pyobj {
        PyObject::Int(value) => {*value + new_value},
        _ => panic!("Expected int"),
    };
}

pub fn int__new__(pyclass: Arc<PyObject>, value: Arc<PyObject>) -> Arc<PyObject> {  // error handling
    let _pyclass = expect_class(pyclass);  // TODO do something to make inheritance good

    let new_value = match &*value {  // cast value
        PyObject::Int(value) => value.clone(),  // copy the value
        PyObject::Float(value) => *value as i64,
        PyObject::Str(value) => value.parse::<i64>().unwrap(),
        _ => panic!("Expected int, str, or float"), // TODO make python error
    };
    
    let pyself = Arc::new(PyObject::Int(new_value));
    pyself
}

pub fn int__init__(pyself: Arc<PyObject>, value: Arc<PyObject>) -> Arc<PyObject> {
    Arc::new(PyObject::None)
}

pub fn int__add__(pyself: Arc<PyObject>, other: Arc<PyObject>) -> Arc<PyObject> {
    let self_value = expect_int(pyself);
    let other_value = expect_int(other);  // TODO make this work for other types (float)
    
    Arc::new(PyObject::Int(self_value + other_value))
}

pub fn int__pow__(pyself: Arc<PyObject>, other: Arc<PyObject>) -> Arc<PyObject> {
    let self_value = expect_int(pyself);
    let other_value = expect_int(other);  // TODO make this work for other types (float)

    Arc::new(PyObject::Int(self_value.pow(other_value as u32)))
}

pub fn int__repr__(pyself: Arc<PyObject>) -> Arc<PyObject> {
    let value = expect_int(pyself);
    Arc::new(PyObject::Str(value.to_string()))
}

