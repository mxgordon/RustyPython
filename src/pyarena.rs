use std::collections::HashMap;
use std::sync::Arc;
use crate::builtins::object::object;
use crate::builtins::print::py_print;
use crate::builtins::pyobjects::{PyInternalFunction, PyObject};

#[derive(Debug)]
pub struct PyArena {
    state: HashMap<String, Arc<PyObject>>,
}

impl PyArena {
    pub fn new() -> Self {
        PyArena {
            state: HashMap::new(),
        }
    }
    
    pub fn load_builtins(&mut self) {
        self.state.insert("object".to_string(), Arc::new(PyObject::Class(object.clone())));
        self.state.insert("print".to_string(), Arc::new(PyObject::InternalSlot(Arc::new(PyInternalFunction::ManyArgs(py_print)))));
    } 

    pub fn set(&mut self, key: String, value: Arc<PyObject>) {
        self.state.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Arc<PyObject>> {
        self.state.get(key).cloned()
    }
}