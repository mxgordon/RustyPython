use std::rc::Rc;
use strum_macros::{Display, EnumIter};
use crate::builtins::structure::pyobject::PyInternalFunction;

#[derive(Clone, Copy, Debug, Display, EnumIter)]
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
    }

    pub fn set_method(&mut self, magic_method: PyMagicMethod, new_method: Rc<PyInternalFunction>) {
        let internal_func= magic_method.get_method_mut(self);

        *internal_func = Some(new_method);
    }
}
