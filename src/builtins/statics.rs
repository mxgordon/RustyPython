use crate::builtins::structure::pyobject::PyObject;

#[derive(Debug)]
pub struct Statics {
    true_: PyObject,
    false_: PyObject,
    
    none_: PyObject,
    
    not_implemented: PyObject,
}

impl Statics {
    pub fn new() -> Self {
        Statics {
            true_: PyObject::create_new_bool(true),
            false_: PyObject::create_new_bool(false),
            
            none_: PyObject::create_new_none(),
            
            not_implemented: PyObject::create_new_not_implemented(),
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
    
    pub fn not_implemented(&self) -> &PyObject {
        &self.not_implemented
    }
}
