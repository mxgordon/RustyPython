use crate::builtins::function_utils::call_function_1_arg_min;
use crate::builtins::structure::magic_methods::PyMagicMethod;
use crate::builtins::structure::pyobject::{FuncReturnType, PyObject};
use crate::pyarena::PyArena;

pub fn math_op(left: PyObject, right: PyObject, py_magic_method: PyMagicMethod, arena: &mut PyArena) -> FuncReturnType {
    let ref left_math_func = left.get_magic_method(&py_magic_method, arena);

    if let Some(left_math_func) = left_math_func {
        let math_result = call_function_1_arg_min(left_math_func, &left, &[right.clone()], arena);

        return match math_result {
            Ok(result) => Ok(result),
            Err(err) => {
                if err.is_same_type(&arena.exceptions.not_implemented_error) {
                    return right_hand_math_op(left, right, py_magic_method, arena)
                }
                Err(err)
            },
        };
    }

    right_hand_math_op(left, right, py_magic_method, arena)
}

fn right_hand_math_op(left: PyObject, right: PyObject, mut py_magic_method: PyMagicMethod,  arena: &mut PyArena) -> FuncReturnType {
    py_magic_method.make_right_handed();
    
    let ref right_math_func = right.get_magic_method(&py_magic_method, arena);

    if let Some(right_math_fun) = right_math_func {
        return call_function_1_arg_min(right_math_fun, &right, &[left], arena);
    }
    
    println!("{:?} {}", right_math_func, py_magic_method);

    let error_msg = format!("unsupported operand type(s) for {}: '{}' and '{}'", py_magic_method, left.clone_class(arena).get_name(), right.clone_class(arena).get_name());
    Err(arena.exceptions.not_implemented_error.instantiate(error_msg))
}