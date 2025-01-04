use std::cell::Cell;
use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::object::{get_object_class};
use crate::builtins::print::py_print;
use crate::builtins::pyfloat::get_float_class;
use crate::builtins::pyint::{get_int_class};
use crate::builtins::range::{get_range_class, get_range_iterator_class};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyobject::{ManyArgFuncType, PyInternalFunction, PyObject};

#[derive(Debug)]
pub struct Globals {
    pub object_class: Rc<PyClass>,
    pub int_class: Rc<PyClass>,
    pub float_class: Rc<PyClass>,
    pub range_class: Rc<PyClass>,
    pub range_iterator_class: Rc<PyClass>,
    pub print_func: Rc<PyInternalFunction>,
}

impl Globals {
    pub(crate) fn new() -> Globals {
        let object_class = Rc::new(get_object_class());
        let int_class = Rc::new(get_int_class(object_class.clone()));
        let float_class = Rc::new(get_float_class(object_class.clone()));
        
        let range_class = Rc::new(get_range_class(object_class.clone()));
        let range_iterator_class = Rc::new(get_range_iterator_class(object_class.clone()));
        
        Globals {
            object_class,
            int_class,
            float_class,
            range_class,
            range_iterator_class,
            print_func: Rc::new(PyInternalFunction::ManyArgFunc(&(py_print as ManyArgFuncType))),
        }
    }
    
    pub fn load_into_hashmap(&self) -> AHashMap<String, Cell<PyObject>> {
        vec![
            ("object".to_string(), Cell::new(PyObject::new_internal_class(self.object_class.clone()))),
            ("int".to_string(), Cell::new(PyObject::new_internal_class(self.int_class.clone()))),
            ("float".to_string(), Cell::new(PyObject::new_internal_class(self.float_class.clone()))),
            ("range".to_string(), Cell::new(PyObject::new_internal_class(self.range_class.clone()))),
            ("print".to_string(), Cell::new(PyObject::new_internal_func(self.print_func.clone()))),
        ].into_iter().collect()
    }
}
