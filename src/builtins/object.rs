use crate::builtins::function_utils::call_function;
use crate::builtins::pyobjects::*;
use crate::builtins::pyobjects::PyInternalFunction::{InitFunc, NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

pub fn expect_class(pyobj: PyPointer<PyObject>) -> PyPointer<PyClass> {
    match *pyobj.borrow() {
        PyObject::Class(ref class) => class.clone(),
        _ => panic!("Expected class"),
    }
}

pub fn object__new__(_arena: &mut PyArena, pyclass: PyPointer<PyClass>, pyargs: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {
    if !pyclass.borrow().defines_attribute(PyMagicMethod::Init) && pyargs.len() > 0 {
        panic!("TypeError: object.__new__() takes exactly one argument (the type to instantiate)");  // TODO make python error
    } 

    let pyself = PyPointer::new(PyObject::Instance(PyPointer::new(PyInstance::new(pyclass))));

    pyself
}

pub fn object__init__(arena: &mut PyArena, pyself: PyPointer<PyObject>, pyargs: Vec<PyPointer<PyObject>>) {
    let pyclass = pyself.borrow().get_class(arena);
    
    if !pyclass.borrow().defines_attribute(PyMagicMethod::New) && pyargs.len() > 0 {
        panic!("TypeError: object.__init__() takes exactly one argument (the instance to initialize)");  // TODO make python error
    }
}

pub fn object__repr__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    PyPointer::new(PyObject::Str(format!("<{} object at {:p}>", pyself.borrow().get_class(arena).borrow().get_name(), &pyself)))
}

pub fn object__str__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {  // by default make str call repr
    let str_func = pyself.borrow().get_attribute("__repr__", arena).unwrap();
    call_function(str_func, vec![pyself.clone()], &mut PyArena::new())
}

pub fn get_object_class() -> PyClass {
    PyClass::Internal {
        name: "object".to_string(),
        super_classes: vec![],
        methods: PyMagicMethods {
            __new__: Some(PyPointer::new(NewFunc(&(object__new__ as NewFuncType)))),
            __init__: Some(PyPointer::new(InitFunc(&(object__init__ as InitFuncType)))),

            __str__: Some(PyPointer::new(UnaryFunc(&(object__str__ as UnaryFuncType)))),
            __repr__: Some(PyPointer::new(UnaryFunc(&(object__repr__ as UnaryFuncType)))),

            ..py_magic_methods_defaults()
        }
    }.create()
}