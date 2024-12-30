use std::collections::HashMap;
use std::rc::Rc;
use crate::builtins::function_utils::call_function;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethod, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyinstance::PyInstance;
use crate::builtins::structure::pyobject::{InitFuncType, InitReturnType, FuncReturnType, NewFuncType, PyObject, PyPointer, UnaryFuncType};
use crate::builtins::structure::pyobject::PyInternalFunction::{InitFunc, NewFunc, UnaryFunc};
use crate::pyarena::PyArena;


pub fn expect_class(pyobj: PyPointer<PyObject>) -> Rc<PyClass> {
    match *pyobj.borrow() {
        PyObject::Class(ref class) => class.clone(),
        _ => panic!("Expected class"),
    }
}

pub fn object__new__(arena: &mut PyArena, pyclass: Rc<PyClass>, pyargs: Vec<PyPointer<PyObject>>) -> FuncReturnType {
    if !pyclass.defines_attribute(PyMagicMethod::Init) && pyargs.len() > 0 {
        return Err(arena.exceptions.type_error.instantiate("object() takes no arguments".to_string()));
    } 

    let pyself = PyPointer::new(PyObject::Instance(PyInstance::new_empty(pyclass)));

    Ok(pyself)
}

pub fn object__init__(arena: &mut PyArena, pyself: PyPointer<PyObject>, pyargs: Vec<PyPointer<PyObject>>) -> InitReturnType {
    let pyself = pyself.borrow();
    let pyclass = pyself.get_class(arena);
    
    if !pyclass.defines_attribute(PyMagicMethod::New) && pyargs.len() > 0 {
        return Err(arena.exceptions.type_error.instantiate("object.__init__() takes exactly one argument (the instance to initialize)".to_string()));
    }
    
    Ok(())
}

pub fn object__repr__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> FuncReturnType {
    Ok(PyPointer::new(PyObject::Str(format!("<{} object at {:p}>", pyself.borrow().get_class(arena).get_name(), &pyself))))
}

pub fn object__str__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> FuncReturnType {
    let str_func = pyself.borrow().get_magic_method(PyMagicMethod::Repr, arena).expect("__repr__ should always be defined");
    call_function(str_func, vec![pyself.clone()], arena)
}

pub fn get_object_class() -> PyClass {
    PyClass::Internal {
        name: "object".to_string(),
        super_classes: vec![],
        attributes: HashMap::new(),
        magic_methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(object__new__ as NewFuncType)))),
            __init__: Some(Rc::new(InitFunc(&(object__init__ as InitFuncType)))),

            __str__: Some(Rc::new(UnaryFunc(&(object__str__ as UnaryFuncType)))),
            __repr__: Some(Rc::new(UnaryFunc(&(object__repr__ as UnaryFuncType)))),

            ..py_magic_methods_defaults()
        }
    }.create()
}