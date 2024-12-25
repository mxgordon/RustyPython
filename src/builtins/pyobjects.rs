use std::cell::{Ref, RefCell, RefMut};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::parser::CodeBlock;
use crate::pyarena::PyArena;
use mopa::{mopafy};

#[derive(Clone, Debug, EnumIter)]
pub enum PyMagicMethod {
    New,
    Init,
    Str,
    Repr,
    Add,
    Sub,
    Mul,
    TrueDiv,
    Pow,
    Iter,
    Next,
}

impl PyMagicMethod {
    pub fn get_method(&self, methods: &PyMagicMethods) -> Option<Rc<PyInternalFunction>> {
        match self {
            PyMagicMethod::New => methods.__new__.clone(),
            PyMagicMethod::Init => methods.__init__.clone(),
            PyMagicMethod::Str => methods.__str__.clone(),
            PyMagicMethod::Repr => methods.__repr__.clone(),
            PyMagicMethod::Add => methods.__add__.clone(),
            PyMagicMethod::Sub => methods.__sub__.clone(),
            PyMagicMethod::Mul => methods.__mul__.clone(),
            PyMagicMethod::TrueDiv => methods.__truediv__.clone(),
            PyMagicMethod::Pow => methods.__pow__.clone(),
            PyMagicMethod::Iter => methods.__iter__.clone(),
            PyMagicMethod::Next => methods.__next__.clone(),
        }
    }
    
    pub fn get_method_mut<'a>(&'a self, methods: &'a mut PyMagicMethods) -> &'a mut Option<Rc<PyInternalFunction>> {
        match self {
            PyMagicMethod::New => &mut methods.__new__,
            PyMagicMethod::Init => &mut methods.__init__,
            PyMagicMethod::Str => &mut methods.__str__,
            PyMagicMethod::Repr => &mut methods.__repr__,
            PyMagicMethod::Add => &mut methods.__add__,
            PyMagicMethod::Sub => &mut methods.__sub__,
            PyMagicMethod::Mul => &mut methods.__mul__,
            PyMagicMethod::TrueDiv => &mut methods.__truediv__,
            PyMagicMethod::Pow => &mut methods.__pow__,
            PyMagicMethod::Iter => &mut methods.__iter__,
            PyMagicMethod::Next => &mut methods.__next__,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PyMagicMethod::New => "__new__",
            PyMagicMethod::Init => "__init__",
            PyMagicMethod::Str => "__str__",
            PyMagicMethod::Repr => "__repr__",
            PyMagicMethod::Add => "__add__",
            PyMagicMethod::Sub => "__sub__",
            PyMagicMethod::Mul => "__mul__",
            PyMagicMethod::TrueDiv => "__truediv__",
            PyMagicMethod::Pow => "__pow__",
            PyMagicMethod::Iter => "__iter__",
            PyMagicMethod::Next => "__next__",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    #[must_use="Make sure you know this is up to date"]
    pub fn from_string(name: &str) -> Option<PyMagicMethod> {
        match name {
            "__new__" => Some(PyMagicMethod::New),
            "__init__" => Some(PyMagicMethod::Init),
            "__str__" => Some(PyMagicMethod::Str),
            "__repr__" => Some(PyMagicMethod::Repr),
            "__add__" => Some(PyMagicMethod::Add),
            "__sub__" => Some(PyMagicMethod::Sub),
            "__mul__" => Some(PyMagicMethod::Mul),
            "__truediv__" => Some(PyMagicMethod::TrueDiv),
            "__pow__" => Some(PyMagicMethod::Pow),
            "__iter__" => Some(PyMagicMethod::Iter),
            "__next__" => Some(PyMagicMethod::Next),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct PyMagicMethods {
    // --- internal functions ---
    // Instantiating functions
    pub __new__: Option<Rc<PyInternalFunction>>,
    pub __init__: Option<Rc<PyInternalFunction>>,

    // String functions
    pub __str__: Option<Rc<PyInternalFunction>>,
    pub __repr__: Option<Rc<PyInternalFunction>>,

    // Math functions
    pub __add__: Option<Rc<PyInternalFunction>>,
    pub __sub__: Option<Rc<PyInternalFunction>>,
    pub __mul__: Option<Rc<PyInternalFunction>>,
    pub __truediv__: Option<Rc<PyInternalFunction>>,
    pub __pow__: Option<Rc<PyInternalFunction>>,

    // Iterating functions
    pub __iter__: Option<Rc<PyInternalFunction>>,
    pub __next__: Option<Rc<PyInternalFunction>>,
}

pub const fn py_magic_methods_defaults() -> PyMagicMethods {
    PyMagicMethods {
        __new__: None,
        __init__: None,
        __str__: None,
        __repr__: None,
        __add__: None,
        __sub__: None,
        __mul__: None,
        __truediv__: None,
        __pow__: None,
        __iter__: None,
        __next__: None,
    }
}
impl PyMagicMethods {
    pub fn get_method(&self, magic_method: PyMagicMethod) -> Option<Rc<PyInternalFunction>> {
        magic_method.get_method(self)
        // let internal_func = magic_method.get_method(&self);
        // 
        // if let Some(internal_func) = internal_func {// TODO move pypointer creating to initialization of PyMagicMethods
        //     return Some(PyPointer::new(internal_func.clone()));
        // }
        // None
    }
    
    pub fn set_method(&mut self, magic_method: PyMagicMethod, new_method: Rc<PyInternalFunction>) {
        let internal_func= magic_method.get_method_mut(self);
        
        *internal_func = Some(new_method);
    }
}
#[derive(Debug)]
pub enum PyClass {
    UserDefined {
        name: String,
        attributes: HashMap<String, PyPointer<PyObject>>,
        super_classes: Vec<Rc<PyClass>>
    },
    Internal {
        name: String,
        super_classes: Vec<Rc<PyClass>>,
        methods: PyMagicMethods
    },
}

impl PyClass {  // TODO !automatic caching function that sets the name, superclass, and inheritance functions
    pub fn get_name(&self) -> String {
        match self {
            PyClass::UserDefined { name, .. } => name.clone(),
            PyClass::Internal { name, .. } => name.clone(),
        }
    }

    pub fn get_super_classes(&self) -> Vec<Rc<PyClass>> {
        match self {
            PyClass::UserDefined { super_classes, .. } => super_classes.clone(),
            PyClass::Internal { super_classes, .. } => super_classes.clone(),
        }
    }

    pub fn defines_attribute(&self, magic_method: PyMagicMethod) -> bool {
        match self {
            PyClass::UserDefined { attributes, .. } => attributes.contains_key(&magic_method.to_string()),
            PyClass::Internal {
                methods, ..
            } => magic_method.get_method(methods).is_some(),
        }
    }
    
    pub fn create(mut self) -> PyClass {
        let attrs = self.get_super_magic_methods();
        self.load_super_magic_methods(attrs);
        self
    }
    
    pub fn get_super_magic_methods(&self) -> Vec<(PyMagicMethod, Rc<PyInternalFunction>)> {
        let mut methods_to_set = Vec::new();
        if let PyClass::Internal { .. } = self {
            for magic_method_type in PyMagicMethod::iter() {
                if self.defines_attribute(magic_method_type.clone()) {
                    continue;
                }

                if let Some(super_method) = self.search_for_attribute_internal(magic_method_type.clone()) {
                    methods_to_set.push((magic_method_type, super_method));
                }
                
            }
        }
        
        methods_to_set
    }

    pub fn load_super_magic_methods(&mut self, methods_to_set: Vec<(PyMagicMethod, Rc<PyInternalFunction>)>) {
        match self {
            PyClass::Internal { methods, .. } => {
                for (magic_method_type, super_method) in methods_to_set {
                    methods.set_method(magic_method_type, super_method);
                }
            }
            _ => todo!()
        }
    }

    pub fn search_for_attribute_internal(&self, magic_method: PyMagicMethod) -> Option<Rc<PyInternalFunction>> {
        let search_result = match self {
            PyClass::UserDefined { .. } => {
               panic!("UserDefined classes will not have internal methods")
            },

            PyClass::Internal { methods, .. } => {
                methods.get_method(magic_method.clone())
            }

        };

        if search_result.is_none() {
            for base_class in self.get_super_classes() {
                let attr = base_class.search_for_attribute_internal(magic_method.clone());

                if attr.is_some() {
                    return attr.clone();
                }
            }
        }

        search_result
    }
    
    pub fn get_magic_method_internal(&self, magic_method: PyMagicMethod) -> Option<Rc<PyInternalFunction>> {
        match self {
            PyClass::UserDefined { .. } => {
                panic!("UserDefined classes will not have internal methods")
            },

            PyClass::Internal {methods, ..} => {
                let attr = methods.get_method(magic_method.clone());
                attr
            }
        }
    }

    pub fn search_for_attribute(&self, magic_method: PyMagicMethod) -> Option<PyPointer<PyObject>> {
        match self {
            PyClass::UserDefined { attributes, .. } => {
                attributes.get(&magic_method.to_string()).cloned()
            },

            PyClass::Internal { methods, .. } => {
                Some(PyPointer::new(PyObject::InternalSlot(methods.get_method(magic_method.clone())?)))
            }
        }
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
pub struct PyInstanceGeneric {
    class: Rc<PyClass>,
    attributes: RwLock<HashMap<String, PyPointer<PyObject>>>,
    // pub internal_storage: Vec<PyObject>

}

pub trait PyInstance: mopa::Any + Debug {
    // fn new(py_class: Rc<PyClass>) -> Self;
    fn set_field(&mut self, key: String, value: PyPointer<PyObject>);
    fn get_field(&self, key: &str) -> Option<PyPointer<PyObject>>;
    fn get_class(&self) -> Rc<PyClass>;
}

mopafy!(PyInstance);

impl PyInstance for PyInstanceGeneric {
    fn set_field(&mut self, key: String, value: PyPointer<PyObject>) {
        let attributes = self.attributes.get_mut().unwrap_or_else(|e| panic!("Failed to get mutable attributes: {:?}", e));
        let _old_value = attributes.insert(key, value);
    }

    fn get_field(&self, key: &str) -> Option<PyPointer<PyObject>> {
        let attributes = self.attributes.read().ok()?;
        attributes.get(key).cloned()
    }

    fn get_class(&self) -> Rc<PyClass> {
        self.class.clone()
    }
}

impl PyInstanceGeneric {
    fn new(py_class: Rc<PyClass>) -> Self {
        PyInstanceGeneric {
            class: py_class,
            attributes: RwLock::new(HashMap::new()),
        }
    }
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
    Instance(Box<dyn PyInstance>),
    Function(PyFunction),
    Exception(PyException),
    InternalSlot(Rc<PyInternalFunction>),
    InternalFlag(PyFlag),
    // Additional types as needed
}

impl PyObject {
    pub fn get_class(&self, arena: &mut PyArena) -> Rc<PyClass> {
        match self {
            PyObject::Instance(py_instance) => py_instance.get_class().clone(),
            PyObject::Int(_) => arena.globals.int_class.clone(),
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

    pub fn set_attribute(&mut self, name: &String, value: PyPointer<PyObject>)  {
        match self {
            PyObject::Instance(instance) => {
                instance.set_field(name.clone(), value.clone());
            }
            _ => panic!("Cannot set {} of an object that is a {:?}", name, self), // TODO make python error
        }
    }

    pub fn get_magic_method(&self, py_magic_method: PyMagicMethod, arena: &mut PyArena) -> Option<PyPointer<PyObject>> {
        match self {
            PyObject::Int(_value) => {arena.globals.int_class.search_for_attribute(py_magic_method)}  // TODO make better
            PyObject::Float(_) => {todo!()}
            PyObject::Str(_) => {todo!()}
            PyObject::Bool(_) => {todo!()}
            PyObject::Class(_) => {todo!()}
            PyObject::Instance(instance) => {
                instance.get_class().clone().search_for_attribute(py_magic_method)
            }
            PyObject::Function(_) => {todo!()}
            PyObject::Exception(_) => {todo!()}
            PyObject::None => {todo!()}
            PyObject::InternalSlot(_) => {todo!()},
            PyObject::InternalFlag(_) => todo!()
        }
    }

    pub fn get_attribute(&self, name: &str, arena: &mut PyArena) -> Option<PyPointer<PyObject>> {
        match self {
            PyObject::Int(_value) => {arena.globals.int_class.search_for_attribute(PyMagicMethod::from_string(name)?)}  // TODO make better
            PyObject::Float(_) => {todo!()}
            PyObject::Str(_) => {todo!()}
            PyObject::Bool(_) => {todo!()}
            PyObject::Class(_) => {todo!()}
            PyObject::Instance(instance) => {
                instance.get_field(name)
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
                let self_descrim = flag_type.clone() as isize;
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

    pub fn expect_internal_slot(&self) -> Rc<PyInternalFunction> {
        match self {
            PyObject::InternalSlot(slot) => slot.clone(),
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
    pub fn expect_instance(&self) -> &Box<dyn PyInstance> {
        match self {
            PyObject::Instance(instance) => instance,
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
    
    pub fn expect_instance_mut(&mut self) -> &mut Box<dyn PyInstance> {
        match self {
            PyObject::Instance(instance) => instance,
            _ => panic!("Expected internal slot"), // TODO make python error
        }
    }
}

pub type NewFuncType = fn(&mut PyArena, Rc<PyClass>, Vec<PyPointer<PyObject>>) -> PyPointer<PyObject>;
pub type InitFuncType = fn(&mut PyArena, PyPointer<PyObject>, Vec<PyPointer<PyObject>>) -> ();
pub type UnaryFuncType = fn(&mut PyArena, PyPointer<PyObject>) -> PyPointer<PyObject>;
pub type BivariateFuncType = fn(&mut PyArena, PyPointer<PyObject>, PyPointer<PyObject>) -> PyPointer<PyObject>;
pub type VariadicFuncType = fn(&mut PyArena, PyPointer<PyObject>, Vec<PyPointer<PyObject>>) -> PyPointer<PyObject>;
pub type ManyArgFuncType = fn(&mut PyArena, Vec<PyPointer<PyObject>>) -> PyPointer<PyObject>;

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