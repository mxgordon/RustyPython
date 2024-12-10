use crate::builtins::function_utils::init_internal_class;
use crate::builtins::object::{py_object};
use crate::builtins::pyint::expect_int;
use crate::builtins::pyobjects::{InitFuncType, PyClass, PyFlag, PyObject, PyPointer, UnaryFuncType};
use crate::builtins::pyobjects::PyInternalFunction::{InitFunc, UnaryFunc};

pub fn range__init__(pyself: PyPointer<PyObject>, args: Vec<PyPointer<PyObject>>) {
    let first = expect_int(args.get(0).unwrap_or_else(|| panic!("Expected at least one arguments to __init__, received zero")).clone());
    let second = args.get(1);
    let third = args.get(2);
    
    let mut start: i64 = 0;
    let stop: i64;
    let mut step = 1;

    if let Some(second) = second {
        start = first;
        stop = expect_int(second.clone());

        if let Some(third) = third {
            step = expect_int(third.clone());
        }
    } else {
        stop = first;
    }


    let mut pyself_mut = pyself.borrow_mut();
    
    pyself_mut.set_attribute("start".to_string(), PyPointer::new(PyObject::Int(start)));
    pyself_mut.set_attribute("stop".to_string(), PyPointer::new(PyObject::Int(stop)));
    pyself_mut.set_attribute("step".to_string(), PyPointer::new(PyObject::Int(step)));
}

pub fn range__repr__(pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let pyself_borrow = pyself.borrow();
    let start = expect_int(pyself_borrow.get_attribute("start".to_string()).unwrap());
    let stop = expect_int(pyself_borrow.get_attribute("stop".to_string()).unwrap());
    let step = expect_int(pyself_borrow.get_attribute("step".to_string()).unwrap());
    
    if step == 1 {
        return PyPointer::new(PyObject::Str(format!("range({}, {})", start, stop)));
    }
    PyPointer::new(PyObject::Str(format!("range({}, {}, {})", start, stop, step)))
}

pub fn range__iter__(pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    init_internal_class(PyPointer::new(py_range_iterator), vec![pyself.clone()])
}




pub const py_range: PyClass = PyClass::Internal {
    name_func: || "range".to_string(),
    super_classes_func: || vec![PyPointer::new(py_object)],
    __new__: None,
    __init__: Some(InitFunc(&(range__init__ as InitFuncType))),

    __str__: None,
    __repr__: Some(UnaryFunc(&(range__repr__ as UnaryFuncType))),

    __add__: None,
    __pow__: None,

    __iter__: Some(UnaryFunc(&(range__iter__ as UnaryFuncType))),
    __next__: None,
};

pub fn range_iterator__init__(pyself: PyPointer<PyObject>, args: Vec<PyPointer<PyObject>>) {
    let range_obj = args.get(0).unwrap_or_else(|| panic!("Expected one argument to __init__, received zero"));
    assert_eq!(args.len(), 1);
    
    let range_obj = range_obj.borrow(); // TODO assert range type
    
    let start = expect_int(range_obj.get_attribute("start".to_string()).unwrap());  // Copy by value
    let stop = expect_int(range_obj.get_attribute("stop".to_string()).unwrap());
    let step = expect_int(range_obj.get_attribute("step".to_string()).unwrap());

    let mut pyself_mut = pyself.borrow_mut();
    
    pyself_mut.set_attribute("start".to_string(), PyPointer::new(PyObject::Int(start)));  // insert values
    pyself_mut.set_attribute("stop".to_string(), PyPointer::new(PyObject::Int(stop)));
    pyself_mut.set_attribute("step".to_string(), PyPointer::new(PyObject::Int(step)));
    pyself_mut.set_attribute("current".to_string(), PyPointer::new(PyObject::Int(start)));
}

pub fn range_iterator__next__(pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let mut pyself_mut = pyself.borrow_mut();
    let mut current = expect_int(pyself_mut.get_attribute("current".to_string()).unwrap());
    let stop = expect_int(pyself_mut.get_attribute("stop".to_string()).unwrap());
    let step = expect_int(pyself_mut.get_attribute("step".to_string()).unwrap());
    
    current += step;
    
    if current >= stop {
        return PyPointer::new(PyObject::InternalFlag(PyPointer::new(PyFlag::StopIteration)));  // StopIteration TODO remove internal pypointer
    }

    pyself_mut.set_attribute("current".to_string(), PyPointer::new(PyObject::Int(current)));
    
    PyPointer::new(PyObject::Int(current))
}

pub const py_range_iterator: PyClass = PyClass::Internal {  // Hidden class
    name_func: || "range_iterator".to_string(),
    super_classes_func: || vec![PyPointer::new(py_object)],
    __new__: None,
    __init__: Some(InitFunc(&(range_iterator__init__ as InitFuncType))),

    __str__: None,
    __repr__: None,

    __add__: None,
    __pow__: None,

    __iter__: None,
    __next__: Some(UnaryFunc(&(range_iterator__next__ as UnaryFuncType))),
};
