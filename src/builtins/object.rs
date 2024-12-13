use crate::builtins::function_utils::call_function;
use crate::builtins::pyobjects::*;
use crate::builtins::pyobjects::PyInternalFunction::{InitFunc, NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

pub fn expect_class(pyobj: PyPointer<PyObject>) -> PyPointer<PyClass> {
    match **pyobj.borrow() {
        PyObject::Class(ref class) => class.clone(),
        _ => panic!("Expected class"),
    }
}

pub fn object__new__(pyclass: PyPointer<PyClass>, pyargs: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {
    if !pyclass.borrow().defines_attribute("__init__".to_string()) && pyargs.len() > 0 {
        panic!("TypeError: object.__new__() takes exactly one argument (the type to instantiate)");  // TODO make python error
    } 

    let pyself = PyPointer::new(PyObject::Instance(PyPointer::new(PyInstance::new(pyclass))));

    pyself
}

pub fn object__init__(pyself: PyPointer<PyObject>, pyargs: Vec<PyPointer<PyObject>>) {    
    let pyclass = pyself.borrow().get_class();
    
    if !pyclass.borrow().defines_attribute("__new__".to_string()) && pyargs.len() > 0 {
        panic!("TypeError: object.__init__() takes exactly one argument (the instance to initialize)");  // TODO make python error
    }
}

pub fn object__repr__(pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    PyPointer::new(PyObject::Str(format!("<{} object at {:p}>", pyself.borrow().get_class().borrow().get_name(), &pyself)))
}

pub fn object__str__(pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {  // by default make str call repr
    let str_func = pyself.borrow().get_attribute("__repr__".to_string()).unwrap();
    call_function(str_func, vec![pyself.clone()], &mut PyArena::new())
}


pub const py_object: PyClass = PyClass::Internal {
    name_func: || "object".to_string(), 
    super_classes_func: || vec![],
    methods: PyMagicMethods {

        __new__: Some(NewFunc(&(object__new__ as NewFuncType))),
        __init__: Some(InitFunc(&(object__init__ as InitFuncType))),

        __str__: Some(UnaryFunc(&(object__str__ as UnaryFuncType))),
        __repr__: Some(UnaryFunc(&(object__repr__ as UnaryFuncType))),

        ..py_magic_methods_defaults()
        
    }
};