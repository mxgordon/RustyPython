use std::ops::Deref;
use std::rc::Rc;
use crate::builtins::object::expect_class;
use crate::builtins::structure::magic_methods::PyMagicMethod;
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyobject::{FuncReturnType, PyImmutableObject, PyInternalFunction, PyMutableObject, PyObject};
use crate::pyarena::PyArena;

pub fn call_function(func: PyObject, args: &[PyObject], arena: &mut PyArena) -> FuncReturnType {
    match func {
        PyObject::Immutable(inner) => {
            match &*inner {
                PyImmutableObject::InternalSlot(func) => eval_internal_func(func.clone(), args, arena),
                PyImmutableObject::InternalClass(pyclass) => eval_obj_init(pyclass.clone(), args, arena),
                _ => {panic!("Immutable object is not a function")}
            }
        }
        PyObject::Mutable(inner) => {
            match &*inner.borrow() {
                PyMutableObject::Function(_) => {todo!()}
                _ => {panic!("Mutable object is not a function")}
            }
        }
        PyObject::IteratorFlag(_) => {panic!("IteratorFlag is not a function")}
    }
}

pub(crate) fn eval_internal_func(func: Rc<PyInternalFunction>, args: &[PyObject], arena: &mut PyArena) -> FuncReturnType {
    match (func.deref(), args.len()) {
        (PyInternalFunction::NewFunc(func), n) => {
            func(arena, expect_class(&args[0]), &args[1..n])  // TODO find a way to not clone the class
        }
        (PyInternalFunction::InitFunc(func), n) => {
            func(arena, &args[0], &args[1..n])?;
            Ok(PyObject::none())  // init always returns None
        }
        (PyInternalFunction::UnaryFunc(func), 1) => {
            func(arena, &args[0])
        }
        (PyInternalFunction::BivariateFunc(func), 2) => {
            func(arena, &args[0], &args[1])
        }
        (PyInternalFunction::ManyArgFunc(func), _n) => {
            func(arena, args)
        }
        (internal_function_type, n) => {
            panic!("Trying to call {:?} function type with {} arguments", internal_function_type, n); // TODO Make python error
        }
    }
}


pub(crate) fn eval_obj_init(pyclass: Rc<PyClass>, args: &[PyObject], arena: &mut PyArena) -> FuncReturnType {
    let new_func = pyclass.search_for_magic_method(PyMagicMethod::New);
    let init_func = pyclass.search_for_magic_method(PyMagicMethod::Init);

    if new_func.is_none() {
        panic!("{:?} has no __new__ method", pyclass); // TODO Make python error
    } else if init_func.is_none() {
        panic!("{:?} has no __init__ method", pyclass)
    }

    let new_func = new_func.unwrap();
    let init_func = init_func.unwrap();

    let new_args = [&[PyObject::new_internal_class(pyclass)], args].concat();

    let new_object = call_function(new_func, &new_args, arena)?;

    let init_args = [&[new_object.clone()], args].concat();

    let _init_rtn = call_function(init_func, &init_args, arena)?; // TODO check if init_rtn is None

    Ok(new_object)
}

pub(crate) fn init_internal_class(pyclass: Rc<PyClass>, args: &[PyObject], arena: &mut PyArena) -> FuncReturnType {
    let new_func = pyclass.get_magic_method_internal(PyMagicMethod::New).unwrap();
    let init_func = pyclass.get_magic_method_internal(PyMagicMethod::Init).unwrap();

    let new_func_args = [&[PyObject::new_internal_class(pyclass)], args].concat();

    let new_object = eval_internal_func(new_func, &new_func_args, arena)?;

    let init_func_args = [&[new_object.clone()], args].concat();

    eval_internal_func(init_func, &init_func_args, arena)?;

    Ok(new_object)
}