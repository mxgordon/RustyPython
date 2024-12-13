use std::cell::{Ref, RefCell, RefMut};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use crate::builtins::pyint::py_int;
use crate::parser::CodeBlock;


// type Rc<T> = Arc<T>;
macro_rules! match_magic_funcs {
    ($name_str:expr, $($name:ident),*) => {
            match $name_str {
                $(
                    stringify!($name) => $name,
                )*
                _ => &None,
            }
    };
}

#[derive(Debug)]
pub struct PyMagicMethods {
    // --- internal functions ---
    // Instantiating functions
    pub __new__: Option<PyInternalFunction>,
    pub __init__: Option<PyInternalFunction>,

    // String functions
    pub __str__: Option<PyInternalFunction>,
    pub __repr__: Option<PyInternalFunction>,

    // Math functions
    pub __add__: Option<PyInternalFunction>,
    pub __pow__: Option<PyInternalFunction>,

    // Iterating functions
    pub __iter__: Option<PyInternalFunction>,
    pub __next__: Option<PyInternalFunction>,
}

pub const fn py_magic_methods_defaults() -> PyMagicMethods {
    PyMagicMethods {
        __new__: None,
        __init__: None,
        __str__: None,
        __repr__: None,
        __add__: None,
        __pow__: None,
        __iter__: None,
        __next__: None,
    }
}

#[derive(Debug)]
pub enum PyClass {
    UserDefined {
        name: String,
        attributes: HashMap<String, PyPointer<PyObject>>,
        super_classes: Vec<PyPointer<PyClass>>
    },
    Internal {
        name_func: fn() -> String,  // immutable string crate
        super_classes_func: fn() -> Vec<PyPointer<PyClass>>,
        methods: PyMagicMethods
    },
}

impl PyClass {
    pub fn get_name(&self) -> String {
        match self {
            PyClass::UserDefined { name, .. } => name.clone(),
            PyClass::Internal { name_func, .. } => name_func(),
        }
    }

    pub fn get_super_classes(&self) -> Vec<PyPointer<PyClass>> {
        match self {
            PyClass::UserDefined { super_classes, .. } => super_classes.clone(),
            PyClass::Internal { super_classes_func, .. } => super_classes_func(),
        }
    }

    pub fn defines_attribute(&self, name_str: String) -> bool {
        match self {
            PyClass::UserDefined { attributes, .. } => attributes.contains_key(&name_str),
            PyClass::Internal {
                methods: PyMagicMethods { __new__, __init__, __str__, __repr__, __add__, __pow__, __iter__, __next__ }, name_func, super_classes_func,
            } => match_magic_funcs!(name_str.as_str(), __new__, __init__, __str__, __repr__, __add__, __pow__, __iter__, __next__).is_some()
        }
    }

    pub fn search_for_attribute(&self, name_str: String) -> Option<PyPointer<PyObject>> {
        let search_result: Option<PyPointer<PyObject>> = match self {
            PyClass::UserDefined { attributes, super_classes, .. } => {
                let attr = attributes.get(&name_str);
                if attr.is_some() {
                    return attr.cloned();
                }

                None
            },
            
            PyClass::Internal { methods: PyMagicMethods {__new__, __init__, __str__, __repr__, __add__, __pow__, __iter__, __next__}, name_func, super_classes_func } => {
                let search = match_magic_funcs!(name_str.as_str(), __new__, __init__, __str__, __repr__, __add__, __pow__, __iter__, __next__).clone();

                if let Some(func) = search {
                    return Some(PyPointer::new(PyObject::InternalSlot(PyPointer::new(func))));
                }

                None
            }

        };

        if search_result.is_none() {
            for base_class in self.get_super_classes() {
                let attr = base_class.borrow().search_for_attribute(name_str.clone());

                if attr.is_some() {
                    return attr.clone();
                }
            }
        }

        search_result
    }
}

#[derive(Debug)]
pub struct PyException {
    message: String,
    notes: Vec<String>,
    base_exceptions: Vec<Arc<PyException>>, // Support for inheritance
}

impl PyException {
    pub fn new(message: &str) -> Self {
        PyException {
            message: message.to_string(),
            notes: vec![],
            base_exceptions: vec![],
        }
    }
}


#[derive(Debug)]
pub struct PyFunction {
    name: String,
    args: Vec<String>,
    body: Vec<CodeBlock>,
}

#[derive(Debug)]
pub struct PyInstance {
    class: PyPointer<PyClass>,
    attributes: RwLock<HashMap<String, PyPointer<PyObject>>>,
}

impl PyInstance {
    pub fn new(py_class: PyPointer<PyClass>) -> Self {
        PyInstance {
            class: py_class,
            attributes: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_field(&mut self, key: String, value: PyPointer<PyObject>) {
        let mut attributes = self.attributes.get_mut().unwrap_or_else(|e| panic!("Failed to get mutable attributes: {:?}", e));
        let _old_value = attributes.insert(key, value);
    }

    pub fn get_field(&self, key: &str) -> Option<PyPointer<PyObject>> {
        let attributes = self.attributes.read().ok()?;
        attributes.get(key).cloned()
    }
}

#[derive(Debug, Clone)]
pub enum PyObject {
    Int(i64),
    Float(f64),
    Str(String),
    // List(Vec<PyObject>),
    // Dict(HashMap<String, PyObject>),
    Bool(bool),
    Class(PyPointer<PyClass>),
    Instance(PyPointer<PyInstance>),
    Function(PyPointer<PyFunction>),
    Exception(PyPointer<PyException>),
    None,
    // InternalSlot(PyPointer<fn(Vec<PyObject>) -> PyObject>),
    InternalSlot(PyPointer<PyInternalFunction>),
    InternalFlag(PyPointer<PyFlag>),
    // Additional types as needed
}

impl PyObject {
    pub fn get_class(&self) -> PyPointer<PyClass> {
        match self {
            PyObject::Instance(py_instance) => py_instance.borrow().class.clone(),
            PyObject::Int(_) => PyPointer::new(py_int),  // TODO try to avoid making new pointer
            PyObject::Float(_) => {todo!()}
            PyObject::Str(_) => {todo!()}
            PyObject::Bool(_) => {todo!()}
            PyObject::Class(_) => {todo!()}
            PyObject::Function(_) => {todo!()}
            PyObject::Exception(_) => {todo!()}
            PyObject::None => {todo!()}
            PyObject::InternalSlot(_) => {todo!()}
            PyObject::InternalFlag(_) => {todo!()}
        }
    }

    pub fn set_attribute(&mut self, name: String, value: PyPointer<PyObject>)  {
        match self {
            PyObject::Instance(instance) => {
                instance.borrow_mut().set_field(name.clone(), value.clone());
            }
            _ => panic!("Cannot set {} of an object that is a {:?}", name, self), // TODO make python error
        }
    }

    pub fn get_attribute(&self, name: String) -> Option<PyPointer<PyObject>> {
        match self {
            PyObject::Int(_value) => {py_int.search_for_attribute(name)}
            PyObject::Float(_) => {todo!()}
            PyObject::Str(_) => {todo!()}
            PyObject::Bool(_) => {todo!()}
            PyObject::Class(_) => {todo!()}
            PyObject::Instance(instance) => {
                let attr = instance.borrow().get_field(&name);
                
                if attr.is_some() {
                    return attr;
                }

                instance.borrow().class.clone().borrow().search_for_attribute(name)
            }
            PyObject::Function(_) => {todo!()}
            PyObject::Exception(_) => {todo!()}
            PyObject::None => {todo!()}
            PyObject::InternalSlot(_) => {todo!()},
            PyObject::InternalFlag(_) => todo!()
        }
    }
    
    pub fn need_string(&self) -> String {
        match self {
            PyObject::Str(s) => s.clone(),
            _ => panic!("Object is not a string"), // TODO make python error
        }
    }
    
    pub fn is_flag_type(&self, flag: PyFlag) -> bool {
        match self {
            PyObject::InternalFlag(flag_type) => {
                let self_descrim = *flag_type.borrow().clone() as isize;
                let other_descrim = flag as isize;

                self_descrim == other_descrim
            },
            _ => false,
        }
    }

    pub fn is_not_flag(&self) -> bool {
        match self {
            PyObject::InternalFlag(_) => false,
            _ => true,
        }
    }

    pub fn expect_internal_slot(&self) -> PyPointer<PyInternalFunction> {
        match self {
            PyObject::InternalSlot(slot) => slot.clone(),
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
}

pub type NewFuncType = fn(PyPointer<PyClass>, Vec<PyPointer<PyObject>>) -> PyPointer<PyObject>;
pub type InitFuncType = fn(PyPointer<PyObject>, Vec<PyPointer<PyObject>>) -> ();
pub type UnaryFuncType = fn(PyPointer<PyObject>) -> PyPointer<PyObject>;
pub type BivariateFuncType = fn(PyPointer<PyObject>, PyPointer<PyObject>) -> PyPointer<PyObject>;
pub type VariadicFuncType = fn(PyPointer<PyObject>, Vec<PyPointer<PyObject>>) -> PyPointer<PyObject>;
pub type ManyArgFuncType = fn(Vec<PyPointer<PyObject>>) -> PyPointer<PyObject>;

#[derive(Debug, Clone)]
pub enum PyInternalFunction {
    NewFunc(&'static NewFuncType),
    InitFunc(&'static InitFuncType),

    UnaryFunc(&'static UnaryFuncType),
    BivariateFunc(&'static BivariateFuncType),
    VariadicFunc(&'static VariadicFuncType),

    ManyArgFunc(&'static ManyArgFuncType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PyFlag {
    StopIteration,
    GeneratorExit,
    Break,
    Continue,
}


#[derive(Debug)]
pub struct PyPointer<T> {
    inner: Rc<RefCell<Box<T>>>
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
            inner: Rc::new(RefCell::new(Box::new(value))),
        }
    }

    pub fn borrow(&self) -> Ref<Box<T>> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Box<T>> {
        self.inner.borrow_mut()
    }
}