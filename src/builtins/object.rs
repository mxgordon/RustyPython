use std::sync::Arc;
use lazy_static::lazy_static;
use crate::builtins::function_utils::call_function;
use crate::builtins::pyobjects::*;
use crate::pyarena::PyArena;

lazy_static! {
    pub static ref py_object: Arc<PyClass> = Arc::new(PyClass::new("object", vec![
        ("__new__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::ManyArgs(object__new__))))),
        ("__init__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__init__))))),
        ("__repr__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__repr__))))),
        ("__str__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__str__))))),
        ].into_iter().collect(),
        vec![]));
}

pub fn expect_class(pyobj: Arc<PyObject>) -> Arc<PyClass> {
    match &*pyobj {
        PyObject::Class(class) => class.clone(),
        _ => panic!("Expected class"),
    }
}

// pub fn object__new__(pyclass: Arc<PyObject>) -> Arc<PyObject> {  // error handling
pub fn object__new__(pyargs: Vec<Arc<PyObject>>) -> Arc<PyObject> {  // error handling
    let pyclass = pyargs.get(0).unwrap_or_else(|| panic!("Expected at least one argument to __new__, received 0"));
    
    let pyclass = expect_class(pyclass.clone());
    let pyself = Arc::new(PyObject::Instance(Arc::new(PyInstance::new(pyclass))));

    pyself
}

pub fn object__init__(pyself: Arc<PyObject>) -> Arc<PyObject> {
    // nothing to do
    Arc::new(PyObject::None)
}

pub fn object__repr__(pyself: Arc<PyObject>) -> Arc<PyObject> {
    Arc::new(PyObject::Str(format!("<{} object at {:p}>", pyself.get_class().unwrap().name, &pyself)))
}

pub fn object__str__(pyself: Arc<PyObject>) -> Arc<PyObject> {  // by default make str call repr
    let str_func = pyself.get_attribute("__repr__".to_string()).unwrap();
    call_function(str_func, vec![pyself.clone()], &mut PyArena::new())
}