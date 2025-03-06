use crate::builtins::structure::pyobject::PyObject;

#[derive(Debug)]
pub struct Statics {
    true_: PyObject,
    false_: PyObject,
    
    none_: PyObject,
}

impl Statics {
    pub fn new() -> Self {
        Statics {
            true_: PyObject::create_new_bool(true),
            false_: PyObject::create_new_bool(false),
            
            none_: PyObject::create_new_none(),
        }
    }
    
    pub fn get_bool(&self, value: bool) -> &PyObject {
        if value {
            &self.true_
        } else {
            &self.false_
        }
    }
    
    pub fn none(&self) -> &PyObject {
        &self.none_
    }
}
