use std::collections::HashMap;
use std::rc::Rc;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{BivariateFuncType, FuncReturnType, NewFuncType, PyObject, PyPointer, UnaryFuncType};
use crate::builtins::structure::pyobject::PyInternalFunction::{BivariateFunc, NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

pub fn expect_int(pyobj: &PyObject, arena: &mut PyArena) -> Result<i64, PyException> {
    match pyobj {
        PyObject::Int(value) => Ok(*value),
        PyObject::Bool(value) => Ok(if *value { 1 } else { 0 }),
        value => {
            let message = format!("'{}' object cannot be interpreted as an integer", value.get_class(arena).get_name());
            Err(arena.exceptions.type_error.instantiate(message))
        },
    }
}

pub fn expect_int_ptr(pyobj: PyPointer<PyObject>, arena: &mut PyArena) -> Result<i64, PyException> {
    expect_int(&*pyobj.borrow(), arena)
}

pub fn int__new__(arena: &mut PyArena, _pyclass: Rc<PyClass>, pyargs: Vec<PyPointer<PyObject>>) -> FuncReturnType {  // error handling
    let value = pyargs.first().unwrap();
    
    let new_value = match *value.borrow() {  // cast value
        PyObject::Int(ref value) => *value,  // copy the value
        PyObject::Float(ref value) => *value as i64,
        PyObject::Str(ref value) => value.parse::<i64>().unwrap(),
        ref value => { 
            let message = format!("int() argument must be a string, a bytes-like object or a real number, not '{}'", value.get_class(arena).get_name());
            Err(arena.exceptions.type_error.instantiate(message))? 
        },
    };
    
    Ok(PyPointer::new(PyObject::Int(new_value)))  // I don't know how to do inheritance with this
}

pub fn int__add__(arena: &mut PyArena, pyself: PyPointer<PyObject>, other: PyPointer<PyObject>) -> FuncReturnType {
    let self_value = expect_int_ptr(pyself, arena)?;
    let other_value = expect_int_ptr(other, arena)?;  // TODO make this work for other types (float)
    
    Ok(PyPointer::new(PyObject::Int(self_value + other_value)))
}

pub fn int__mul__(arena: &mut PyArena, pyself: PyPointer<PyObject>, other: PyPointer<PyObject>) -> FuncReturnType {
    let self_value = expect_int_ptr(pyself, arena)?;
    let other_value = expect_int_ptr(other, arena)?;  // TODO make this work for other types (float)
    
    Ok(PyPointer::new(PyObject::Int(self_value * other_value)))
}

pub fn int__truediv__(arena: &mut PyArena, pyself: PyPointer<PyObject>, other: PyPointer<PyObject>) -> FuncReturnType {
    let self_value = expect_int_ptr(pyself, arena)?;
    let other_value = expect_int_ptr(other, arena)?;  // TODO make this work for other types (float)
    
    Ok(PyPointer::new(PyObject::Float(self_value as f64 / other_value as f64)))
}

pub fn int__pow__(arena: &mut PyArena, pyself: PyPointer<PyObject>, other: PyPointer<PyObject>) -> FuncReturnType {
    let self_value = expect_int_ptr(pyself, arena)?;
    let other_value = expect_int_ptr(other, arena)?;  // TODO make this work for other types (float)

    Ok(PyPointer::new(PyObject::Int(self_value.pow(other_value as u32))))
}

pub fn int__repr__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> FuncReturnType {
    let value = expect_int_ptr(pyself, arena)?;
    Ok(PyPointer::new(PyObject::Str(value.to_string())))
}
pub fn get_int_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "int".to_string(),
        super_classes: vec![object_class],
        attributes: HashMap::new(),
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

