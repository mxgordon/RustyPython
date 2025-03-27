use crate::builtins::globals::Globals;
use crate::builtins::statics::Statics;
use crate::builtins::structure::pyexception::Exceptions;
use crate::builtins::structure::pyobject::PyObject;
use crate::parser::Variable;
use ahash::{AHashMap, RandomState};
use std::cell::{Ref, RefCell};
use std::collections::hash_map::RawEntryMut;
use std::hash::BuildHasher;
use std::rc::Rc;

// #[derive(Debug)]
pub struct PyArena {
    // state: AHashMap<String, Cell<PyObject>>,
    frames: Vec<Frame>,
    hasher: RandomState,
    pub globals: Globals,
    pub statics: Statics,
    pub exceptions: Exceptions,
}

impl PyArena {
    pub fn new(initial_fast_locals_size: usize) -> Self {
        let hasher = RandomState::new();
        let globals = Globals::new();
        let statics = Statics::new();
        let exceptions = Exceptions::new();

        let top_frame = Frame::new(hasher.clone(), initial_fast_locals_size).add_globals(&globals);

        PyArena {
            frames: vec![top_frame],
            hasher,
            globals,
            statics,
            exceptions,
        }
    }

    pub fn get_hash(&self, key: &str) -> u64 {
        self.hasher.hash_one(key)
    }

    pub fn get_current_frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    pub fn get_current_frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn get_parent_frame(&self, parent_level: usize) -> Option<&Frame> {
        self.frames.get(self.frames.len() - 2 - parent_level)
    }

    pub fn get_parent_frame_mut(&mut self, parent_level: usize) -> Option<&mut Frame> {
        let idx = self.frames.len() - 2 - parent_level;

        self.frames.get_mut(idx)
    }

    pub fn search_for_var(&self, variable: &Rc<Variable>) -> Option<Ref<PyObject>> {
        let hash = self.hasher.hash_one(&variable.name);

        for frame in self.frames.iter().rev() {
            // start with the newest frame
            let frame_var = frame.get_with_hash(variable, hash);

            if let Some(frame_var) = frame_var {
                return Some(frame_var.borrow())
            }
        }

        None
    }

    // pub fn set_occupied_from_hash(&mut self, hash: u64, key: &str, value: PyObject) {
    //     let entry = self.state.raw_entry_mut().from_key_hashed_nocheck(hash, key);
    //
    //     match entry {
    //         RawEntryMut::Occupied(entry) => {
    //             entry.get().set(value);
    //         }
    //         RawEntryMut::Vacant(_entry) => {
    //             panic!("Tried to set occupied from hash, but entry was vacant");
    //         }
    //     }
    // }

    // pub fn set_local(&mut self, key: String, value: PyObject) {
    //     let local_frame = self.frames.last_mut().unwrap();
    //
    //     let entry = local_frame.raw_entry_mut().from_key(&key);
    //
    //     match entry {
    //         RawEntryMut::Occupied(entry) => {
    //             entry.get().set(value);
    //         }
    //         RawEntryMut::Vacant(entry) => {
    //             entry.insert(key, Cell::new(value));
    //         }
    //     }
    // }

    // pub fn update_local(&mut self, key: &str, value: PyObject) {
    //     let local_frame = self.frames.las;
    //
    //     let entry = local_frame.raw_entry_mut().from_key(key);
    //
    //     match entry {
    //         RawEntryMut::Occupied(entry) => {
    //             entry.get().set(value);
    //         }
    //         RawEntryMut::Vacant(_entry) => {
    //             panic!("Tried to update, but entry was vacant");
    //         }
    //     }
    // }

    // pub fn get(&self, key: &str) -> Option<&PyObject> {
    //     unsafe {
    //         Some(&*self.state.get(key)?.as_ptr())
    //     }
    // }
    //
    // pub fn get_cell(&self, key: &str) -> Option<&Cell<PyObject>> {
    //     self.state.get(key)
    // }
    //
    // pub fn remove(&mut self, key: &str) {
    //     self.state.remove(key);
    // }
}

type FrameRef = Rc<RefCell<PyObject>>;

pub struct Frame {
    // name: String,
    locals: AHashMap<String, FrameRef>,
    // size: usize,
    fast_locals: Vec<Option<FrameRef>>,
}

impl Frame {
    pub fn new(hasher: RandomState, fast_locals_size: usize) -> Self {
        Frame {
            locals: AHashMap::with_hasher(hasher),
            fast_locals: vec![None; fast_locals_size],
        }
    }

    pub fn add_globals(mut self, globals: &Globals) -> Self {
        let exposed_globals = globals.create_exposed_globals();

        self.locals.extend(exposed_globals.into_iter());

        self
    }

    fn set_and_return_local(
        &mut self,
        variable: &Rc<Variable>,
        value: PyObject,
    ) -> Rc<RefCell<PyObject>> {
        let entry = self.locals.raw_entry_mut().from_key(&variable.name);

        match entry {
            RawEntryMut::Occupied(entry) => {
                let local_cell = entry.get();

                let mut local_pyobj = local_cell.borrow_mut();
                *local_pyobj = value;

                local_cell.clone()
            }
            RawEntryMut::Vacant(entry) => {
                let new_value = Rc::new(RefCell::new(value));

                entry.insert(variable.name.clone(), new_value.clone());

                new_value.clone()
            }
        }
    }

    fn set_local(&mut self, variable: &Rc<Variable>, value: PyObject) {
        let entry = self.locals.raw_entry_mut().from_key(&variable.name);

        match entry {
            RawEntryMut::Occupied(entry) => {
                let local_cell = entry.get();

                let mut local_pyobj = local_cell.borrow_mut();
                *local_pyobj = value;
            }
            RawEntryMut::Vacant(entry) => {
                let new_value = Rc::new(RefCell::new(value));

                entry.insert(variable.name.clone(), new_value.clone());
            }
        }
    }

    pub fn set(&mut self, variable: &Rc<Variable>, value: PyObject) {
        if let Some(fast_locals_loc) = variable.fast_locals_loc {
            if let Some(ref fast_local) = self.fast_locals[fast_locals_loc] {
                let mut fast_local_pyobj = fast_local.borrow_mut();

                *fast_local_pyobj = value; // sets value using fast_locals
            } else {
                let new_fast_local = self.set_and_return_local(variable, value);

                self.fast_locals[fast_locals_loc] = Some(new_fast_local);
            }
        } else {
            self.set_local(variable, value);
        }
    }

    fn get_local(&self, variable: &Rc<Variable>) -> Option<&FrameRef> {
        self.locals.get(&variable.name)
    }

    pub fn get(&mut self, variable: &Rc<Variable>) -> Option<&FrameRef> {
        if let Some(fast_locals_loc) = variable.fast_locals_loc {
            self.fast_locals[fast_locals_loc].as_ref()
        } else {
            self.get_local(variable)
        }
    }

    fn get_local_with_hash(&self, variable: &Rc<Variable>, hash: u64) -> Option<&FrameRef> {
            let entry = self.locals.raw_entry().from_key_hashed_nocheck(hash, &variable.name);

            entry.and_then(|(_key, value)| Some(value))
    }
    pub fn get_with_hash(&self, variable: &Rc<Variable>, hash: u64) -> Option<&FrameRef> {
        if let Some(fast_locals_loc) = variable.fast_locals_loc {
            self.fast_locals[fast_locals_loc].as_ref()
        } else {
            self.get_local_with_hash(variable, hash)
        }
    }

    // fn get_local_frame_ref(&self, variable: &Rc<Variable>) -> Option<&FrameRef> {
    //     self.locals.get(&variable.name)
    // }

    // fn get_fast_local(&self, position: usize) -> &Option<PyObject> {
    //     self.fast_locals.get(position).expect("index out of bounds")
    // }
    // //
    // // fn get_fast_locals_mut(&mut self, position: usize) -> &mut Option<PyObject> {
    // //     self.fast_locals.get_mut(position).expect("index out of bounds")
    // // }
    //
    // fn set_fast_local(&mut self, position: usize, new_py_obj: PyObject) {
    //     self.fast_locals[position] = Some(new_py_obj)
    // }
    //
    //
    // pub fn set_occupied_from_hash(&mut self, hash: u64, key: &str, value: PyObject) {
    //     let entry = self.locals.raw_entry_mut().from_key_hashed_nocheck(hash, key);
    //
    //     match entry {
    //         RawEntryMut::Occupied(entry) => {
    //             entry.get().set(value);
    //         }
    //         RawEntryMut::Vacant(_entry) => {
    //             panic!("Tried to set occupied from hash, but entry was vacant");
    //         }
    //     }
    // }
    //
    // pub fn set_from_var(&mut self, variable: &Rc<Variable>, value: PyObject) {
    //     self.set(variable.name.clone(), value);
    // }
    //
    // pub fn set(&mut self, key: String, value: PyObject) {
    //     let entry = self.locals.raw_entry_mut().from_key(&key);
    //
    //     match entry {
    //         RawEntryMut::Occupied(entry) => {
    //             entry.get().clone().  //.set(value);
    //         }
    //         RawEntryMut::Vacant(entry) => {
    //             entry.insert(key, Rc::new(value));
    //         }
    //     }
    // }
    //
    // pub fn update_from_var(&mut self, variable: &Rc<Variable>, value: PyObject) {
    //     self.update(&variable.name, value)
    // }
    //
    // pub fn update(&mut self, key: &str, value: PyObject) {
    //     let entry = self.locals.raw_entry_mut().from_key(key);
    //
    //     match entry {
    //         RawEntryMut::Occupied(entry) => {
    //             entry.get().set(value);
    //         }
    //         RawEntryMut::Vacant(_entry) => {
    //             panic!("Tried to update, but entry was vacant");
    //         }
    //     }
    // }
    //
    // pub fn get_from_var(&self, variable: &Rc<Variable>) -> Option<&PyObject> {
    //     self.get(&variable.name)
    // }
    //
    // pub fn get(&self, key: &str) -> Option<&PyObject> {
    //     unsafe {
    //         Some(&*self.locals.get(key)?.as_ptr())
    //     }
    // }
    //
    // pub fn get_with_hash(&self, hash: u64, key: &str) -> Option<&PyObject> {
    //     let entry = self.locals.raw_entry().from_key_hashed_nocheck(hash, key);
    //
    //     entry.and_then(|(_key, value)| {
    //         unsafe {
    //             Some(&*value.as_ptr())
    //         }
    //     })
    // }
    //
    // pub fn get_cell(&self, key: &str) -> Option<&Cell<PyObject>> {
    //     self.locals.get(key)
    // }

    pub fn remove(&mut self, key: &str) {
        self.locals.remove(key);
    }
}
