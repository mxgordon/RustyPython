use crate::builtins::object::expect_class;
use crate::builtins::pyobjects::{PyClass, PyInternalFunction, PyObject, PyPointer};
use crate::pyarena::PyArena;

pub fn call_function(func: PyPointer<PyObject>, args: Vec<PyPointer<PyObject>>, arena: &mut PyArena) -> PyPointer<PyObject> {
    match **func.borrow() {
        PyObject::Function(ref _func) => {
            todo!()
        }
        PyObject::InternalSlot(ref func) => {
            eval_internal_func(func.clone(), args)
        }
        PyObject::Class(ref pyclass) => {
            eval_obj_init(pyclass.clone(), args, arena)
        }
        _ => {
            panic!("{:?} is not a function", func); // TODO Make python error
        }
    }
}

pub(crate) fn eval_internal_func(func: PyPointer<PyInternalFunction>, args: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {
    match (*func.borrow().clone(), args.len()) {  //TODO figure out if I don't need to clone this
        (PyInternalFunction::NewFunc(func), n) => {
            func(expect_class(args[0].clone()), args[1..n].to_vec())
        }
        (PyInternalFunction::InitFunc(func), n) => {
            func(args[0].clone(), args[1..n].to_vec());
            PyPointer::new(PyObject::None)  // init always returns None
        }
        (PyInternalFunction::UnaryFunc(func), 1) => {
            func(args[0].clone())
        }
        (PyInternalFunction::BivariateFunc(func), 2) => {
            func(args[0].clone(), args[1].clone())
        }
        (PyInternalFunction::ManyArgFunc(func), _n) => {
            func(args)
        }
        (internal_function_type, n) => {
            panic!("Trying to call {:?} function type with {} arguments", internal_function_type, n); // TODO Make python error
        }
    }
}


pub(crate) fn eval_obj_init(pyclass: PyPointer<PyClass>, args: Vec<PyPointer<PyObject>>, arena: &mut PyArena) -> PyPointer<PyObject> {
    let pyclass_borrow = pyclass.borrow();

    let new_func = pyclass_borrow.search_for_attribute("__new__".to_string());
    let init_func = pyclass_borrow.search_for_attribute("__init__".to_string());

    if new_func.is_none() {
        panic!("Class has no __new__ method"); // TODO Make python error
    } else if init_func.is_none() {
        panic!("Class has no __init__ method")
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

pub(crate) fn init_internal_class(pyclass: PyPointer<PyClass>, args: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {
    let pyclass_borrow = pyclass.borrow();

    let new_func = pyclass_borrow.search_for_attribute("__new__".to_string()).unwrap().borrow().expect_internal_slot();
    let init_func = pyclass_borrow.search_for_attribute("__init__".to_string()).unwrap().borrow().expect_internal_slot();

    let mut new_func_args = vec![PyPointer::new(PyObject::Class(pyclass.clone())) ];
    new_func_args.extend(args.clone());

    let new_object = eval_internal_func(new_func, new_func_args);

    let mut init_func_args = vec![new_object.clone()];
    init_func_args.extend(args.clone());

    eval_internal_func(init_func, init_func_args);

    new_object
}