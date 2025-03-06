use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::rc::Rc;
use crate::parser::CodeBlock;
use crate::pyarena::PyArena;
use crate::builtins::structure::magic_methods::{PyMagicMethod};
use crate::builtins::structure::pyclass::PyClass;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyinstance::PyInstance;

#[derive(Clone, Debug)]
pub enum PyObject {
    Immutable(Rc<PyImmutableObject>),  // TODO, consider using Cow's here
    Mutable(PyPointer<PyMutableObject>),
    Internal(PyInternalObject),
    IteratorFlag(PyIteratorFlag)
}

impl PyObject {
    pub fn new_string(value: String) -> Self {
        Self::new_immutable(PyImmutableObject::Str(value))
    }
    
    pub fn new_int(value: i64) -> Self {
        Self::new_immutable(PyImmutableObject::Int(value))
    }
    pub fn new_float(value: f64) -> Self {
        Self::new_immutable(PyImmutableObject::Float(value))
    }
    pub fn new_internal_class(value: Rc<PyClass>) -> Self {
        Self::new_internal(PyInternalObject::InternalClass(value))
    }
    
    pub fn new_internal_func(value: Rc<PyInternalFunction>) -> Self {
        Self::new_internal(PyInternalObject::InternalFunction(value))
    }
    
    pub fn new_mutable(value: PyMutableObject) -> Self {
        PyObject::Mutable(PyPointer::new(value))
    }
    
    pub fn new_immutable(value: PyImmutableObject) -> Self {
        PyObject::Immutable(Rc::new(value))
    }
    
    pub fn new_internal(value: PyInternalObject) -> Self {
        PyObject::Internal(value)
    }
    
    
    pub fn break_() -> Self {  // TODO prob should be moved out of the pyobject class (currently in here for legacy reasons)
        PyObject::IteratorFlag(PyIteratorFlag::Break)
    }
    
    pub fn continue_() -> Self {  // TODO prob should be moved out of the pyobject class (currently in here for legacy reasons)
        PyObject::IteratorFlag(PyIteratorFlag::Continue)
    }
    
    pub fn create_new_none() -> Self {
        PyObject::Immutable(Rc::new(PyImmutableObject::None))
    }
    
    pub fn create_new_bool(value: bool) -> Self {
        Self::new_immutable(PyImmutableObject::Bool(value))
    }
    
    pub fn stop_iteration() -> Self {  // TODO prob should be moved out of the pyobject class (currently in here for legacy reasons)
        PyObject::IteratorFlag(PyIteratorFlag::StopIteration)
    }
    
    pub fn expect_immutable(&self) -> &Rc<PyImmutableObject> {
        match self {
            PyObject::Immutable(inner) => inner,
            _ => panic!("Expected immutable object"), // TODO make python error
        }
    }
    
    pub fn expect_internal(&self) -> &PyInternalObject {
        match self {
            PyObject::Internal(inner) => inner,
            _ => panic!("Expected immutable object"), // TODO make python error
        }
    }
    
    pub fn expect_mutable(&self) -> &PyPointer<PyMutableObject> {
        match self {
            PyObject::Mutable(inner) => inner,
            _ => panic!("Expected mutable object"), // TODO make python error
        }
    }
    
    pub fn get_magic_method(&self, py_magic_method: &PyMagicMethod, arena: &mut PyArena) -> Option<PyObject> {
        match self {
            PyObject::Immutable(inner) => inner.get_magic_method(py_magic_method, arena),
            PyObject::Mutable(inner) => inner.borrow().get_magic_method(py_magic_method, arena),
            PyObject::IteratorFlag(_) => {panic!("IteratorFlag has no magic methods")}
            PyObject::Internal(_) => {todo!()}
        }
    }
    
    pub fn clone_class(&self, arena: &mut PyArena) -> Rc<PyClass> {
        match *self {
            PyObject::Immutable(ref inner) => inner.get_class(arena).clone(),
            PyObject::Mutable(ref inner) => inner.borrow().get_class().clone(),
            PyObject::IteratorFlag(_) => {panic!("IteratorFlag has no class")}
            PyObject::Internal(_) => {todo!()}
        }
    }
}


#[derive(Debug)]
pub enum PyImmutableObject {
    None,
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),  // TODO, maybe use immutable string type here
}

impl PyImmutableObject {
    pub fn get_class<'a>(&self, arena: &'a mut PyArena) -> &'a Rc<PyClass> {
        match self {
            PyImmutableObject::None => {&arena.globals.none_class}
            PyImmutableObject::Int(_) => {&arena.globals.int_class}
            PyImmutableObject::Float(_) => {&arena.globals.float_class}
            PyImmutableObject::Bool(_) => {&arena.globals.bool_class}
            PyImmutableObject::Str(_) => {todo!()}
        }
    }
    
    pub fn expect_string(&self) -> String {
        match self {
            PyImmutableObject::Str(s) => s.clone(),
            _ => panic!("Object is not a string"), // TODO make python error
        }
    }
    
    pub fn get_magic_method(&self, py_magic_method: &PyMagicMethod, arena: &mut PyArena) -> Option<PyObject> {
        self.get_class(arena).search_for_magic_method(py_magic_method)
    }
}

#[derive(Debug, Clone)]
pub enum PyInternalObject {
    InternalFunction(Rc<PyInternalFunction>),
    InternalClass(Rc<PyClass>)
}

impl PyInternalObject {
    pub fn expect_internal_slot(&self) -> Rc<PyInternalFunction> {
        match self {
            PyInternalObject::InternalFunction(slot) => slot.clone(),
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
    
    pub fn expect_internal_function(&self) -> Rc<PyInternalFunction> {
        match self {
            PyInternalObject::InternalFunction(slot) => slot.clone(),
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
}



#[derive(Debug)]
pub enum PyMutableObject {
    // Class(Rc<PyClass>),
    Instance(PyInstance),
    Function(PyFunction),
}

impl PyMutableObject {
    pub fn get_class(&self) -> &Rc<PyClass> {
        match self {
            PyMutableObject::Instance(py_instance) => py_instance.get_class(),
            // PyMutableObject::Class(py_class) => py_class,
            PyMutableObject::Function(_py_function) => todo!(),
        }
    }

    pub fn get_field(&self, name: &str, arena: &mut PyArena) -> FuncReturnType {
        match self {
            PyMutableObject::Instance(instance) => instance.get_field(name, arena),
            // PyMutableObject::Class(py_class) => todo!(),
            PyMutableObject::Function(_py_function) => todo!(),
        }
    }
    
    pub fn expect_instance(&self) -> &PyInstance {
        match self {
            PyMutableObject::Instance(instance) => instance,
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
    
    pub fn expect_instance_mut(&mut self) -> &mut PyInstance {
        match self {
            PyMutableObject::Instance(instance) => instance,
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }


    pub fn get_magic_method(&self, py_magic_method: &PyMagicMethod, _arena: &mut PyArena) -> Option<PyObject> {
        match self {
            // PyMutableObject::Class(_) => {todo!()}
            PyMutableObject::Instance(instance) => { instance.get_class().search_for_magic_method(py_magic_method) }
            PyMutableObject::Function(_) => {todo!()}
        }
    }
}

#[derive(Debug)]
pub struct PyFunction {
    name: String,
    args: Vec<String>,
    body: Vec<CodeBlock>,
}

pub type FuncReturnType = Result<PyObject, PyException>;
pub type EmptyFuncReturnType = Result<(), PyException>;

pub type NewFuncType = fn(&mut PyArena, Rc<PyClass>, &[PyObject]) -> FuncReturnType;
pub type InitFuncType = fn(&mut PyArena, &PyObject, &[PyObject]) -> EmptyFuncReturnType;
pub type UnaryFuncType = fn(&mut PyArena, &PyObject) -> FuncReturnType;
pub type BivariateFuncType = fn(&mut PyArena, &PyObject, &PyObject) -> FuncReturnType;
pub type VariadicFuncType = fn(&mut PyArena, &PyObject, &[PyObject]) -> FuncReturnType;
pub type ManyArgFuncType = fn(&mut PyArena, &[PyObject]) -> FuncReturnType;

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

#[derive(Debug, Clone)]
pub enum PyIteratorFlag {
    Break,
    Continue,
    StopIteration
}