use crate::builtins::pyobjects::{PyObject, PyPointer};
use crate::builtins::str::py_str;
use crate::pyarena::PyArena;

pub fn py_print(arena: &mut PyArena, args: Vec<PyPointer<PyObject>>) -> PyPointer<PyObject> {
    let sep = " ";
    
    let strs: Vec<String> = args.iter().map(|arg| py_str(arg.clone(), arena).borrow().need_string()).collect();
    
    let result = strs.join(sep);
    
    println!("{}", result);
    
    PyPointer::new(PyObject::None)
}