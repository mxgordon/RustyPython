use std::sync::Arc;
use crate::builtins::pyobjects::PyObject;
use crate::builtins::str::py_str;

pub fn py_print(args: Vec<Arc<PyObject>>) -> Arc<PyObject> {
    let sep = " ";
    
    let strs: Vec<String> = args.iter().map(|arg| py_str(arg.clone()).need_string()).collect();
    
    let result = strs.join(sep);
    
    println!("{}", result);
    
    Arc::new(PyObject::None)
}