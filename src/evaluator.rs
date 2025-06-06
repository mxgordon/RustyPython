use std::cell::Ref;
use std::ops::Deref;
use std::rc::Rc;
use crate::builtins::function_utils::{call_function, eval_internal_func, eval_obj_init};
use crate::builtins::functions::compare::compare_op;
use crate::builtins::functions::math_op::math_op;
use crate::builtins::structure::magic_methods::PyMagicMethod;
use crate::builtins::structure::magic_methods::PyMagicMethod::{Add, Mul, Pow, Sub, TrueDiv};
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{EmptyFuncReturnType, FuncReturnType, PyInternalObject, PyIteratorFlag, PyObject};
use crate::builtins::types::pybool::{convert_pyobj_to_bool};
use crate::builtins::types::str::py_repr;
use crate::parser::*;
use crate::pyarena::PyArena;

pub fn evaluate(code: CodeBlock) {
    let mut arena =  PyArena::new();
    
    let code_result = eval_code_block(&code, &mut arena);
    
    if let Err(err) = code_result {
        println!("{}", err);
    }
}

fn eval_var<'a>(name: &str, arena: &'a PyArena) -> Result<&'a PyObject, PyException> {
    arena.get(name).ok_or_else(|| arena.exceptions.name_error.instantiate(format!("name '{name}' is not defined")))
}

// fn eval_var<'a>(variable: &Rc<Variable>, arena: &'a PyArena) -> Result<Ref<'a, PyObject>, PyException> {
//     // if let Some(fl_loc) = variable.fast_locals_loc {
//     //     let fast_local = arena.get_current_frame().get_fast_local(fl_loc);
//     //     
//     //     return if let Some(fast_local) = fast_local {
//     //         Ok(fast_local)
//     //     } else {
//     //         Err(arena.exceptions.unbound_local_error.instantiate(format!("cannot access local variable '{}' where it is not associated with a value", variable.name)))
//     //     }
//     // }
//     arena.search_for_var(variable).ok_or_else(|| arena.exceptions.name_error.instantiate(format!("name '{}' is not defined", variable.name)))
// }

fn eval_val(value: &Value, arena: &mut PyArena) -> PyObject {
    match value {
        Value::Integer(value) => PyObject::new_int(*value),
        Value::Float(value) => PyObject::new_float(*value),
        Value::String(value) => PyObject::new_string(value.clone()),
        Value::Boolean(value) => arena.statics.get_bool(*value).clone(),
        Value::None => arena.statics.none().clone(),
    }
}

fn eval_args(args: &[Expr], arena: &mut PyArena) -> Result<Vec<PyObject>, PyException> {
    let mut evaluated_args = Vec::with_capacity(args.len());
    
    for arg in args {
        evaluated_args.push(eval_expr(arg, arena)?);
    }
    
    Ok(evaluated_args)
}

fn eval_fun_call(func: &Box<Expr>, args: &[Expr], arena: &mut PyArena) -> FuncReturnType {
    let func = eval_expr(func, arena)?;
    
    let evaluated_args = eval_args(args, arena)?;

    match func.expect_internal() {
        // PyImmutableObject::Function(ref _func) => { // TODO allow for calling of custom functions
        //     todo!()
        // }
        PyInternalObject::InternalFunction(func) => {
            eval_internal_func(func, &evaluated_args[..], arena)
        }
        PyInternalObject::InternalClass(pyclass) => {
            eval_obj_init(pyclass.clone(), &evaluated_args[..], arena)
        }
    }
}

// fn call_method_of_pyobj_with_args(func_name: String, pyobj_expr: &Expr, args: Vec<&Expr>, arena: &mut PyArena) -> FuncReturnType {
//     let magic_method = PyMagicMethod::from_string(func_name.as_str());
// 
//     if let Some(magic_method) = magic_method {
//         return call_magic_method_of_pyobj_with_args(magic_method, pyobj_expr, args, arena);
//     }
//     
//     let pyobj = eval_expr(pyobj_expr, arena)?;
//     let mut pyargs = vec![];
//     
//     for arg in args {
//         pyargs.push(eval_expr(arg, arena)?);
//     }
// 
//     let func = pyobj.get_attribute(func_name.clone().as_str(), arena);
//     let mut func_args = vec![pyobj];
//     func_args.extend(pyargs);
// 
//     call_function(func, func_args, arena)
// }

// fn call_magic_method_of_pyobj_with_args(py_magic_method: &PyMagicMethod, pyobj_expr: &Expr, args: Vec<&Expr>, arena: &mut PyArena) -> FuncReturnType {
//     let pyobj = eval_expr(pyobj_expr, arena)?;
//     
//     let evaluated_args = eval_args(args, arena)?;
// 
//     let func = pyobj.get_magic_method(py_magic_method, arena).ok_or_else(|| { 
//         let message = format!("cannot find method '{py_magic_method}' in '{}' object", pyobj.clone_class(arena).get_name());
//         arena.exceptions.type_error.instantiate(message) 
//     })?;
//     
//     let func_args = [&[pyobj], &*pyargs].concat();
// 
//     call_function(func, &func_args, arena)
// }

fn eval_not(expr: &Expr, arena: &mut PyArena) -> FuncReturnType {
    let pyobj = eval_expr(expr, arena)?;

    let boolean = convert_pyobj_to_bool(&pyobj, arena)?;

    Ok(arena.statics.get_bool(!boolean).clone())
}

fn eval_and(expr1: &Expr, expr2: &Expr, arena: &mut PyArena) -> FuncReturnType {
    let pyobj1 = eval_expr(expr1, arena)?;
    let boolean1 = convert_pyobj_to_bool(&pyobj1, arena)?;

    if !boolean1 {
        return Ok(pyobj1);
    }

    let pyobj2 = eval_expr(expr2, arena)?;

    Ok(pyobj2)
}

fn eval_or(expr1: &Expr, expr2: &Expr, arena: &mut PyArena) -> FuncReturnType {
    let pyobj1 = eval_expr(expr1, arena)?;
    let boolean1 = convert_pyobj_to_bool(&pyobj1, arena)?;

    if boolean1 {
        return Ok(pyobj1);
    }

    let pyobj2 = eval_expr(expr2, arena)?;

    Ok(pyobj2)
}

fn eval_expr(expr: &Expr, arena: &mut PyArena) -> FuncReturnType {
    match expr {
        Expr::Var(name) => Ok(eval_var(name, arena)?.deref().clone()),
        Expr::Val(value) => Ok(eval_val(value, arena)),
        Expr::Times(first, second) => math_op(eval_expr(first, arena)?, eval_expr(second, arena)?, Mul {right: false}, arena),
        Expr::Divide(first, second) => math_op(eval_expr(first, arena)?, eval_expr(second, arena)?, TrueDiv {right: false}, arena), // TODO implement __div__ (prob not)
        Expr::Plus(first, second) => math_op(eval_expr(first, arena)?, eval_expr(second, arena)?, Add {right: false}, arena),
        Expr::Minus(first, second) => math_op(eval_expr(first, arena)?, eval_expr(second, arena)?, Sub {right: false}, arena),
        Expr::Pow(first, second) => math_op(eval_expr(first, arena)?, eval_expr(second, arena)?, Pow {right: false}, arena),
        Expr::Comparison(first, comp, second) => compare_op(&eval_expr(first, arena)?, &eval_expr(second, arena)?, comp, arena),
        Expr::FunCall(name, args) => eval_fun_call(name, args, arena),
        Expr::Not(expr) => eval_not(expr, arena),
        Expr::And(first, second) => eval_and(first, second, arena),
        Expr::Or(first, second) => eval_or(first, second, arena),
    }
}

fn eval_defn_var(variable: &Rc<Variable>, expr: &Expr, arena: &mut PyArena) -> EmptyFuncReturnType {
    let result = eval_expr(expr, arena)?;
    arena.get_current_frame_mut().set(variable, result);
    
    Ok(())
}

fn eval_op_equals(variable: &Rc<Variable>, expr: &Expr, op: PyMagicMethod, arena: &mut PyArena) -> EmptyFuncReturnType {
    // TODO check if in-place funcs are defined and use them if so
    let old_value = eval_var(variable, arena)?.clone();
    
    let new_value = math_op(old_value, eval_expr(expr, arena)?, op, arena)?;
    
    // arena.get_current_frame_mut().update_from_var(variable, new_value);
    arena.get_current_frame_mut().set(variable, new_value);
    
    Ok(())
}

fn eval_assert(expr1: &Expr, expr2: &Option<Expr>, arena: &mut PyArena) -> EmptyFuncReturnType {
    let result1 = eval_expr(expr1, arena)?;
    
    if let Some(expr2) = expr2 {
        let result2 = eval_expr(expr2, arena)?;
        
        let is_equal = compare_op(&result1, &result2, &Comparator::Equal, arena)?;
        
        if !convert_pyobj_to_bool(&is_equal, arena)? {
            let msg = py_repr(&result2, arena)?.expect_immutable().expect_string();
            return Err(arena.exceptions.assertion_error.instantiate(msg));
        }
        
        return Ok(());
    }
    
    if !convert_pyobj_to_bool(&result1, arena)? {
        return Err(arena.exceptions.assertion_error.empty());
    }
    
    Ok(())
}

fn eval_defn(define: &Define, arena: &mut PyArena) -> EmptyFuncReturnType {
    match define {
        Define::PlusEq(variable, expr) => eval_op_equals(variable, expr, Add {right: false}, arena),
        Define::MinusEq(variable, expr) => eval_op_equals(variable, expr, Add {right: false}, arena),
        Define::DivEq(variable, expr) => eval_op_equals(variable, expr, Add {right: false}, arena),
        Define::MultEq(variable, expr) => eval_op_equals(variable, expr, Add {right: false}, arena),
        Define::VarDefn(variable, expr) => { eval_defn_var(variable, expr, arena) },
        Define::FunDefn(variable, args, code, scope) => {todo!()}
    }
}

fn eval_for(iter_variable: &Rc<Variable>, iter: &Expr, code: &CodeBlock, arena: &mut PyArena) -> CodeBlockReturn {
    let iterable = eval_expr(iter, arena)?;
    let iter_func = iterable.get_magic_method(&PyMagicMethod::Iter, arena).unwrap();  // TODO Make python error
    
    let iterator = call_function(iter_func, &[iterable], arena)?; 
    
    let next_func = iterator.get_magic_method(&PyMagicMethod::Next, arena).unwrap_or_else(|| panic!("Iterator doesn't have __next__ method"));
    
    let mut next_func_rtn = call_function(next_func.clone(), &[iterator.clone()], arena);
    // let var_name = var.to_string();
    
    let none_value = arena.statics.none().clone();
    
    let current_frame = arena.get_current_frame_mut();
    
    current_frame.set(iter_variable, none_value);  // set value to None to ensure it's occupied
    
    let var_frame = current_frame.get(iter_variable).expect("This better be here").clone();
    let mut var_frame_ref = var_frame.borrow_mut();
    
    // let cell = arena.get_current_frame().get_cell(&iter_variable.name).expect("cell should exist").as_ptr();

    while let Ok(ref mut next_val) = next_func_rtn {
        if let PyObject::IteratorFlag(flag_type) = next_val {
            match flag_type {
                PyIteratorFlag::StopIteration => break,
                _ => panic!("IteratorFlag should only be StopIteration")
            }
        }
        
        // let mut var_ref = var_frame_ref.borrow_mut();
        *var_frame_ref = next_func.clone();
        
        // current_frame.set(iter_variable, next_val.clone());
        
        // unsafe {  // ! I think this is chill
        //     *cell = next_val.clone();
        // }
        
        let code_result = eval_code_block(code, arena)?;

        if let Some(code_rtn) = code_result {
            match code_rtn {
                PyObject::IteratorFlag(ref flag_type) => {
                    match flag_type {
                        PyIteratorFlag::Break => break,
                        PyIteratorFlag::Continue => continue,
                        _ => panic!("IteratorFlag should be Break or Continue")
                    }
                }
                _ => return Ok(Some(code_rtn))
            }
        }

        next_func_rtn = call_function(next_func.clone(), &[iterator.clone()], arena);
        
    };
    
    next_func_rtn?;
    
    Ok(None)
}

fn eval_while(condition: &Expr, code: &CodeBlock, arena: &mut PyArena) -> CodeBlockReturn {
    while convert_pyobj_to_bool(&eval_expr(condition, arena)?, arena)? {
        let return_value = eval_code_block(code, arena)?;

        if let Some(return_value) = return_value {
            match return_value {
                PyObject::IteratorFlag(ref flag_type) => {
                    match flag_type {
                        PyIteratorFlag::Break => break,
                        PyIteratorFlag::Continue => continue,
                        _ => panic!("IteratorFlag should be Break or Continue")
                    }
                }
                _ => return Ok(Some(return_value))
            }
        }

    }

    Ok(None)
}

fn eval_if(cond: &Expr, if_code: &CodeBlock, elif_cond_code: &Vec<(Expr, CodeBlock)>, else_code: &Option<CodeBlock>, arena: &mut PyArena) -> CodeBlockReturn {
    if convert_pyobj_to_bool(&eval_expr(cond, arena)?, arena)? {
        return eval_code_block(if_code, arena);
    }

    for (elif_cond, elif_code) in elif_cond_code {
        if convert_pyobj_to_bool(&eval_expr(elif_cond, arena)?, arena)? {
            return eval_code_block(elif_code, arena);
        }
    }

    if let Some(else_code) = else_code {
        return eval_code_block(else_code, arena);
    }

    Ok(None)
}

fn eval_code_block(code: &CodeBlock, arena: &mut PyArena) -> CodeBlockReturn {
    for statement in code.statements.iter() {
        let mut rtn_val: Option<PyObject> = None;
        match statement {
            Statement::Expr(expr) => { eval_expr(expr, arena)?; },
            Statement::Defn(define) => eval_defn(define, arena)?,
            Statement::If(cond, if_code, elif, else_code) => rtn_val = eval_if(cond, if_code, elif, else_code, arena)?,
            Statement::For(iter_var, iter_exp, code) => rtn_val = eval_for(iter_var, iter_exp, code, arena)?,
            Statement::While(condition, code) => rtn_val = eval_while(condition, code, arena)?,
            Statement::Return(rtn_expr) => rtn_val = Some(eval_expr(rtn_expr, arena)?),
            Statement::Assert(expr1, expr2) => eval_assert(expr1, expr2, arena)?,
            Statement::Continue => rtn_val = Some(PyObject::continue_()),
            Statement::Break => rtn_val = Some(PyObject::break_()),
        };

        if rtn_val.is_some() {
            return Ok(rtn_val);
        }
    };
    Ok(None)
}

type CodeBlockReturn = Result<Option<PyObject>, PyException>;