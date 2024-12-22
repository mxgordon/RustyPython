use std::ops::Deref;
use std::rc::Rc;
use crate::builtins::object::expect_class;
use crate::builtins::pyobjects::{PyClass, PyInternalFunction, PyMagicMethod, PyObject, PyPointer};
use crate::pyarena::PyArena;

pub fn call_function(func: PyPointer<PyObject>, args: Vec<PyPointer<PyObject>>, arena: &mut PyArena) -> PyPointer<PyObject> {
    match *func.borrow() {
        PyObject::Function(ref _func) => {
            todo!()
        }
        PyObject::InternalSlot(ref func) => {
            eval_internal_func(func.clone(), args, arena)
        }
        PyObject::Class(ref pyclass) => {
            eval_obj_init(pyclass.clone(), args, arena)
        }
        _ => {
            panic!("{:?} is not a function", func); // TODO Make python error
        }
    }
}

pub(crate) fn eval_internal_func(func: Rc<PyInternalFunction>, args: Vec<PyPointer<PyObject>>, arena: &mut PyArena) -> PyPointer<PyObject> {
    match (func.deref(), args.len()) {  //TODO figure out if I don't need to clone this
        (PyInternalFunction::NewFunc(func), n) => {
            func(arena, expect_class(args[0].clone()), args[1..n].to_vec())
        }
        (PyInternalFunction::InitFunc(func), n) => {
            func(arena, args[0].clone(), args[1..n].to_vec());
            PyPointer::new(PyObject::None)  // init always returns None
        }
        (PyInternalFunction::UnaryFunc(func), 1) => {
            func(arena, args[0].clone())
        }
        (PyInternalFunction::BivariateFunc(func), 2) => {
            func(arena, args[0].clone(), args[1].clone())
        }
        (PyInternalFunction::ManyArgFunc(func), _n) => {
            func(arena, args)
        }
        (internal_function_type, n) => {
            panic!("Trying to call {:?} function type with {} arguments", internal_function_type, n); // TODO Make python error
        }
    }
}


pub(crate) fn eval_obj_init(pyclass: PyPointer<PyClass>, args: Vec<PyPointer<PyObject>>, arena: &mut PyArena) -> PyPointer<PyObject> {
    let pyclass_borrow = pyclass.borrow();

    let new_func = pyclass_borrow.search_for_attribute(PyMagicMethod::New);
    let init_func = pyclass_borrow.search_for_attribute(PyMagicMethod::Init);

    if new_func.is_none() {
        panic!("{:?} has no __new__ method", pyclass_borrow); // TODO Make python error
    } else if init_func.is_none() {
        panic!("{:?} has no __init__ method", pyclass_borrow)
    }

    let new_func = new_func.unwrap();
    let init_func = init_func.unwrap();

    let mut new_args = vec![PyPointer::new(PyObject::Class(pyclass.clone())) ];
    new_args.extend(args.clone());

    let new_object = call_function(new_func, new_args, arena);

    let mut init_args = vec![new_object.clone()];
    init_args.extend(args.clone());

    let _init_rtn = call_function(init_func, init_args, arena); // TODO check if init_rtn is None

    new_object
}

pub(crate) fn init_internal_class(pyclass: PyPointer<PyClass>, args: Vec<PyPointer<PyObject>>, arena: &mut PyArena) -> PyPointer<PyObject> {
    let pyclass_borrow = pyclass.borrow();

    let new_func = pyclass_borrow.get_magic_method_internal(PyMagicMethod::New).unwrap();
    let init_func = pyclass_borrow.get_magic_method_internal(PyMagicMethod::Init).unwrap();

    let mut new_func_args = vec![PyPointer::new(PyObject::Class(pyclass.clone())) ];
    new_func_args.extend(args.clone());

    let new_object = eval_internal_func(new_func, new_func_args, arena);

    let mut init_func_args = vec![new_object.clone()];
    init_func_args.extend(args.clone());

    eval_internal_func(init_func, init_func_args, arena);

    new_object
}