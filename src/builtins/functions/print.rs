use crate::builtins::types::str::py_str_tmp;
use crate::builtins::structure::pyobject::{FuncReturnType, PyObject};
use crate::pyarena::PyArena;

pub fn py_print(arena: &mut PyArena, args: &[PyObject]) -> FuncReturnType {
    let sep = " ";
    
    let str_fold = args.iter().try_fold(Vec::new(), |mut acc, arg| {
        acc.push(py_str_tmp(arg, arena)?.expect_immutable().expect_string());
        Ok(acc)
    })?;
    
    let result = str_fold.join(sep);
    
    println!("{}", result);
    
    Ok(arena.statics.none().clone())
}