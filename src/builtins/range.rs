use std::sync::Arc;
use lazy_static::lazy_static;
use crate::builtins::object::py_object;
use crate::builtins::pyint::expect_int;
use crate::builtins::pyobjects::{PyClass, PyInternalFunction, PyObject};

lazy_static! {
    pub static ref py_range: Arc<PyClass> = Arc::new(PyClass::new("range", vec![
        // ("__new__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__new__))))),
        // ("__init__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__init__))))),
        // ("__repr__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__repr__))))),
        // ("__str__".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::OneArg(object__str__))))),
        ].into_iter().collect(),
        vec![py_object.clone()]));
}


pub fn range__init__(args: Vec<Arc<PyObject>>) -> Arc<PyObject> {
    let pyself = args.get(0).unwrap_or_else(|| panic!("Expected at least one argument to __init__, received 0"));
    let first = expect_int(args.get(1).unwrap_or_else(|| panic!("Expected at least two arguments to __init__, received 1")).clone());
    let second = args.get(2);
    let step = args.get(3);
    
    let mut start: i64;
    let mut stop: i64;
    // let 
    
    if let Some(second) = second {
        start = first;
        stop = expect_int(second.clone());
        
        if let Some(step) = step {
            // step = expect_int(step.clone());
        } else {
            // step = 1;
        }
    } else {
        pyself.set_attribute("start".to_string(), Arc::new(PyObject::Int(0)));
        // start = 0;
        // stop = first;
    }
    

    // let mut values = vec![];
    // for i in (start..end).step_by(step) {
    //     values.push(Arc::new(PyObject::Int(i)));
    // }
    // 
    // let pylist = Arc::new(PyObject::List(values));
    // pyself.set_attribute("values".to_string(), pylist.clone());
    Arc::new(PyObject::None)
}
