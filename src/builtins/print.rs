use crate::builtins::str::py_str;
use crate::builtins::structure::pyobject::{FuncReturnType, PyObject};
use crate::pyarena::PyArena;

pub fn py_print(arena: &mut PyArena, args: &[PyObject]) -> FuncReturnType {
    let sep = " ";
    let mut strs = Vec::new();
    
    for arg in args {
        strs.push(py_str(arg, arena)?.expect_immutable().expect_string());
    }
    
    let result = strs.join(sep);
    
    println!("{}", result);
    
    Ok(PyObject::none())
}