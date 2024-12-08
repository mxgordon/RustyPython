use std::sync::Arc;
use crate::builtins::pyobjects::PyObject;
use crate::evaluator::{eval_internal_func, eval_obj_init};
use crate::pyarena::PyArena;

pub fn call_function(func: Arc<PyObject>, args: Vec<Arc<PyObject>>, arena: &mut PyArena) -> Arc<PyObject> {
    match &*func {
        PyObject::Function(_func) => {
            todo!()
        }
        PyObject::InternalSlot(func) => {
            eval_internal_func(func.clone(), args, arena)
        }
        PyObject::Class(pyclass) => {
            eval_obj_init(pyclass.clone(), args, arena)
        }
        _ => {
            panic!("{:?} is not a function", func); // TODO Make python error
        }
    }
}