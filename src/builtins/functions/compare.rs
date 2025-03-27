use crate::builtins::function_utils::call_function_1_arg_min;
use crate::builtins::structure::magic_methods::PyMagicMethod;
use crate::builtins::structure::pyobject::{FuncReturnType, PyObject};
use crate::parser::Comparitor;
use crate::pyarena::PyArena;

pub fn compare_op(left: &PyObject, right: &PyObject, comp: &Comparitor, arena: &mut PyArena) -> FuncReturnType {
    match comp {
        Comparitor::Equal => left_hand_compare_op(&PyMagicMethod::Eq, left, right, arena),
        Comparitor::NotEqual => left_hand_compare_op(&PyMagicMethod::Ne, left, right, arena),
        Comparitor::LessThan => left_hand_compare_op(&PyMagicMethod::Lt, left, right, arena),
        Comparitor::LessThanOrEqual => left_hand_compare_op(&PyMagicMethod::Le, left, right, arena),
        Comparitor::GreaterThan => left_hand_compare_op(&PyMagicMethod::Gt, left, right, arena),
        Comparitor::GreaterThanOrEqual => left_hand_compare_op(&PyMagicMethod::Ge, left, right, arena),

        Comparitor::Is => Ok(is_compare(false, left, right, arena)),
        Comparitor::IsNot => Ok(is_compare(true, left, right, arena)),
        Comparitor::In => {todo!()}
        Comparitor::NotIn => {todo!()}
    }
}

fn is_compare(not: bool, left: &PyObject, right: &PyObject, arena: &mut PyArena) -> PyObject {
    let left_loc = left.get_memory_location();
    let right_loc = right.get_memory_location();
    
    arena.statics.get_bool((left_loc == right_loc) != not).clone()
}

fn left_hand_compare_op(op: &PyMagicMethod, left: &PyObject, right: &PyObject, arena: &mut PyArena) -> FuncReturnType {
    let left_compare_func = left.get_magic_method(op, arena);
    
    if let Some(left_compare_func) = left_compare_func {
        let left_compare = call_function_1_arg_min(&left_compare_func, left, &[right.clone()], arena);
        
        return left_compare.or_else(|err| {
            if err.is_same_type(&arena.exceptions.not_implemented_error) {
                return right_hand_compare_op(op, left, right, arena);
            }
            Err(err)
        })
    }
    
    right_hand_compare_op(op, left, right, arena)
}

fn right_hand_compare_op(op: &PyMagicMethod, left: &PyObject, right: &PyObject, arena: &mut PyArena) -> FuncReturnType {
    let right_op = flip_to_right_hand_op(op);
    let right_compare_func = right.get_magic_method(right_op, arena);
    
    if let Some(right_compare_func) = right_compare_func {
        let right_compare = call_function_1_arg_min(&right_compare_func, right, &[left.clone()], arena);
        
        return match right_compare {
            Ok(result) => Ok(result),
            Err(err) => {
                if err.is_same_type(&arena.exceptions.not_implemented_error) {
                    let message = format!("'{}' not supported between instances of '{}' and '{}'", op, left.clone_class(arena).get_name(), right.clone_class(arena).get_name());
                    return Err(arena.exceptions.type_error.instantiate(message));
                }
                Err(err)
            },
        };
    }
    
    let message = format!("'{}' not supported between instances of '{}' and '{}'", op, left.clone_class(arena).get_name(), right.clone_class(arena).get_name());
    Err(arena.exceptions.type_error.instantiate(message))
}

fn flip_to_right_hand_op(op: &PyMagicMethod) -> &PyMagicMethod {
    match op {
        PyMagicMethod::Eq => {&PyMagicMethod::Eq}
        PyMagicMethod::Ne => {&PyMagicMethod::Ne}
        PyMagicMethod::Lt => {&PyMagicMethod::Gt}
        PyMagicMethod::Le => {&PyMagicMethod::Ge}
        PyMagicMethod::Gt => {&PyMagicMethod::Lt}
        PyMagicMethod::Ge => {&PyMagicMethod::Le}
        e => {panic!("{} is not a handed compare operation", e)}
    }
}