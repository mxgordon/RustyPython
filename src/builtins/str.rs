use crate::builtins::structure::magic_methods::PyMagicMethod;
use crate::builtins::structure::pyobject::{FuncReturnType, PyImmutableObject, PyInternalFunction, PyObject};
use crate::pyarena::PyArena;

pub fn py_str(obj: &PyObject, arena: &mut PyArena) -> FuncReturnType {
    let str_fn = obj.expect_immutable().get_magic_method(PyMagicMethod::Str, arena);
    
    if str_fn.is_none() {
        panic!("Object has no __str__ method");
    }
    
    let str_fn = str_fn.unwrap();
    let str_rtn;
    
    match **str_fn.expect_immutable() {  // TODO allow for user defined functions too
        PyImmutableObject::InternalSlot(ref func) => {
            match **func { 
                PyInternalFunction::UnaryFunc(func) => {
                    str_rtn = func.call((arena, obj));
                },
                _ => {todo!()}
            }
        },
        // PyObject::Function(ref _func) => {
        //     todo!()
        // },
        _ => {panic!("Object has no __str__ method");}  // TODO Make python error
    }
    
    str_rtn  // TODO assert str_rtn is a string
}