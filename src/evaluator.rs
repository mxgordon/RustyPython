use crate::builtins::function_utils::{call_function, eval_internal_func, eval_obj_init};
use crate::parser::*;
use crate::pyarena::PyArena;
use crate::builtins::pyobjects::*;

pub fn evaluate(code: CodeBlock) {
    let mut arena =  PyArena::new();
    
    eval_code_block(code, &mut arena);
}

fn eval_var(name: &str, arena: &PyArena) -> PyPointer<PyObject> {
    let var = arena.get(name);

    if var.is_none() {
        panic!("Variable {} not found", name); // TODO Make python error
    }

    var.unwrap().clone()
}

fn eval_val(value: &Value) -> PyPointer<PyObject> {
    match value {
        Value::Integer(value) => {
            PyPointer::new(PyObject::Int(value.clone()))
        }
        Value::Float(value) => {
            PyPointer::new(PyObject::Float(value.clone()))
        }
        Value::String(value) => {
            PyPointer::new(PyObject::Str(value.clone()))
        }
        Value::Boolean(value) => {
            PyPointer::new(PyObject::Bool(value.clone()))
        }
        Value::None => {
            PyPointer::new(PyObject::None)
        }
    }
}

fn eval_fun_call(func: &Box<Expr>, args: Vec<Expr>, arena: &mut PyArena) -> PyPointer<PyObject> {
    let func = eval_expr(&*func, arena);

    let py_args = args.iter().map(|arg| eval_expr(arg, arena)).collect();

    match **func.clone().borrow() {
        PyObject::Function(ref _func) => {
            todo!()
        }
        PyObject::InternalSlot(ref func) => {
            eval_internal_func(func.clone(), py_args)
        }
        PyObject::Class(ref pyclass) => {
            eval_obj_init(pyclass.clone(), py_args, arena)
        }
        _ => {
            panic!("{:?} is not a function", func); // TODO Make python error
        }
    }
}

fn call_function_of_pyobj_with_args(func_name: String, pyobj_expr: &Expr, args: Vec<&Expr>, arena: &mut PyArena) -> PyPointer<PyObject> {
    let pyobj = eval_expr(pyobj_expr, arena);
    let pyargs: Vec<PyPointer<PyObject>> = args.iter().map(|arg| eval_expr(arg, arena)).collect();

    let func = pyobj.borrow().get_attribute(func_name.clone()).unwrap_or_else(|| panic!("Object has no attribute {}", func_name)); // TODO Make python error
    let mut func_args = vec![pyobj];
    func_args.extend(pyargs);

    call_function(func, func_args, arena)
}

fn eval_expr(expr: &Expr, arena: &mut PyArena) -> PyPointer<PyObject> {
    match expr {
        Expr::Var(name) => eval_var(name, arena),
        Expr::Val(value) => eval_val(value),
        Expr::Times(_first, _second) => {todo!()}
        Expr::Divide(_first, _second) => {todo!()}
        Expr::Plus(first, second) => call_function_of_pyobj_with_args("__add__".to_string(), first, vec![second], arena),
        Expr::Minus(_first, _second) => {todo!()}
        Expr::Pow(first, second) => call_function_of_pyobj_with_args("__pow__".to_string(), first, vec![second], arena),
        Expr::FunCall(name, args) => eval_fun_call(name, args.clone(), arena)
    }
}

fn eval_defn_var(name: String, expr: &Expr, arena: &mut PyArena) {
    let result = eval_expr(expr, arena);
    arena.set(name, result);
}

fn eval_defn(define: &Define, arena: &mut PyArena) {
    match define {
        Define::PlusEq(var_name, expr) => {
            let other = eval_expr(expr, arena);
            let variable = eval_var(var_name, arena);
            
            let add_func = variable.borrow().get_attribute("__add__".to_string()).unwrap_or_else(|| panic!("Object has no __add__ method"));  // TODO Make python error
            
            let result = call_function(add_func, vec![variable, other], arena);
            
            arena.set(var_name.clone(), result);
        }
        Define::MinusEq(_, _) => {todo!()}
        Define::DivEq(_, _) => {todo!()}
        Define::MultEq(_, _) => {todo!()}
        Define::VarDefn(name, expr) => { eval_defn_var(name.clone(), expr, arena) },
        Define::FunDefn(_, _, _) => {todo!()}
    }
}

fn eval_for(var: &str, iter: &Expr, code: &CodeBlock, arena: &mut PyArena) -> Option<PyPointer<PyObject>> {
    let iterable = eval_expr(iter, arena);
    let iter_func = iterable.borrow().get_attribute("__iter__".to_string()).unwrap_or_else(|| panic!("Object is not iterable"));  // TODO Make python error
    
    let iterator = call_function(iter_func, vec![iterable], arena); 
    
    let next_func = iterator.borrow().get_attribute("__next__".to_string()).unwrap_or_else(|| panic!("Iterator doesn't have __next__ method"));
    
    let mut next_val = call_function(next_func.clone(), vec![iterator.clone()], arena);
    
    while !next_val.borrow().is_flag_type(PyFlag::StopIteration) {
        arena.set(var.to_string(), next_val.clone());

        let code_result = eval_code_block(code.clone(), arena);

        if let Some(code_rtn) = code_result {
            if code_rtn.borrow().is_not_flag() {
                return Some(code_rtn);
            }

            match **code_rtn.clone().borrow() {
                PyObject::InternalFlag(ref flag) => {
                    match **flag.borrow() {
                        PyFlag::StopIteration => {panic!("StopIteration flag should not be returned")},
                        PyFlag::GeneratorExit => {panic!("GeneratorExit flag should not be returned")},
                        PyFlag::Break => break,
                        PyFlag::Continue => continue,
                    }
                }
                _ => return Some(code_rtn)
            }
        }


        next_val = call_function(next_func.clone(), vec![iterator.clone()], arena);
    };
    arena.remove(var).unwrap_or_else(|| panic!("Variable {} not found", var)); // TODO Make python error

    None
}

fn eval_code_block(code: CodeBlock, arena: &mut PyArena) -> Option<PyPointer<PyObject>> {
    for statement in code.statements.iter() {
        let mut rtn_val: Option<PyPointer<PyObject>> = None;
        match statement {
            Statement::Expr(expr) => {
                let _result = eval_expr(expr, arena);
            }
            Statement::Defn(define) => eval_defn(define, arena),
            Statement::For(iter_var, iter_exp, code) => rtn_val = eval_for(iter_var, iter_exp, code, arena),
            Statement::Return(rtn_expr) => rtn_val = Some(eval_expr(rtn_expr, arena)),
            Statement::Continue => rtn_val = Some(PyPointer::new(PyObject::InternalFlag(PyPointer::new(PyFlag::Continue)))),
            Statement::Break => rtn_val = Some(PyPointer::new(PyObject::InternalFlag(PyPointer::new(PyFlag::Break)))),
        };

        if rtn_val.is_some() {
            return rtn_val;
        }
    };
    None
}