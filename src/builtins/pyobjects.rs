// use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, PoisonError, RwLock};
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

    pub fn set_field(&mut self, key: String, value: PyObject) -> Result<bool, PoisonError<&mut HashMap<String, Arc<PyObject>>>>  {
        let mut attributes = self.attributes.get_mut()?;
        let insert_result = attributes.insert(key, Arc::new(value));
        Ok(insert_result.is_some())
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
    // Additional types as needed
}

impl PyObject {
    pub fn get_class(&self) -> Option<Arc<PyClass>> {
        match self {
            // PyObject::Class(py_class) => Some(py_class.clone()),// needs to be type
            PyObject::Instance(py_instance) => Some(py_instance.class.clone()),
            _ => None,
        }
    }

    pub fn get_attribute(&self, name: String) -> Option<Arc<PyObject>> {
        match self {
            PyObject::Int(_) => {todo!()}
            PyObject::Float(_) => {todo!()}
            PyObject::Str(_) => {todo!()}
            PyObject::Bool(_) => {todo!()}
            PyObject::Class(_) => {todo!()}
            PyObject::Instance(instance) => {
                let attr = instance.get_field(&name);
                println!("*{:?}", attr);
                if attr.is_some() {
                    return attr;
                }

                instance.class.clone().search_for_attribute(name)
            }
            PyObject::Function(_) => {todo!()}
            PyObject::Exception(_) => {todo!()}
            PyObject::None => {todo!()}
            PyObject::InternalSlot(_) => {todo!()}
        }
    }
    
    pub fn need_string(&self) -> String {
        match self {
            PyObject::Str(s) => s.clone(),
            _ => panic!("Object is not a string"), // TODO make python error
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

