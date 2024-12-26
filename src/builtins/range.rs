use std::collections::HashMap;
use std::fmt::{Debug};
use std::rc::Rc;
use crate::builtins::function_utils::init_internal_class;
use crate::builtins::pyint::{expect_int_ptr};
use crate::builtins::pyobjects::{py_magic_methods_defaults, NewFuncType, PyClass, PyException, PyFlag, PyInstance, PyInstanceInternal, PyMagicMethods, PyObject, PyPointer, UnaryFuncType};
use crate::builtins::pyobjects::PyInternalFunction::{NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

#[derive(Debug)]
pub struct RangeInstance {
    start: i64,
    stop: i64,
    step: i64
}

impl PyInstanceInternal for RangeInstance {
    fn set_field(&mut self, _key: String, _value: PyPointer<PyObject>) -> Result<bool, PyException> {
        Err(PyException::new("All attributes are readonly"))
    }

    fn get_field(&self, key: &str) -> Result<PyPointer<PyObject>, PyException> {
        match key {
            "start" => Ok(PyPointer::new(PyObject::Int(self.start))),
            "stop" => Ok(PyPointer::new(PyObject::Int(self.stop))),
            "step" => Ok(PyPointer::new(PyObject::Int(self.step))),
            _ => Err(PyException::new("Cannot find field"))
        }
    }
}

pub fn range__new__(_arena: &mut PyArena, pyclass: Rc<PyClass>, args: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {
    let first = expect_int_ptr(args.get(0).unwrap_or_else(|| panic!("Expected at least one argument to __init__, received zero")).clone());
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
    
    PyPointer::new(PyObject::Instance(PyInstance::new_empty_attrs(pyclass, Box::new(
        RangeInstance {
            start,
            stop,
            step, 
        }
    ))))
}

pub fn range__repr__(_arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let pyself = pyself.borrow();
    let instance = pyself.expect_instance();

    let range_internal = instance.internal.downcast_ref::<RangeInstance>();

    if let Some(range_internal) = range_internal {
        if range_internal.step == 1 {
            return PyPointer::new(PyObject::Str(format!("range({}, {})", range_internal.start, range_internal.stop)));
        }
        PyPointer::new(PyObject::Str(format!("range({}, {}, {})", range_internal.start, range_internal.stop, range_internal.step)))
    } else {
        panic!("Instance received is not of RangeInstance type, instead {:?}", instance)
    }
}

pub fn range__iter__(arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    init_internal_class(arena.globals.range_iterator_class.clone(), vec![pyself.clone()], arena)
}

pub fn get_range_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "range".to_string(),
        super_classes: vec![object_class],
        attributes: HashMap::new(),
        magic_methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(range__new__ as NewFuncType)))),
            // __init__: Some(Rc::new(InitFunc(&(range__init__ as InitFuncType)))),

            __repr__: Some(Rc::new(UnaryFunc(&(range__repr__ as UnaryFuncType)))),

            __iter__: Some(Rc::new(UnaryFunc(&(range__iter__ as UnaryFuncType)))),

            ..py_magic_methods_defaults()
        }
    }.create()
}

#[derive(Debug)]
struct RangeIteratorInstance {
    current: i64,
    stop: i64,
    step: i64
}

impl PyInstanceInternal for RangeIteratorInstance {
    fn set_field(&mut self, _key: String, _value: PyPointer<PyObject>) -> Result<bool, PyException> {
        panic!("range_iterator type has no public fields")
    }

    fn get_field(&self, _key: &str) -> Result<PyPointer<PyObject>, PyException> {
        panic!("range_iterator type has no public fields")
    }
}

pub fn range_iterator__new__(_arena: &mut PyArena, pyclass: Rc<PyClass>, args: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {
    let range_obj = args.first().unwrap_or_else(|| panic!("Expected one argument to __new__, received zero"));
    assert_eq!(args.len(), 1);

    let range_obj = range_obj.borrow(); 
    let range_instance = range_obj.expect_instance();

    let range_internal = range_instance.internal.downcast_ref::<RangeInstance>().expect("arg instance should be of RangeInstance type");
    PyPointer::new(PyObject::Instance(PyInstance::new_empty_attrs(
        pyclass,
        Box::new(RangeIteratorInstance {
            current: range_internal.start,
            stop: range_internal.stop,
            step: range_internal.step,
        })
    )))
}

pub fn range_iterator__next__(_arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let mut pyself = pyself.borrow_mut();
    let instance = pyself.expect_instance_mut();
    
    if let Some(range_iterator_internal) = instance.internal.downcast_mut::<RangeIteratorInstance>() {
        let current = &mut range_iterator_internal.current;
        let stop = range_iterator_internal.stop;
        let step = range_iterator_internal.step;
        
        if *current >= stop { // TODO make it depend on the sign of the step var
            return PyPointer::new(PyObject::InternalFlag(PyFlag::StopIteration));
        }
        
        let rtn_val = PyPointer::new(PyObject::Int(*current));
        
        *current += step;
        
        rtn_val
    } else {
        panic!("instance is not RangeIteratorType, its {:?}", instance)
    }
}

pub fn get_range_iterator_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {  // Hidden class
        name: "range_iterator".to_string(),
        super_classes: vec![object_class],
        attributes: HashMap::new(),
        magic_methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(range_iterator__new__ as NewFuncType)))),
            
            __next__: Some(Rc::new(UnaryFunc(&(range_iterator__next__ as UnaryFuncType)))),
            ..py_magic_methods_defaults()
        }
    }.create()
}
