#![allow(non_snake_case)]
use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::function_utils::call_function_1_arg_min;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethod, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{BivariateFuncType, FuncReturnType, NewFuncType, PyImmutableObject, PyMutableObject, PyObject, UnaryFuncType};
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

pub fn expect_int_promotion(pyobj: &PyObject, arena: &mut PyArena) -> Result<i64, PyException> {
    match **pyobj.expect_immutable() {
        PyImmutableObject::Int(ref value) => {Ok(*value)}
        PyImmutableObject::Bool(ref value) => {Ok(*value as i64)}
        ref value => {
            Err(arena.exceptions.not_implemented_error.empty())
        },
    }
}

pub fn convert_mutable_to_int(pyobj: &PyObject, mutable_obj: &PyMutableObject, arena: &mut PyArena ) -> Result<i64, PyException> {
    let int_func = mutable_obj.get_magic_method(&PyMagicMethod::Int, arena);

    if let Some(int_func) = int_func {
        let func_result = call_function_1_arg_min(int_func, pyobj, &[], arena)?;

        let int_result = expect_int(&func_result, arena);

        return int_result.map_err(|error| {
            let message = format!("{}.__int__ returned non-int (type {{<other type>}})", pyobj.clone_class(arena).get_name());
            arena.exceptions.type_error.instantiate(message)
        });
    }

    let message = format!("int() argument must be a string, a bytes-like object or a real number, not '{}'", pyobj.clone_class(arena).get_name());
    Err(arena.exceptions.type_error.instantiate(message))
}

pub fn convert_immutable_to_int(immutable_obj: &PyImmutableObject, arena: &mut PyArena ) -> Result<i64, PyException> {
    match *immutable_obj {
        PyImmutableObject::Int(ref value) => Ok(*value),  // copy the value
        PyImmutableObject::Bool(ref value) => Ok(if *value { 1 } else { 0 }),
        PyImmutableObject::Float(ref value) => Ok(*value as i64),
        PyImmutableObject::Str(ref value) => value.parse::<i64>().map_err(|_error| {
            let message = format!("invalid literal for int() with base 10: '{}'", value);  // TODO add support for different bases
            arena.exceptions.type_error.instantiate(message)
        }),
        ref value => {
            let message = format!("float() argument must be a string, a bytes-like object or a real number, not '{}'", value.get_class(arena).get_name());
            Err(arena.exceptions.type_error.instantiate(message))
        },
    }
}

pub fn parse_int_op_func_params(pyself: &PyObject, other: &PyObject, arena: &mut PyArena) -> Result<(i64, i64), PyException> {
    let self_value = expect_int_promotion(pyself, arena)?; // this is done for easy inheritance to bool
    let other_value = expect_int_promotion(other, arena)?;
    Ok((self_value, other_value))
}

pub fn int__new__(arena: &mut PyArena, _pyclass: Rc<PyClass>, pyargs: &[PyObject]) -> FuncReturnType {  // error handling
    let value = pyargs.first();
    let new_value;

    if let Some(value) = value {
        new_value = match value {
            PyObject::Immutable(immutable) => convert_immutable_to_int(immutable, arena)?,
            PyObject::Mutable(mutable) => convert_mutable_to_int(value, &mutable.borrow(), arena)?,
            value => {
                let message = format!("int() argument must be a string, a bytes-like object or a real number, not '{}'", value.clone_class(arena).get_name());
                return Err(arena.exceptions.type_error.instantiate(message))
            },
        }
    } else {
        new_value = 0;
    }

    Ok(PyObject::new_int(new_value))  // I don't know how to do inheritance with this
}

pub fn int__add__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;
    
    Ok(PyObject::new_int(self_value + other_value))
}

pub fn int__sub__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;
    
    Ok(PyObject::new_int(self_value - other_value))
}

pub fn int__rsub__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;
    
    Ok(PyObject::new_int(other_value - self_value))
}

pub fn int__mul__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    Ok(PyObject::new_int(self_value * other_value))
}

pub fn int__truediv__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    Ok(PyObject::new_float(self_value as f64 / other_value as f64))
}

pub fn int__rtruediv__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    Ok(PyObject::new_float(other_value as f64 / self_value as f64))
}

pub fn int__pow__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;
    
    if other_value < 0 {
        return Ok(PyObject::new_float((self_value as f64).powf(other_value as f64)));
    }

    Ok(PyObject::new_int(self_value.pow(other_value as u32)))
}

pub fn int__rpow__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    if other_value < 0 {
        return Ok(PyObject::new_float((other_value as f64).powf(self_value as f64)));
    }

    Ok(PyObject::new_int(other_value.pow(self_value as u32)))
}

pub fn int__repr__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let value = expect_int(&pyself, arena)?;
    Ok(PyObject::new_string(value.to_string()))
}


pub fn int__eq__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    Ok(arena.statics.get_bool(self_value == other_value).clone())
}


pub fn int__gt__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    Ok(arena.statics.get_bool(self_value > other_value).clone())
}


pub fn int__lt__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    Ok(arena.statics.get_bool(self_value < other_value).clone())
}


pub fn int__ge__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    Ok(arena.statics.get_bool(self_value >= other_value).clone())
}


pub fn int__le__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    Ok(arena.statics.get_bool(self_value <= other_value).clone())
}


pub fn int__ne__(arena: &mut PyArena, pyself: &PyObject, other: &PyObject) -> FuncReturnType {
    let (self_value, other_value) = parse_int_op_func_params(pyself, other, arena)?;

    Ok(arena.statics.get_bool(self_value != other_value).clone())
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
            __radd__: Some(Rc::new(BivariateFunc(&(int__add__ as BivariateFuncType)))),
            __sub__: Some(Rc::new(BivariateFunc(&(int__sub__ as BivariateFuncType)))),
            __rsub__: Some(Rc::new(BivariateFunc(&(int__rsub__ as BivariateFuncType)))),
            __mul__: Some(Rc::new(BivariateFunc(&(int__mul__ as BivariateFuncType)))),
            __rmul__: Some(Rc::new(BivariateFunc(&(int__mul__ as BivariateFuncType)))),
            __truediv__: Some(Rc::new(BivariateFunc(&(int__truediv__ as BivariateFuncType)))),
            __rtruediv__: Some(Rc::new(BivariateFunc(&(int__rtruediv__ as BivariateFuncType)))),
            __pow__: Some(Rc::new(BivariateFunc(&(int__pow__ as BivariateFuncType)))),
            __rpow__: Some(Rc::new(BivariateFunc(&(int__rpow__ as BivariateFuncType)))),
            
            __eq__: Some(Rc::new(BivariateFunc(&(int__eq__ as BivariateFuncType)))),
            __ge__: Some(Rc::new(BivariateFunc(&(int__ge__ as BivariateFuncType)))),
            __le__: Some(Rc::new(BivariateFunc(&(int__le__ as BivariateFuncType)))),
            __gt__: Some(Rc::new(BivariateFunc(&(int__gt__ as BivariateFuncType)))),
            __lt__: Some(Rc::new(BivariateFunc(&(int__lt__ as BivariateFuncType)))),
            __ne__: Some(Rc::new(BivariateFunc(&(int__ne__ as BivariateFuncType)))),

            ..py_magic_methods_defaults()
        }
    }.create()
}

