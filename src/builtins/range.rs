use std::fmt::{Debug};
use std::rc::Rc;
use ahash::AHashMap;
use crate::builtins::function_utils::init_internal_class;
use crate::builtins::pyint::expect_int;
use crate::builtins::structure::magic_methods::{py_magic_methods_defaults, PyMagicMethods};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyinstance::{PyInstance, PyInstanceInternal};
use crate::builtins::structure::pyobject::{EmptyFuncReturnType, FuncReturnType, NewFuncType, PyImmutableObject, PyMutableObject, PyObject, UnaryFuncType};
use crate::builtins::structure::pyobject::PyInternalFunction::{NewFunc, UnaryFunc};
use crate::pyarena::PyArena;

#[derive(Debug)]
pub struct RangeInstance {
    start: i64,
    stop: i64,
    step: i64
}

impl PyInstanceInternal for RangeInstance {
    fn set_field(&mut self, key: String, _value: PyObject, pyarena: &mut PyArena) -> Option<EmptyFuncReturnType> {
        let fields = ["start", "stop", "step"];

        if fields.contains(&key.as_str()) {
            return Some(Err(pyarena.exceptions.attribute_error.instantiate("readonly attribute".to_string())));
        }
        
        None
    }

    fn get_field(&self, key: &str, _pyarena: &mut PyArena) -> Option<PyObject> {
        match key {
            "start" => Some(PyObject::new_int(self.start)),
            "stop" => Some(PyObject::new_int(self.stop)),
            "step" => Some(PyObject::new_int(self.step)),
            _ => None
        }
    }
}

pub fn range__new__(arena: &mut PyArena, pyclass: Rc<PyClass>, args: &[PyObject]) -> FuncReturnType {
    let arg1 = args.get(0).ok_or_else(|| arena.exceptions.type_error.instantiate("range expected at least 1 argument, got 0".to_string()))?;
    
    let first = expect_int(arg1, arena)?;
    let second = args.get(1);
    let third = args.get(2);

    let mut start: i64 = 0;
    let stop: i64;
    let mut step = 1;

    if let Some(second) = second {
        start = first;
        stop = expect_int(second, arena)?;

        if let Some(third) = third {
            step = expect_int(third, arena)?;
        }
    } else {
        stop = first;
    }
    
    Ok(PyObject::new_mutable(PyMutableObject::Instance(PyInstance::new_empty_attrs(pyclass, Box::new(
        RangeInstance {
            start,
            stop,
            step, 
        }
    )))))
}

pub fn range__repr__(_arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let pyself = pyself.expect_mutable().borrow();
    let instance = pyself.expect_instance();

    let range_internal = instance.internal.downcast_ref::<RangeInstance>();

    if let Some(range_internal) = range_internal {
        if range_internal.step == 1 {
            return Ok(PyObject::new_immutable(PyImmutableObject::Str(format!("range({}, {})", range_internal.start, range_internal.stop))));
        }
        Ok(PyObject::new_immutable(PyImmutableObject::Str(format!("range({}, {}, {})", range_internal.start, range_internal.stop, range_internal.step))))
    } else {
        panic!("Instance received is not of RangeInstance type, instead {:?}", instance)  // should be an internal error only, very bad
    }
}

pub fn range__iter__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    init_internal_class(arena.globals.range_iterator_class.clone(), &[pyself.clone()], arena)
}

pub fn get_range_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {
        name: "range".to_string(),
        super_classes: vec![object_class],
        attributes: AHashMap::new(),
        magic_methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(range__new__ as NewFuncType)))),

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
    fn set_field(&mut self, _key: String, _value: PyObject, _arena: &mut PyArena) -> Option<EmptyFuncReturnType> {
        None
    }

    fn get_field(&self, _key: &str, _arena: &mut PyArena) -> Option<PyObject> {
        None
    }
}

pub fn range_iterator__new__(arena: &mut PyArena, pyclass: Rc<PyClass>, args: &[PyObject]) -> FuncReturnType {
    let range_obj = args.first().ok_or_else(|| arena.exceptions.type_error.instantiate("range_iterator expected at least 1 argument, got 0".to_string())).clone()?;
    assert_eq!(args.len(), 1);

    let range_obj = range_obj.expect_mutable().borrow(); 
    let range_instance = range_obj.expect_instance();

    let range_internal = range_instance.internal.downcast_ref::<RangeInstance>().expect("arg instance should be of RangeInstance type");  // Bad error
    Ok(PyObject::new_mutable(PyMutableObject::Instance(PyInstance::new_empty_attrs(
        pyclass,
        Box::new(RangeIteratorInstance {
            current: range_internal.start,
            stop: range_internal.stop,
            step: range_internal.step,
        })
    ))))
}

pub fn range_iterator__next__(arena: &mut PyArena, pyself: &PyObject) -> FuncReturnType {
    let mut pyself = pyself.expect_mutable().borrow_mut();
    let instance = pyself.expect_instance_mut();
    
    if let Some(range_iterator_internal) = instance.internal.downcast_mut::<RangeIteratorInstance>() {
        let current = &mut range_iterator_internal.current;
        let stop = range_iterator_internal.stop;
        let step = range_iterator_internal.step;
        
        if step > 0 && *current >= stop {
            return Err(arena.exceptions.stop_iteration.empty());
        } else if step < 0 && *current <= stop {
            return Err(arena.exceptions.stop_iteration.empty());
        }
        
        let rtn_val = PyObject::new_immutable(PyImmutableObject::Int(*current));
        
        *current += step;
        
        Ok(rtn_val)
    } else {
        panic!("instance is not RangeIteratorType, its {:?}", instance)
    }
}

pub fn get_range_iterator_class(object_class: Rc<PyClass>) -> PyClass {
    PyClass::Internal {  // Hidden class
        name: "range_iterator".to_string(),
        super_classes: vec![object_class],
        attributes: AHashMap::new(),
        magic_methods: PyMagicMethods {
            __new__: Some(Rc::new(NewFunc(&(range_iterator__new__ as NewFuncType)))),
            
            __next__: Some(Rc::new(UnaryFunc(&(range_iterator__next__ as UnaryFuncType)))),
            ..py_magic_methods_defaults()
        }
    }.create()
}
