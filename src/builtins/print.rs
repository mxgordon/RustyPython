use crate::builtins::pyobjects::{PyObject, PyPointer};
use crate::builtins::str::py_str;

pub fn py_print(args: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {
    let sep = " ";
    
    let strs: Vec<String> = args.iter().map(|arg| py_str(arg.clone()).borrow().need_string()).collect();
    
    let result = strs.join(sep);
    
    println!("{}", result);
    
    PyPointer::new(PyObject::None)
}