use ahash::AHashMap;
use crate::builtins::globals::Globals;
use crate::builtins::structure::pyexception::Exceptions;
use crate::builtins::structure::pyobject::{PyObject, PyPointer};

#[derive(Debug)]
pub struct PyArena {
    state: AHashMap<String, PyPointer<PyObject>>,
    pub globals: Globals,
    pub exceptions: Exceptions,
}

impl PyArena {
    pub fn new() -> Self {
        let globals = Globals::new();
        let exceptions = Exceptions::new();

        PyArena {
            state: globals.load_into_hashmap(),
            globals,
            exceptions
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