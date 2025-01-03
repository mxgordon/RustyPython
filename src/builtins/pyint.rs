use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{BivariateFuncType, FuncReturnType, NewFuncType, PyImmutableObject, PyObject, UnaryFuncType};
use crate::builtins::structure::pyobject::PyInternalFunction::{BivariateFunc, NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

pub fn expect_int(pyobj: &PyObject, arena: &mut PyArena) -> Result<i64, PyException> {
    match **pyobj.expect_immutable() {
        PyImmutableObject::Int(ref value) => {Ok(*value)}
        ref value => {
            let message = format!("'{}' object cannot be interpreted as an integer", value.get_class(arena).get_name());
            Err(arena.exceptions.type_error.instantiate(message))
        },
    }
}

pub fn int__new__(arena: &mut PyArena, _pyclass: Rc<PyClass>, pyargs: &[PyObject]) -> FuncReturnType {  // error handling
    let value = pyargs.first().unwrap();
    
    let new_value = match **value.expect_immutable() {  // cast value
        PyImmutableObject::Int(ref value) => *value,  // copy the value
        PyImmutableObject::Float(ref value) => *value as i64,
        PyImmutableObject::Str(ref value) => value.parse::<i64>().unwrap(),
        ref value => { 
            let message = format!("int() argument must be a string, a bytes-like object or a real number, not '{}'", value.get_class(arena).get_name());
            Err(arena.exceptions.type_error.instantiate(message))? 
        },
    };
    
    Ok(PyObject::new_int(new_value))  // I don't know how to do inheritance with this
}

pub fn int__add__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let self_value = expect_int(&pyself, arena)?;
    let other_value = expect_int(&other, arena)?;  // TODO make this work for other types (float)
    
    Ok(PyObject::new_int(self_value + other_value))
}

pub fn int__mul__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let self_value = expect_int(&pyself, arena)?;
    let other_value = expect_int(&other, arena)?;  // TODO make this work for other types (float)

    Ok(PyObject::new_int(self_value * other_value))
}

pub fn int__truediv__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let self_value = expect_int(&pyself, arena)?;
    let other_value = expect_int(&other, arena)?;  // TODO make this work for other types (float)

    Ok(PyObject::new_float(self_value as f64 / other_value as f64))
}

pub fn int__pow__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let self_value = expect_int(&pyself, arena)?;
    let other_value = expect_int(&other, arena)?;  // TODO make this work for other types (float)

    Ok(PyObject::new_int(self_value.pow(other_value as u32)))
}

pub fn int__repr__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let value = expect_int(&pyself, arena)?;
    Ok(PyObject::new_string(value.to_string()))
}

pub fn get_int_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "int".to_string(),
        super_classes: vec![object_class],
        attributes: AHashMap::new(),
        magic_methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(int__new__ as NewFuncType)))),

            __repr__: Some(Rc::new(UnaryFunc(&(int__repr__ as UnaryFuncType)))),
            
            __add__: Some(Rc::new(BivariateFunc(&(int__add__ as BivariateFuncType)))),
            __mul__: Some(Rc::new(BivariateFunc(&(int__mul__ as BivariateFuncType)))),
            __truediv__: Some(Rc::new(BivariateFunc(&(int__truediv__ as BivariateFuncType)))),
            __pow__: Some(Rc::new(BivariateFunc(&(int__pow__ as BivariateFuncType)))),

            ..py_magic_methods_defaults()
        }
    }.create()
}

