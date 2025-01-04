use std::cell::Cell;
use std::collections::hash_map::{Keys, RawEntryMut};
use std::hash::BuildHasher;
use ahash::{AHashMap, RandomState};
use crate::builtins::globals::Globals;
use crate::builtins::structure::pyexception::Exceptions;
use crate::builtins::structure::pyobject::{PyObject};

// #[derive(Debug)]
pub struct PyArena {
    state: AHashMap<String, Cell<PyObject>>,
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

    pub fn get_hash(&self, key: &str) -> u64 {
        let hasher = self.state.hasher();
        hasher.hash_one(key)
    }

    pub fn set_occupied_from_hash(&mut self, hash: u64, key: &str, value: PyObject) {
        let entry = self.state.raw_entry_mut().from_key_hashed_nocheck(hash, key);

        match entry {
            RawEntryMut::Occupied(entry) => {
                entry.get().set(value);
            }
            RawEntryMut::Vacant(_entry) => {
                panic!("Tried to set occupied from hash, but entry was vacant");
            }
        }
    }

    pub fn set(&mut self, key: String, value: PyObject) {
        let entry = self.state.raw_entry_mut().from_key(&key);
        
        match entry {
            RawEntryMut::Occupied(entry) => {
                entry.get().set(value);
            }
            RawEntryMut::Vacant(entry) => {
                entry.insert(key, Cell::new(value));
            }
        }
    }
    
    pub fn update(&mut self, key: &str, value: PyObject) {
        let entry = self.state.raw_entry_mut().from_key(key);
        
        match entry {
            RawEntryMut::Occupied(entry) => {
                entry.get().set(value);
            }
            RawEntryMut::Vacant(_entry) => {
                panic!("Tried to update, but entry was vacant");
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&PyObject> {
        unsafe {
            Some(&*self.state.get(key)?.as_ptr())
        }
    }
    
    pub fn get_cell(&self, key: &str) -> Option<&Cell<PyObject>> {
        self.state.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.state.remove(key);
    }
}