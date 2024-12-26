use crate::builtins::pyobjects::{PyInternalFunction, PyMagicMethod, PyObject, PyPointer};
use crate::pyarena::PyArena;

pub fn py_str(obj: PyPointer<PyObject>, arena: &mut PyArena) -> PyPointer<PyObject> {
    let obj = obj.clone();
    let str_fn = obj.borrow().get_magic_method(PyMagicMethod::Str, arena);
    
    if str_fn.is_none() {
        panic!("Object has no __str__ method"); // TODO Make python error
    }
    
    let str_fn = str_fn.unwrap();
    let str_rtn;
    
    match *str_fn.borrow() {
        PyObject::InternalSlot(ref func) => {
            match **func { 
                PyInternalFunction::UnaryFunc(func) => {
                    str_rtn = func.call((arena, obj.clone()));
                },
                _ => {todo!()}
            }
        },
        PyObject::Function(ref _func) => {
            todo!()
        },
        _ => {panic!("Object has no __str__ method");}  // TODO Make python error
    }
    
    //assert! TODO check if str_rtn is a string
    
    str_rtn
}