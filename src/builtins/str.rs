use std::sync::Arc;
use crate::builtins::pyobjects::{PyInternalFunction, PyObject};

pub fn py_str(obj: Arc<PyObject>) -> Arc<PyObject> {
    let obj = obj.clone();
    let str_fn = obj.get_attribute("__str__".to_string());
    
    if str_fn.is_none() {
        panic!("Object has no __str__ method"); // TODO Make python error
    }
    
    let str_fn = str_fn.unwrap();
    let mut str_rtn = Arc::new(PyObject::None);
    
    match &*str_fn {
        PyObject::InternalSlot(func) => {
            match &*func.clone() { 
                PyInternalFunction::OneArg(func) => {
                    str_rtn = func.call((obj.clone(),));
                },
                _ => {todo!()}
            }
        },
        PyObject::Function(func) => {
            todo!()
        },
        _ => {panic!("Object has no __str__ method");}  // TODO Make python error
    }
    
    //assert! TODO check if str_rtn is a string
    
    str_rtn
}