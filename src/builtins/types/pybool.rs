#![allow(non_snake_case)]
use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::function_utils::call_function_1_arg_min;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethod, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{FuncReturnType, NewFuncType, PyImmutableObject, PyMutableObject, PyObject, UnaryFuncType};
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


fn convert_mutable_to_bool(pyobj: &PyObject, mutable_obj: &PyMutableObject, arena: &mut PyArena ) -> Result<bool, PyException> {
    let bool_func = mutable_obj.get_magic_method(&PyMagicMethod::Bool, arena);

    if let Some(bool_func) = bool_func {
        let func_result = call_function_1_arg_min(&bool_func, pyobj, &[], arena)?;

        let bool_result = expect_bool(&func_result, arena);

        return bool_result.map_err(|_error| {
            let message = format!("{}.__bool__ should return bool, returned (type {{<other type>}})", pyobj.clone_class(arena).get_name());  // TODO inconsistent python error messaging?
            arena.exceptions.type_error.instantiate(message)
        });
    }

    let message = format!("bool() argument must be a string, a bytes-like object or a real number, not '{}'", pyobj.clone_class(arena).get_name());
    Err(arena.exceptions.type_error.instantiate(message))
}

fn convert_immutable_to_bool(immutable_obj: &PyImmutableObject) -> Result<bool, PyException> {
    match *immutable_obj {
        PyImmutableObject::Bool(ref value) => Ok(*value),
        PyImmutableObject::Int(ref value) => Ok(*value != 0),  // copy the value
        PyImmutableObject::Float(ref value) => Ok(*value != 0.0),
        PyImmutableObject::Str(ref value) => Ok(!value.is_empty()),
        PyImmutableObject::None => Ok(false)
    }
}

pub fn convert_pyobj_to_bool(pyobj: &PyObject, arena: &mut PyArena) -> Result<bool, PyException> {
    match *pyobj {
        PyObject::Immutable(ref immutable) => convert_immutable_to_bool(immutable),
        PyObject::Mutable(ref mutable) => convert_mutable_to_bool(pyobj, &mutable.borrow(), arena),
        PyObject::Internal(_) => {todo!()}
        PyObject::IteratorFlag(_) => {panic!()}
    }
}


pub fn bool__new__(arena: &mut PyArena, _pyclass: Rc<PyClass>, pyargs: &[PyObject]) -> FuncReturnType {  // error handling
    let arg = pyargs.first();
    let value;
    
    if let Some(arg) = arg {
        value = convert_pyobj_to_bool(arg, arena)?;
    } else {
        value = false;
    }
    
    Ok(arena.statics.get_bool(value).clone())
}


pub fn bool__repr__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let value = expect_bool(&pyself, arena)?;
    Ok(PyObject::new_string(if value { "True".to_string() } else {"False".to_string()}))
}

pub fn bool__bool__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let value = expect_bool(&pyself, arena)?;
    Ok(arena.statics.get_bool(value).clone())
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

