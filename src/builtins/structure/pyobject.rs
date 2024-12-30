use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::rc::Rc;
use crate::parser::CodeBlock;
use crate::pyarena::PyArena;
use crate::builtins::structure::magic_methods::{PyMagicMethod};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyinstance::PyInstance;

#[derive(Debug)]
pub struct PyFunction {
    name: String,
    args: Vec<String>,
    body: Vec<CodeBlock>,
}


#[derive(Debug)]
pub enum PyObject {
    None,
    Int(i64),
    Float(f64),
    Str(String),
    // List(Vec<PyObject>),
    // Dict(HashMap<String, PyObject>),
    Bool(bool),
    Class(Rc<PyClass>),
    Instance(PyInstance),
    Function(PyFunction),
    // Exception(PyExceptionType),
    InternalSlot(Rc<PyInternalFunction>),
    IteratorFlag(PyIteratorFlag)
}

impl PyObject {
    pub fn get_class<'a>(&'a self, arena: &'a mut PyArena) -> &'a Rc<PyClass> {
        match self {
            PyObject::Instance(py_instance) => py_instance.get_class(),
            PyObject::Int(_) => &arena.globals.int_class,
            PyObject::Float(_) => {&arena.globals.float_class}
            PyObject::Str(_) => {todo!()}
            PyObject::Bool(_) => {todo!()}
            PyObject::Class(_) => {todo!()}
            PyObject::Function(_) => {todo!()}
            PyObject::None => {todo!()}
            PyObject::InternalSlot(_) => {todo!()}
            PyObject::IteratorFlag(_) => todo!()
        }
    }

    pub fn set_attribute(&mut self, name: &String, value: PyPointer<PyObject>)  {
        todo!("needs to use the public set field as not to create new fields")
        // match self {
        //     PyObject::Instance(instance) => {
        //         instance.set_field(name.to_owned(), value).expect("TODO: panic message");
        //     }
        //     _ => panic!("Cannot set {} of an object that is a {:?}", &name, &self), // TODO make python error
        // }
    }

    pub fn get_magic_method(&self, py_magic_method: PyMagicMethod, arena: &mut PyArena) -> Option<PyPointer<PyObject>> {
        match self {
            PyObject::Int(_) => {arena.globals.int_class.search_for_magic_method(py_magic_method)}
            PyObject::Float(_) => {arena.globals.float_class.search_for_magic_method(py_magic_method)}
            PyObject::Str(_) => {todo!()}
            PyObject::Bool(_) => {todo!()}
            PyObject::Class(_) => {todo!()}
            PyObject::Instance(instance) => { instance.get_class().search_for_magic_method(py_magic_method) }
            PyObject::Function(_) => {todo!()}
            // PyObject::Exception(_) => {todo!()}
            PyObject::None => {todo!()}
            PyObject::InternalSlot(_) => {todo!()},
            PyObject::IteratorFlag(_) => todo!()
        }
    }

    pub fn get_attribute(&self, name: &str, arena: &mut PyArena) -> PyPointer<PyObject> {
        // todo!("needs improvement, must also search attributes hashmap")
        match self {
            PyObject::Int(_value) => {arena.globals.int_class.search_for_method(name).unwrap()}  // TODO make better
            PyObject::Float(_) => {arena.globals.float_class.search_for_method(name).unwrap()}
            PyObject::Str(_) => {todo!()}
            PyObject::Bool(_) => {todo!()}
            PyObject::Class(_) => {todo!()}
            PyObject::Instance(instance) => {instance.get_field(name, arena).unwrap()}
            PyObject::Function(_) => {todo!()}
            PyObject::None => {todo!()}
            PyObject::InternalSlot(_) => {todo!()},
            PyObject::IteratorFlag(_) => {todo!()}
        }
    }
    
    pub fn expect_string(&self) -> String {
        match self {
            PyObject::Str(s) => s.clone(),
            _ => panic!("Object is not a string"), // TODO make python error
        }
    }

    pub fn expect_internal_slot(&self) -> Rc<PyInternalFunction> {
        match self {
            PyObject::InternalSlot(slot) => slot.clone(),
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
    pub fn expect_instance(&self) -> &PyInstance {
        match self {
            PyObject::Instance(instance) => instance,
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
    
    pub fn expect_instance_mut(&mut self) -> &mut PyInstance {
        match self {
            PyObject::Instance(instance) => instance,
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
}

pub type FuncReturnType = Result<PyPointer<PyObject>, PyException>;
pub type InitReturnType = Result<(), PyException>;

pub type NewFuncType = fn(&mut PyArena, Rc<PyClass>, Vec<PyPointer<PyObject>>) -> FuncReturnType;
pub type InitFuncType = fn(&mut PyArena, PyPointer<PyObject>, Vec<PyPointer<PyObject>>) -> InitReturnType;
pub type UnaryFuncType = fn(&mut PyArena, PyPointer<PyObject>) -> FuncReturnType;
pub type BivariateFuncType = fn(&mut PyArena, PyPointer<PyObject>, PyPointer<PyObject>) -> FuncReturnType;
pub type VariadicFuncType = fn(&mut PyArena, PyPointer<PyObject>, Vec<PyPointer<PyObject>>) -> FuncReturnType;
pub type ManyArgFuncType = fn(&mut PyArena, Vec<PyPointer<PyObject>>) -> FuncReturnType;

#[derive(Debug, Clone)]
pub enum PyInternalFunction {
    NewFunc(&'static NewFuncType),
    InitFunc(&'static InitFuncType),

    UnaryFunc(&'static UnaryFuncType),
    BivariateFunc(&'static BivariateFuncType),
    VariadicFunc(&'static VariadicFuncType),

    ManyArgFunc(&'static ManyArgFuncType),
}

#[derive(Debug)]
pub struct PyPointer<T> {
    inner: Rc<RefCell<T>>
}

impl<T> Clone for PyPointer<T> {
    fn clone(&self) -> Self {
        PyPointer {
            inner: self.inner.clone()
        }
    }
}

impl<T> PyPointer<T> {
    pub fn new(value: T) -> Self {
        PyPointer {
            inner: Rc::new(RefCell::new(value)),
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}

#[derive(Debug)]
pub enum PyIteratorFlag {
    Break,
    Continue
}