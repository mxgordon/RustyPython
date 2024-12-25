use std::fmt::{Debug};
use std::rc::Rc;
use crate::builtins::function_utils::init_internal_class;
use crate::builtins::pyint::{expect_int_ptr};
use crate::builtins::pyobjects::{py_magic_methods_defaults, NewFuncType, PyClass, PyFlag, PyInstance, PyMagicMethods, PyObject, PyPointer, UnaryFuncType};
use crate::builtins::pyobjects::PyInternalFunction::{NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

#[derive(Debug)]
pub struct RangeInstance {
    class: Rc<PyClass>,
    start: i64,
    stop: i64,
    step: i64
}

impl PyInstance for RangeInstance {
    fn set_field(&mut self, _key: String, _value: PyPointer<PyObject>) {
        panic!("All attributes are readonly")
    }

    fn get_field(&self, key: &str) -> Option<PyPointer<PyObject>> {
        match key {
            "start" => Some(PyPointer::new(PyObject::Int(self.start))),
            "stop" => Some(PyPointer::new(PyObject::Int(self.stop))),
            "step" => Some(PyPointer::new(PyObject::Int(self.step))),
            _ => None
        }
    }

    fn get_class(&self) -> Rc<PyClass> {
        self.class.clone()
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
    
    PyPointer::new(PyObject::Instance(Box::new(RangeInstance {
        class: pyclass,
        start,
        stop,
        step,
    })))
}

pub fn range__repr__(_arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let pyself = pyself.borrow();
    let instance = pyself.expect_instance();

    let range_instance = instance.downcast_ref::<RangeInstance>();

    if let Some(range_instance) = range_instance {
        if range_instance.step == 1 {
            return PyPointer::new(PyObject::Str(format!("range({}, {})", range_instance.start, range_instance.stop)));
        }
        PyPointer::new(PyObject::Str(format!("range({}, {}, {})", range_instance.start, range_instance.stop, range_instance.step)))
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
        methods: PyMagicMethods {
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
    class: Rc<PyClass>,
    current: i64,
    stop: i64,
    step: i64
}

impl PyInstance for RangeIteratorInstance {
    fn set_field(&mut self, _key: String, _value: PyPointer<PyObject>) {
        panic!("range_iterator type has no public fields")
    }

    fn get_field(&self, _key: &str) -> Option<PyPointer<PyObject>> {
        panic!("range_iterator type has no public fields")
    }

    fn get_class(&self) -> Rc<PyClass> {
        self.class.clone()
    }
}

pub fn range_iterator__new__(_arena: &mut PyArena, pyclass: Rc<PyClass>, args: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {
    let range_obj = args.first().unwrap_or_else(|| panic!("Expected one argument to __new__, received zero"));
    assert_eq!(args.len(), 1);

    let range_obj = range_obj.borrow(); 
    let range_instance = range_obj.expect_instance();

    let range_instance = range_instance.downcast_ref::<RangeInstance>().expect("arg instance should be of RangeInstance type");
    PyPointer::new(PyObject::Instance(Box::new(RangeIteratorInstance {
        class: pyclass,
        current: range_instance.start,
        stop: range_instance.stop,
        step: range_instance.step,
    })))
}

pub fn range_iterator__next__(_arena: &mut PyArena, pyself: PyPointer<PyObject>) -> PyPointer<PyObject> {
    let mut pyself = pyself.borrow_mut();
    let instance = pyself.expect_instance_mut();
    
    if let Some(range_iterator_instance) = instance.downcast_mut::<RangeIteratorInstance>() {
        let current = &mut range_iterator_instance.current;
        let stop = range_iterator_instance.stop;
        let step = range_iterator_instance.step;
        
        if *current >= stop {
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
        methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(range_iterator__new__ as NewFuncType)))),
            
            __next__: Some(Rc::new(UnaryFunc(&(range_iterator__next__ as UnaryFuncType)))),
            ..py_magic_methods_defaults()
        }
    }.create()
}
