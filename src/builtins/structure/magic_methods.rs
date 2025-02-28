use std::rc::Rc;
use strum_macros::{Display, EnumIter};
use crate::builtins::structure::pyobject::PyInternalFunction;

#[derive(Clone, Copy, Debug, Display, EnumIter)]
pub enum PyMagicMethod {
    New,
    Init,
    
    Str,
    Repr,
    
    Add {right: bool},
    Sub {right: bool},
    Mul {right: bool},
    TrueDiv {right: bool},
    Pow {right: bool},
    
    Int,
    Bytes,
    Bool,
    Float,
    
    Iter,
    Next,
    
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Contains,
}

impl PyMagicMethod {
    pub fn get_method(&self, methods: &PyMagicMethods) -> Option<Rc<PyInternalFunction>> {
        match self {
            PyMagicMethod::New => methods.__new__.clone(),
            PyMagicMethod::Init => methods.__init__.clone(),
            PyMagicMethod::Str => methods.__str__.clone(),
            PyMagicMethod::Repr => methods.__repr__.clone(),
            PyMagicMethod::Add {right} => if *right {methods.__radd__.clone()} else { methods.__add__.clone() },
            PyMagicMethod::Sub {right} => if *right {methods.__rsub__.clone()} else { methods.__sub__.clone() },
            PyMagicMethod::Mul {right} => if *right {methods.__rmul__.clone()} else { methods.__mul__.clone() },
            PyMagicMethod::TrueDiv {right} => if *right {methods.__rtruediv__.clone()} else { methods.__truediv__.clone() },
            PyMagicMethod::Pow {right} => if *right {methods.__rpow__.clone()} else { methods.__pow__.clone() },
            PyMagicMethod::Int => methods.__int__.clone(),
            PyMagicMethod::Bool => methods.__bool__.clone(),
            PyMagicMethod::Bytes => methods.__bytes__.clone(),
            PyMagicMethod::Float => methods.__float__.clone(),
            PyMagicMethod::Iter => methods.__iter__.clone(),
            PyMagicMethod::Next => methods.__next__.clone(),
            PyMagicMethod::Eq => methods.__eq__.clone(),
            PyMagicMethod::Ne => methods.__ne__.clone(),
            PyMagicMethod::Lt => methods.__lt__.clone(),
            PyMagicMethod::Le => methods.__le__.clone(),
            PyMagicMethod::Gt => methods.__gt__.clone(),
            PyMagicMethod::Ge => methods.__ge__.clone(),
            PyMagicMethod::Contains => methods.__contains__.clone(),
        }
    }

    pub fn get_method_mut<'a>(&'a self, methods: &'a mut PyMagicMethods) -> &'a mut Option<Rc<PyInternalFunction>> {
        match self {
            PyMagicMethod::New => &mut methods.__new__,
            PyMagicMethod::Init => &mut methods.__init__,
            PyMagicMethod::Str => &mut methods.__str__,
            PyMagicMethod::Repr => &mut methods.__repr__,
            PyMagicMethod::Add {right} => if *right {&mut methods.__radd__} else { &mut methods.__add__ },
            PyMagicMethod::Sub {right} => if *right {&mut methods.__rsub__} else { &mut methods.__sub__ },
            PyMagicMethod::Mul {right} => if *right {&mut methods.__rmul__} else { &mut methods.__mul__ },
            PyMagicMethod::TrueDiv {right} => if *right {&mut methods.__rtruediv__} else { &mut methods.__truediv__ },
            PyMagicMethod::Pow {right} => if *right {&mut methods.__rpow__} else { &mut methods.__pow__ },
            PyMagicMethod::Int => &mut methods.__int__,
            PyMagicMethod::Bool => &mut methods.__bool__,
            PyMagicMethod::Bytes => &mut methods.__bytes__,
            PyMagicMethod::Float => &mut methods.__float__,
            PyMagicMethod::Iter => &mut methods.__iter__,
            PyMagicMethod::Next => &mut methods.__next__,
            PyMagicMethod::Eq => &mut methods.__eq__,
            PyMagicMethod::Ne => &mut methods.__ne__,
            PyMagicMethod::Lt => &mut methods.__lt__,
            PyMagicMethod::Le => &mut methods.__le__,
            PyMagicMethod::Gt => &mut methods.__gt__,
            PyMagicMethod::Ge => &mut methods.__ge__,
            PyMagicMethod::Contains => &mut methods.__contains__,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PyMagicMethod::New => "__new__",
            PyMagicMethod::Init => "__init__",
            PyMagicMethod::Str => "__str__",
            PyMagicMethod::Repr => "__repr__",
            PyMagicMethod::Add{right} => if *right {"__radd__"} else {"__add__"},
            PyMagicMethod::Sub{right} => if *right {"__rsub__"} else {"__sub__"},
            PyMagicMethod::Mul{right} => if *right {"__rmul__"} else {"__mul__"},
            PyMagicMethod::TrueDiv{right} => if *right {"__rtruediv__"} else {"__truediv__"},
            PyMagicMethod::Pow{right} => if *right {"__rpow__"} else {"__pow__"},
            PyMagicMethod::Int => "__int__",
            PyMagicMethod::Bool => "__bool__",
            PyMagicMethod::Bytes => "__bytes__",
            PyMagicMethod::Float => "__float__",
            PyMagicMethod::Iter => "__iter__",
            PyMagicMethod::Next => "__next__",
            PyMagicMethod::Eq => "__eq__",
            PyMagicMethod::Ne => "__ne__",
            PyMagicMethod::Lt => "__lt__",
            PyMagicMethod::Le => "__le__",
            PyMagicMethod::Gt => "__gt__",
            PyMagicMethod::Ge => "__ge__",
            PyMagicMethod::Contains => "__contains__",
        }
    }

    #[must_use="Make sure you know this is up to date"]
    pub fn from_string(name: &str) -> Option<PyMagicMethod> {
        match name {
            "__new__" => Some(PyMagicMethod::New),
            "__init__" => Some(PyMagicMethod::Init),
            "__str__" => Some(PyMagicMethod::Str),
            "__repr__" => Some(PyMagicMethod::Repr),
            "__add__" => Some(PyMagicMethod::Add{right: false}),
            "__sub__" => Some(PyMagicMethod::Sub{right: false}),
            "__mul__" => Some(PyMagicMethod::Mul{right: false}),
            "__truediv__" => Some(PyMagicMethod::TrueDiv{right: false}),
            "__pow__" => Some(PyMagicMethod::Pow{right: false}),
            "__radd__" => Some(PyMagicMethod::Add{right: true}),
            "__rsub__" => Some(PyMagicMethod::Sub{right: true}),
            "__rmul__" => Some(PyMagicMethod::Mul{right: true}),
            "__rtruediv__" => Some(PyMagicMethod::TrueDiv{right: true}),
            "__rpow__" => Some(PyMagicMethod::Pow{right: true}),
            "__int__" => Some(PyMagicMethod::Int),
            "__bool__" => Some(PyMagicMethod::Bool),
            "__bytes__" => Some(PyMagicMethod::Bytes),
            "__float__" => Some(PyMagicMethod::Float),
            "__iter__" => Some(PyMagicMethod::Iter),
            "__next__" => Some(PyMagicMethod::Next),
            "__eq__" => Some(PyMagicMethod::Eq),
            "__ne__" => Some(PyMagicMethod::Ne),
            "__lt__" => Some(PyMagicMethod::Lt),
            "__le__" => Some(PyMagicMethod::Le),
            "__gt__" => Some(PyMagicMethod::Gt),
            "__ge__" => Some(PyMagicMethod::Ge),
            "__contains__" => Some(PyMagicMethod::Contains),
            _ => None,
        }
    }
    
    pub fn make_right_handed(&mut self) {
        match self {
            PyMagicMethod::Add{right} => *right = true,
            PyMagicMethod::Sub{right} => *right = true,
            PyMagicMethod::Mul{right} => *right = true,
            PyMagicMethod::TrueDiv{right} => *right = true,
            PyMagicMethod::Pow{right} => *right = true,
            _ => {panic!("Cannot make `{}` right handed", self.as_str())},
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
    // Right-hand math functions
    pub __radd__: Option<Rc<PyInternalFunction>>,
    pub __rsub__: Option<Rc<PyInternalFunction>>,
    pub __rmul__: Option<Rc<PyInternalFunction>>,
    pub __rtruediv__: Option<Rc<PyInternalFunction>>,
    pub __rpow__: Option<Rc<PyInternalFunction>>,

    // Type conversion functions
    pub __int__: Option<Rc<PyInternalFunction>>,
    pub __float__: Option<Rc<PyInternalFunction>>,
    pub __bytes__: Option<Rc<PyInternalFunction>>,
    pub __bool__: Option<Rc<PyInternalFunction>>,

    // Iterating functions
    pub __iter__: Option<Rc<PyInternalFunction>>,
    pub __next__: Option<Rc<PyInternalFunction>>,
    
    // Comparison functions
    pub __eq__: Option<Rc<PyInternalFunction>>,
    pub __ne__: Option<Rc<PyInternalFunction>>,
    pub __lt__: Option<Rc<PyInternalFunction>>,
    pub __le__: Option<Rc<PyInternalFunction>>,
    pub __gt__: Option<Rc<PyInternalFunction>>,
    pub __ge__: Option<Rc<PyInternalFunction>>,
    pub __contains__: Option<Rc<PyInternalFunction>>,
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
        
        __radd__: None,
        __rsub__: None,
        __rmul__: None,
        __rtruediv__: None,
        __rpow__: None,
        
        __int__: None,
        __float__: None,
        __bytes__: None,
        __bool__: None,
        
        __iter__: None,
        __next__: None,

        __eq__: None,
        __ne__: None,
        __lt__: None,
        __le__: None,
        __gt__: None,
        __ge__: None,
        __contains__: None,
    }
}

impl PyMagicMethods {
    pub fn get_method(&self, magic_method: &PyMagicMethod) -> Option<Rc<PyInternalFunction>> {
        magic_method.get_method(self)
    }

    pub fn set_method(&mut self, magic_method: &PyMagicMethod, new_method: Rc<PyInternalFunction>) {
        let internal_func= magic_method.get_method_mut(self);

        *internal_func = Some(new_method);
    }
}
