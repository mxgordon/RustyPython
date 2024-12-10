use std::collections::HashMap;
use crate::builtins::globals::Globals;
use crate::builtins::pyobjects::{PyObject, PyPointer};

#[derive(Debug)]
pub struct PyArena {
    state: HashMap<String, PyPointer<PyObject>>,
    globals: Globals
}

impl PyArena {
    pub fn new() -> Self {
        let globals = Globals::new();
        
        PyArena {
            state: globals.load_into_hashmap(),
            globals
        }
    }

    pub fn set(&mut self, key: String, value: PyPointer<PyObject>) {
        self.state.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<PyPointer<PyObject>> {
        self.state.get(key).cloned()
    }
    
    pub fn remove(&mut self, key: &str) -> Option<PyPointer<PyObject>> {
        self.state.remove(key)
    }
}