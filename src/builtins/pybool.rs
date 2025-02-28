#![allow(non_snake_case)]
use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::function_utils::call_function_1_arg_min;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethod, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{FuncReturnType, NewFuncType, PyImmutableObject, PyObject, UnaryFuncType};
use crate::builtins::structure::pyobject::PyInternalFunction::{NewFunc, UnaryFunc};
use crate::pyarena::PyArena;


pub fn expect_bool(pyobj: &PyObject, arena: &mut PyArena) -> Result<bool, PyException> {
    match **pyobj.expect_immutable() {
        PyImmutableObject::Bool(ref value) => {Ok(*value)}
        ref value => {
            let message = format!("'{}' object cannot be interpreted as a boolean", value.get_class(arena).get_name());
            Err(arena.exceptions.type_error.instantiate(message))
        },
    }
}


pub fn bool__new__(arena: &mut PyArena, _pyclass: Rc<PyClass>, pyargs: &[PyObject]) -> FuncReturnType {  // error handling
    let value = pyargs.first().unwrap();
    let bool_magic_method = value.expect_immutable().get_magic_method(&PyMagicMethod::Bool, arena);
    let bool_magic_method = bool_magic_method.unwrap_or_else(|| panic!("Object has no __str__ method"));
    let py_bool_value = call_function_1_arg_min(bool_magic_method, value, &[], arena)?;
    
    let _bool_value = expect_bool(&py_bool_value, arena)?;
    
    Ok(py_bool_value)
}


pub fn bool__repr__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let value = expect_bool(&pyself, arena)?;
    Ok(PyObject::new_string(if value { "True".to_string() } else {"False".to_string()}))
}

pub fn bool__bool__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let value = expect_bool(&pyself, arena)?;
    Ok(PyObject::new_bool(value))
}

pub fn bool__int__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let value = expect_bool(&pyself, arena)?;
    Ok(PyObject::new_int(value as i64))
}



pub fn get_bool_class(int_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "bool".to_string(),
        super_classes: vec![int_class],
        attributes: AHashMap::new(),
        magic_methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(bool__new__ as NewFuncType)))),

            __repr__: Some(Rc::new(UnaryFunc(&(bool__repr__ as UnaryFuncType)))),
            
            __bool__: Some(Rc::new(UnaryFunc(&(bool__bool__ as UnaryFuncType)))),
            __int__: Some(Rc::new(UnaryFunc(&(bool__int__ as UnaryFuncType)))),

            ..py_magic_methods_defaults()
        }
    }.create()
}

