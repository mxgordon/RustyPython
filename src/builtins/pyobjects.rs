use std::cell::{Ref, RefCell};
use std::cmp::PartialEq;
// use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, PoisonError, RwLock};
use crate::builtins::pyint::py_int;
// use std::rc::Arc;
use crate::parser::CodeBlock;

#[derive(Debug)]
pub struct PyClass {
    pub name: String,
    attributes: HashMap<String, Arc<PyObject>>,
    base_classes: Vec<Arc<PyClass>>, // Support for inheritance
}

impl PyClass {
    pub fn new(name: &str, attributes: HashMap<String, Arc<PyObject>>, base_classes: Vec<Arc<PyClass>>) -> Self {
        PyClass {
            name: name.to_string(),
            attributes,
            base_classes,
        }
    }

    pub fn search_for_attribute(&self, name: String) -> Option<Arc<PyObject>> {
        let attr = self.attributes.get(&name);
        if attr.is_some() {
            return attr.cloned();
        }

        for base_class in &self.base_classes {
            let bc = base_class.clone();

            let attr = bc.search_for_attribute(name.clone());
            
            if attr.is_some() {
                return attr;
            }
        }
        None
    }
    
    // pub fn new_internal_attrs(name: &str, attributes: Vec<&str>, base_classes: Vec<Arc<PyClass>>) -> Self {
    //     let attr_map = attributes.into_iter().map(|attr| (attr.to_string(), PyObject::InternalDef)).collect();
    //
    //     PyClass {
    //         name: name.to_string(),
    //         attributes: attr_map,
    //         base_classes,
    //     }
    // }
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
    class: Arc<PyClass>,
    attributes: RwLock<HashMap<String, Arc<PyObject>>>,
}

impl PyInstance {
    pub fn new(py_class: Arc<PyClass>) -> Self {
        PyInstance {
            class: py_class,
            attributes: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_field(&mut self, key: String, value: Arc<PyObject>) -> bool  {
        let mut attributes = self.attributes.get_mut().unwrap_or_else(|e| panic!("Failed to get mutable attributes: {:?}", e));
        let insert_result = attributes.insert(key, value);
        insert_result.is_some()
    }

    // pub fn get_field(&self, key: &str) -> Result<Arc<PyObject>, PoisonError<std::sync::RwLockReadGuard<HashMap<String, Arc<PyObject>>>>> {
    pub fn get_field(&self, key: &str) -> Option<Arc<PyObject>> {
        let attributes = self.attributes.read().ok()?;
        attributes.get(key).cloned()
        // Ok(attributes.get(key).unwrap_or(&Arc::new(PyObject::None)).clone())
    }
}

#[derive(Debug)]
pub enum PyObject {
    Int(i64),
    Float(f64),
    Str(String),
    // List(Vec<PyObject>),
    // Dict(HashMap<String, PyObject>),
    Bool(bool),
    Class(Arc<PyClass>),
    Instance(Arc<PyInstance>),
    Function(Arc<PyFunction>),
    Exception(Arc<PyException>),
    None,
    // InternalSlot(Arc<fn(Vec<PyObject>) -> PyObject>),
    InternalSlot(Arc<PyInternalFunction>),
    InternalFlag(Arc<PyFlag>),
    // Additional types as needed
}

impl PyObject {
    pub fn get_class(&self) -> Option<Arc<PyClass>> {
        match self {
            PyObject::Instance(py_instance) => Some(py_instance.class.clone()),
            PyObject::Int(_) => Some(py_int.clone()),
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

    pub fn set_attribute(&mut self, name: String, value: Arc<PyObject>)  {
        match self {
            PyObject::Instance(instance) => {
                // let attr = instance.get_field(&name);
                // 
                // if attr.is_some() {
                //     return attr;
                // }
                let succeeded = instance.set_field(name.clone(), value.clone());
                // TODO repace Arc with something that is reference counted cloned and can be borrowed mutably maybe either Rc<RefCell<T>> or Arc<Mutex<T>> or Rc<Box<T>>
                if !succeeded {
                    panic!("Failed to set attribute {} of instance", name);
                }

                // instance.class.clone().search_for_attribute(name)
            }
            _ => panic!("Cannot set {} of an object that is a {:?}", name, self), // TODO make python error
        }
    }

    pub fn get_attribute(&self, name: String) -> Option<Arc<PyObject>> {
        match self {
            PyObject::Int(_value) => {py_int.search_for_attribute(name)}
            PyObject::Float(_) => {todo!()}
            PyObject::Str(_) => {todo!()}
            PyObject::Bool(_) => {todo!()}
            PyObject::Class(_) => {todo!()}
            PyObject::Instance(instance) => {
                let attr = instance.get_field(&name);
                
                if attr.is_some() {
                    return attr;
                }

                instance.class.clone().search_for_attribute(name)
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
                let self_descrim = (**flag_type).clone() as isize;
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
}



#[derive(Debug)]
pub enum PyInternalFunction {
    NoArgs(fn() -> Arc<PyObject>),
    OneArg(fn(Arc<PyObject>) -> Arc<PyObject>),
    TwoArgs(fn(Arc<PyObject>, Arc<PyObject>) -> Arc<PyObject>),
    ThreeArgs(fn(Arc<PyObject>, Arc<PyObject>, Arc<PyObject>) -> Arc<PyObject>),
    ManyArgs(fn(Vec<Arc<PyObject>>) -> Arc<PyObject>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PyFlag {
    StopIteration,
    GeneratorExit,
    Break,
    Continue,
}


#[derive(Debug)]
pub struct PyObjectPointer {
    inner: Rc<RefCell<Box<PyObject>>>
}

impl PyObjectPointer {
    pub fn new(inner: PyObject) -> Self {
        PyObjectPointer {
            inner: Rc::new(RefCell::new(Box::new(inner)))
        }
    }

    // pub fn get_class(&self) -> Option<PyObjectPointer> {
    //     // self.inner.clone()
    //     let inner_class = self.inner.borrow().get_class()?;
    //     
    // }
}

impl Clone for PyObjectPointer {
    fn clone(&self) -> Self {
        PyObjectPointer {
            inner: self.inner.clone()
        }
    }
}

impl Deref for PyObjectPointer {
    type Target = RefCell<Box<PyObject>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// DerefMut is optional and depends on whether you need mutable access via methods
// impl DerefMut for PyObjectPointer {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }

// impl Deref for PyObjectPointer {
//     type Target = PyObject;
// 
//     // fn deref(&self) -> &Self::Target {
//     //     let borrowed = self.inner.clone().borrow();
//     //     let x = &*borrowed;
//     //     x
//     //     
//     // }
// }