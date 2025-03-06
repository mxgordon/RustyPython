use crate::builtins::function_utils::call_function_1_arg_min;
use crate::builtins::structure::magic_methods::PyMagicMethod;
use crate::builtins::structure::pyobject::{FuncReturnType, PyObject};
use crate::pyarena::PyArena;

pub fn py_str_tmp(obj: &PyObject, arena: &mut PyArena) -> FuncReturnType {
    let str_fn = obj.expect_immutable().get_magic_method(&PyMagicMethod::Str, arena);
    
    if str_fn.is_none() {
        panic!("Object has no __str__ method");
    }
    
    let str_fn = str_fn.unwrap();
    
    let str_rtn = call_function_1_arg_min(&str_fn, obj, &[], arena);
    
    str_rtn  // TODO assert str_rtn is a string
}