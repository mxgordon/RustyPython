#![allow(non_snake_case)]
use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::function_utils::call_function;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethod, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyinstance::PyInstance;
use crate::builtins::structure::pyobject::{InitFuncType, EmptyFuncReturnType, FuncReturnType, NewFuncType, UnaryFuncType, PyObject, PyMutableObject, PyImmutableObject, PyInternalObject};
use crate::builtins::structure::pyobject::PyInternalFunction::{InitFunc, NewFunc, UnaryFunc};
use crate::parser::Define;
use crate::pyarena::PyArena;


pub fn expect_class(pyobj: &PyObject) -> Rc<PyClass> {
    match pyobj.expect_internal() {
        PyInternalObject::InternalClass(class) => class.clone(),
        _ => panic!("Expected class"),
    }
}

pub fn object__new__(arena: &mut PyArena, pyclass: Rc<PyClass>, pyargs: &[PyObject]) -> FuncReturnType {
    if !pyclass.defines_attribute(PyMagicMethod::Init) && pyargs.len() > 0 {
        return Err(arena.exceptions.type_error.instantiate("object() takes no arguments".to_string()));
    } 

    let pyself = PyObject::new_mutable(PyMutableObject::Instance(PyInstance::new_empty(pyclass)));

    Ok(pyself)
}

pub fn object__init__(arena: &mut PyArena, pyself: &PyObject, pyargs: &[PyObject]) -> EmptyFuncReturnType {
    let pyclass = pyself.clone_class(arena);

    if !pyclass.defines_attribute(PyMagicMethod::New) && pyargs.len() > 0 {
        return Err(arena.exceptions.type_error.instantiate("object.__init__() takes exactly one argument (the instance to initialize)".to_string()));
    }

    Ok(())
}

pub fn object__repr__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    Ok(PyObject::new_immutable(PyImmutableObject::Str(format!("<{} object at {:p}>", pyself.clone_class(arena).get_name(), &pyself))))
}

pub fn object__str__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let str_func = pyself.get_magic_method(&PyMagicMethod::Repr, arena).expect("__repr__ should always be defined");
    call_function(str_func, &[pyself.clone()], arena)
}

pub fn get_object_class() -> PyClass {
    PyClass::Internal {
        name: "object".to_string(),
        super_classes: vec![],
        attributes: AHashMap::new(),
        magic_methods: Box::new(PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(object__new__ as NewFuncType)))),
            __init__: Some(Rc::new(InitFunc(&(object__init__ as InitFuncType)))),

            __str__: Some(Rc::new(UnaryFunc(&(object__str__ as UnaryFuncType)))),
            __repr__: Some(Rc::new(UnaryFunc(&(object__repr__ as UnaryFuncType)))),

            ..Default::default()
        })
    }.create()
}