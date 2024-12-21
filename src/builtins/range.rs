use once_cell::sync::Lazy;
use crate::builtins::function_utils::init_internal_class;
use crate::builtins::pyint::{expect_int, expect_int_ptr};
use crate::builtins::pyobjects::{py_magic_methods_defaults, InitFuncType, PyClass, PyFlag, PyMagicMethods, PyObject, PyPointer, UnaryFuncType};
use crate::builtins::pyobjects::PyInternalFunction::{InitFunc, UnaryFunc};

use crate::pyarena::PyArena;
pub static CURRENT_STRING: Lazy<String> = Lazy::new(|| "current".to_string());

pub const START_IDX: usize = 0;
pub const STOP_IDX: usize = 1;
pub const STEP_IDX: usize = 2;
pub const CURRENT_IDX: usize = 3;


pub fn range__init__(_arena: &mut PyArena, pyself: PyPointer<PyObject>, args: Vec<PyPointer<PyObject>>) {
    let first = expect_int_ptr(args.get(0).unwrap_or_else(|| panic!("Expected at least one arguments to __init__, received zero")).clone());
    let second = args.get(1);
    let third = args.get(2);
    
    let mut start: i64 = 0;
    let stop: i64;
    let mut step = 1;

    if let Some(second) = second {
        start = first;
        stop = expect_int_ptr(second.clone());

        if let Some(third) = third {
            step = expect_int_ptr(third.clone());
        }
    } else {
        stop = first;
    }


    let mut pyself_mut = pyself.borrow_mut();
    
    pyself_mut.set_attribute(&"start".to_string(), PyPointer::new(PyObject::Int(start)));
    pyself_mut.set_attribute(&"stop".to_string(), PyPointer::new(PyObject::Int(stop)));
    pyself_mut.set_attribute(&"step".to_string(), PyPointer::new(PyObject::Int(step)));
}

pub fn range__repr__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let pyself_borrow = pyself.borrow();
    let start = expect_int_ptr(pyself_borrow.get_attribute("start", arena).unwrap());
    let stop = expect_int_ptr(pyself_borrow.get_attribute("stop", arena).unwrap());
    let step = expect_int_ptr(pyself_borrow.get_attribute("step", arena).unwrap());
    
    if step == 1 {
        return PyPointer::new(PyObject::Str(format!("range({}, {})", start, stop)));
    }
    PyPointer::new(PyObject::Str(format!("range({}, {}, {})", start, stop, step)))
}

pub fn range__iter__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    init_internal_class(arena.globals.range_iterator_class.clone(), vec![pyself.clone()], arena)
}

pub fn get_range_class(object_class: PyPointer<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "range".to_string(),
        super_classes: vec![object_class],
        methods: PyMagicMethods {
            __init__: Some(PyPointer::new(InitFunc(&(range__init__ as InitFuncType)))),

            __repr__: Some(PyPointer::new(UnaryFunc(&(range__repr__ as UnaryFuncType)))),

            __iter__: Some(PyPointer::new(UnaryFunc(&(range__iter__ as UnaryFuncType)))),

            ..py_magic_methods_defaults()
        }
    }.create()
}

pub fn range_iterator__init__(arena: &mut PyArena, pyself: PyPointer<PyObject>, args: Vec<PyPointer<PyObject>>) {
    let range_obj = args.get(0).unwrap_or_else(|| panic!("Expected one argument to __init__, received zero"));
    assert_eq!(args.len(), 1);


    
    let range_obj = range_obj.borrow(); // TODO assert range type

    let start = expect_int_ptr(range_obj.get_attribute("start", arena).unwrap());  // Copy by value
    let stop = expect_int_ptr(range_obj.get_attribute("stop", arena).unwrap());
    let step = expect_int_ptr(range_obj.get_attribute("step", arena).unwrap());


    let pyself = pyself.borrow();
    let instance_ptr = pyself.expect_instance();
    let mut instance = instance_ptr.borrow_mut();

    instance.internal_storage.insert(START_IDX, PyObject::Int(start));
    instance.internal_storage.insert(STOP_IDX, PyObject::Int(stop));
    instance.internal_storage.insert(STEP_IDX, PyObject::Int(step));
    instance.internal_storage.insert(CURRENT_IDX, PyObject::Int(start));
}

pub fn range_iterator__next__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let mut pyself_mut = pyself.borrow_mut();
    let instance_ptr = pyself_mut.expect_instance();
    let mut instance = instance_ptr.borrow_mut();

    let mut current = expect_int(instance.internal_storage.get(CURRENT_IDX).unwrap().clone());
    let stop = expect_int(instance.internal_storage.get(STOP_IDX).unwrap().clone());
    let step = expect_int(instance.internal_storage.get(STEP_IDX).unwrap().clone());
    
    if current >= stop {
        return PyPointer::new(PyObject::InternalFlag(PyPointer::new(PyFlag::StopIteration)));  // StopIteration TODO remove internal pypointer
    }

    instance.internal_storage[CURRENT_IDX] = PyObject::Int(current + step);
    
    PyPointer::new(PyObject::Int(current))
}

pub fn get_range_iterator_class(object_class: PyPointer<PyClass>) -> PyClass {
    PyClass::Internal {  // Hidden class
        name: "range_iterator".to_string(),
        super_classes: vec![object_class],
        methods: PyMagicMethods {
            __init__: Some(PyPointer::new(InitFunc(&(range_iterator__init__ as InitFuncType)))),
            __next__: Some(PyPointer::new(UnaryFunc(&(range_iterator__next__ as UnaryFuncType)))),
            ..py_magic_methods_defaults()
        }
    }.create()
}
