use std::collections::HashMap;
use crate::builtins::object::py_object;
use crate::builtins::print::py_print;
use crate::builtins::pyint::{py_int};
use crate::builtins::pyobjects::{ManyArgFuncType, PyClass, PyInternalFunction, PyObject, PyPointer};
use crate::builtins::range::{py_range};

#[derive(Debug)]
pub struct Globals {
    pub object_class: PyPointer<PyClass>,
    pub int_class: PyPointer<PyClass>,
    pub range_class: PyPointer<PyClass>,
    pub print_func: PyPointer<PyInternalFunction>,
}

impl Globals {
    pub(crate) fn new() -> Globals {
        let object_class = PyPointer::new(py_object);
        let int_class = PyPointer::new(py_int);
        let range_class = PyPointer::new(py_range);
        
        Globals {
            object_class,
            int_class,
            range_class,
            print_func: PyPointer::new(PyInternalFunction::ManyArgFunc(&(py_print as ManyArgFuncType))),
        }
    }
    
    pub(crate) fn load_into_hashmap(&self) -> HashMap<String, PyPointer<PyObject>> {
        vec![
            ("object".to_string(), PyPointer::new(PyObject::Class(self.object_class.clone()))),
            ("int".to_string(), PyPointer::new(PyObject::Class(self.int_class.clone()))),
            ("range".to_string(), PyPointer::new(PyObject::Class(self.range_class.clone()))),
            ("print".to_string(), PyPointer::new(PyObject::InternalSlot(self.print_func.clone()))),
        ].into_iter().collect()
    }
}
