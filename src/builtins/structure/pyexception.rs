use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct PyException {
    name: &'static str,
    message: Option<String>,
    traceback: Option<Vec<String>>,
    super_exceptions: Vec<Rc<PyException>>
}

impl PyException {
    fn new(name: &'static str, super_exception: Vec<Rc<PyException>>) -> Rc<PyException> {
        Rc::new(PyException {
            name,
            message: None,
            traceback: None,
            super_exceptions: super_exception
        })
    }
    
    pub fn instantiate(&self, message: String) -> PyException {
        let mut new_exception = self.clone();
        new_exception.message = Some(message);
        new_exception
    }
    
    pub fn empty(&self) -> PyException {
        self.clone()
    }

    pub fn add_trace(&mut self, trace: Box<dyn Display>) {
        if self.traceback.is_none() {
            self.traceback = Some(vec![]);
        }
        self.traceback.as_mut().unwrap().push(trace.to_string());
    }
    
    pub fn is_same_type(&self, exception: &PyException) -> bool {
        self.name == exception.name
    }
}

impl Display for PyException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Traceback (most recent call last):")?;
        if let Some(traceback) = &self.traceback {
            for trace in traceback {
                write!(f, "\n\t{}", trace)?;
            }
        }

        if let Some(message) = &self.message {
            write!(f, "{}: {}", self.name, message)?;
        } else {
            write!(f, "{}", self.name)?;
        }

        Ok(())
    }
}

#[warn(dead_code)]
#[derive(Debug)]
pub struct Exceptions {
    // hierarchy based on https://docs.python.org/3/library/exceptions.html#exception-hierarchy
    pub base_exception: Rc<PyException>,

    pub generator_exit: Rc<PyException>,
    pub system_exit: Rc<PyException>,
    pub keyboard_interrupt: Rc<PyException>,
    pub exception: Rc<PyException>,

    pub arithmatic_error: Rc<PyException>,
        pub overflow_error: Rc<PyException>,
        pub zero_division_error: Rc<PyException>,
    pub assertion_error: Rc<PyException>,
    pub attribute_error: Rc<PyException>,
    pub buffer_error: Rc<PyException>,
    pub eof_error: Rc<PyException>,
    pub import_error: Rc<PyException>,
    pub lookup_error: Rc<PyException>,
        pub index_error: Rc<PyException>,
        pub key_error: Rc<PyException>,
    pub memory_error: Rc<PyException>,
    pub name_error: Rc<PyException>,
    pub os_error: Rc<PyException>,
    pub reference_error: Rc<PyException>,
    pub runtime_error: Rc<PyException>,
        pub not_implemented_error: Rc<PyException>,
        pub recursion_error: Rc<PyException>,
    pub stop_async_iteration: Rc<PyException>,
    pub stop_iteration: Rc<PyException>,
    pub syntax_error: Rc<PyException>,
    pub system_error: Rc<PyException>,
    pub type_error: Rc<PyException>,
    pub value_error: Rc<PyException>,
}

impl Exceptions {
    pub fn new() -> Self {
        let base_exception = PyException::new("BaseException", vec![]);
        
        let generator_exit = PyException::new("GeneratorExit", vec![base_exception.clone()]);
        let system_exit = PyException::new("SystemExit", vec![base_exception.clone()]);
        let keyboard_interrupt = PyException::new("KeyboardInterrupt", vec![base_exception.clone()]);
        let exception = PyException::new("Exception", vec![base_exception.clone()]);
        
        let arithmatic_error = PyException::new("ArithmaticError", vec![exception.clone()]);
            let overflow_error = PyException::new("OverflowError", vec![arithmatic_error.clone()]);
            let zero_division_error = PyException::new("ZeroDivisionError", vec![arithmatic_error.clone()]);
        
        let assertion_error = PyException::new("AssertionError", vec![exception.clone()]);
        let attribute_error = PyException::new("AttributeError", vec![exception.clone()]);
        let buffer_error = PyException::new("BufferError", vec![exception.clone()]);
        let eof_error = PyException::new("EOFError", vec![exception.clone()]);
        let import_error = PyException::new("ImportError", vec![exception.clone()]);
        let lookup_error = PyException::new("LookupError", vec![exception.clone()]);
            let index_error = PyException::new("IndexError", vec![lookup_error.clone()]);
            let key_error = PyException::new("KeyError", vec![lookup_error.clone()]);
        let memory_error = PyException::new("MemoryError", vec![exception.clone()]);
        let name_error = PyException::new("NameError", vec![exception.clone()]);
        let os_error = PyException::new("OSError", vec![exception.clone()]);
        let reference_error = PyException::new("ReferenceError", vec![exception.clone()]);
        let runtime_error = PyException::new("RuntimeError", vec![exception.clone()]);
            let not_implemented_error = PyException::new("NotImplementedError", vec![runtime_error.clone()]);
            let recursion_error = PyException::new("RecursionError", vec![runtime_error.clone()]);
        let stop_async_iteration = PyException::new("StopAsyncIteration", vec![exception.clone()]);
        let stop_iteration = PyException::new("StopIteration", vec![exception.clone()]);
        let syntax_error = PyException::new("SyntaxError", vec![exception.clone()]);
        let system_error = PyException::new("SystemError", vec![exception.clone()]);
        let type_error = PyException::new("TypeError", vec![exception.clone()]);
        let value_error = PyException::new("ValueError", vec![exception.clone()]);
        
        Exceptions {
            base_exception,
            generator_exit,
            system_exit,
            keyboard_interrupt,
            exception,
            arithmatic_error,
            overflow_error,
            zero_division_error,
            assertion_error,
            attribute_error,
            buffer_error,
            eof_error,
            import_error,
            lookup_error,
            index_error,
            key_error,
            memory_error,
            name_error,
            os_error,
            reference_error,
            runtime_error,
            not_implemented_error,
            recursion_error,
            stop_async_iteration,
            stop_iteration,
            syntax_error,
            system_error,
            type_error,
            value_error,
        }
    }
}

