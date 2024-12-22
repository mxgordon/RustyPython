use std::collections::HashMap;
use std::rc::Rc;
use crate::builtins::object::{get_object_class};
use crate::builtins::print::py_print;
use crate::builtins::pyint::{get_int_class};
use crate::builtins::pyobjects::{ManyArgFuncType, PyClass, PyInternalFunction, PyObject, PyPointer};
use crate::builtins::range::{get_range_class, get_range_iterator_class};

#[derive(Debug)]
pub struct Globals {
    pub object_class: PyPointer<PyClass>,
    pub int_class: PyPointer<PyClass>,
    pub range_class: PyPointer<PyClass>,
    pub range_iterator_class: PyPointer<PyClass>,
    pub print_func: Rc<PyInternalFunction>,
}

impl Globals {
    pub(crate) fn new() -> Globals {
        let object_class = PyPointer::new(get_object_class());
        let int_class = PyPointer::new(get_int_class(object_class.clone()));
        
        let range_class = PyPointer::new(get_range_class(object_class.clone()));
        let range_iterator_class = PyPointer::new(get_range_iterator_class(object_class.clone()));
        
        Globals {
            object_class,
            int_class,
            range_class,
            range_iterator_class,
            print_func: Rc::new(PyInternalFunction::ManyArgFunc(&(py_print as ManyArgFuncType))),
        }
    }
    
    pub fn load_into_hashmap(&self) -> HashMap<String, PyPointer<PyObject>> {
        vec![
            ("object".to_string(), PyPointer::new(PyObject::Class(self.object_class.clone()))),
            ("int".to_string(), PyPointer::new(PyObject::Class(self.int_class.clone()))),
            ("range".to_string(), PyPointer::new(PyObject::Class(self.range_class.clone()))),
            ("print".to_string(), PyPointer::new(PyObject::InternalSlot(self.print_func.clone()))),
        ].into_iter().collect()
    }
}
