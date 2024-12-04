use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use lazy_static::lazy_static;
use crate::builtins::pyobjects::*;

use crate::builtins::types::*;

lazy_static! {
    pub static ref object: Arc<PyClass> = Arc::new(PyClass::new("object", vec![
        ("__new__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__new__))))),
        ("__init__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__init__))))),
        ("__repr__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__repr__))))),
        ("__str__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__str__))))),
        
        ].into_iter().collect(),
        // ("__str__".to_string(), PyObject::InternalDef),
        vec![]));
}
// static object: PyClass = PyClass::new_internal_attrs("object", vec!["__new__"], vec![]);

// static OBJECT: Lazy<PyClass> = Lazy::new(|| {
//     PyClass::new_internal_attrs("object", vec!["__new__"], vec![])
// });

pub fn expect_class(pyobj: Arc<PyObject>) -> Arc<PyClass> {
    match &*pyobj {
        PyObject::Class(class) => class.clone(),
        _ => panic!("Expected class"),
    }
}

pub fn object__new__(pyclass: Arc<PyObject>) -> Arc<PyObject> {  // error handling
    let pyclass = expect_class(pyclass);
    let pyself = Arc::new(PyObject::Instance(Arc::new(PyInstance::new(pyclass))));

    pyself
}

pub fn object__init__(pyself: Arc<PyObject>) -> Arc<PyObject> {
    // nothing to do
    Arc::new(PyObject::None)
}

pub fn object__repr__(pyself: Arc<PyObject>) -> Arc<PyObject> {
    object__str__(pyself)
}

pub fn object__str__(pyself: Arc<PyObject>) -> Arc<PyObject> {
    Arc::new(PyObject::Str(format!("<{} object at {:p}>", pyself.get_class().unwrap().name, &pyself)))
}