use std::collections::hash_map::OccupiedEntry;
use ahash::AHashMap;
use crate::builtins::globals::Globals;
use crate::builtins::structure::pyexception::Exceptions;
use crate::builtins::structure::pyobject::{PyObject};

#[derive(Debug)]
pub struct PyArena<'a> {
    state: AHashMap<String, PyObject>,
    cache: Vec<OccupiedEntry<'a, String, PyObject>>,
    pub globals: Globals,
    pub exceptions: Exceptions,
}

impl<'a> PyArena<'a> {
    pub fn new() -> Self {
        let globals = Globals::new();
        let exceptions = Exceptions::new();

        PyArena {
            state: globals.load_into_hashmap(),
            cache: Vec::new(),
            globals,
            exceptions
        }
    }

    pub fn set(&mut self, key: String, value: PyObject) {
        self.state.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&PyObject> {
        self.state.get(key)
    }
    
    pub fn get_mut_ref(&mut self, key: &str) -> Option<&mut PyObject> {
        self.state.get_mut(key)
    }
    
    // pub fn set_and_cache_entry(&'a mut self, key: String, value: PyObject) -> usize {
    //     let entry = self.state.entry(key).insert_entry(value);
    //     self.cache.push(entry);
    //     self.cache.len() - 1
    // }
    
    pub fn set_cached_entry(&mut self, index: usize, value: PyObject) {
        self.cache[index].insert(value);
    }

    pub fn remove(&mut self, key: &str) -> Option<PyObject> {
        self.state.remove(key)
    }
}