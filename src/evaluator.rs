use std::any::Any;
use std::sync::Arc;
use crate::builtins::function_utils::call_function;
use crate::parser::*;
use crate::pyarena::PyArena;
use crate::builtins::pyobjects::*;

pub fn evaluate(code: CodeBlock) {
    let mut arena =  PyArena::new();
    
    arena.load_builtins();
    
    eval_code_block(code, &mut arena);
}

fn eval_var(name: &str, arena: &PyArena) -> Arc<PyObject> {
    let var = arena.get(name);

    if var.is_none() {
        panic!("Variable {} not found", name); // TODO Make python error
    }

    var.unwrap().clone()
}

fn eval_val(value: &Value) -> Arc<PyObject> {
    match value {
        Value::Integer(value) => {
            Arc::new(PyObject::Int(value.clone()))
        }
        Value::Float(value) => {
            Arc::new(PyObject::Float(value.clone()))
        }
        Value::String(value) => {
            Arc::new(PyObject::Str(value.clone()))
        }
        Value::Boolean(value) => {
            Arc::new(PyObject::Bool(value.clone()))
        }
        Value::None => {
            Arc::new(PyObject::None)
        }
    }
}

pub(crate) fn eval_obj_init(pyclass: Arc<PyClass>, args: Vec<Arc<PyObject>>, arena: &mut PyArena) -> Arc<PyObject> {
    let new_func = pyclass.search_for_attribute("__new__".to_string());
    let init_func = pyclass.search_for_attribute("__init__".to_string());

    if new_func.is_none() {
        panic!("Class has no __new__ method"); // TODO Make python error
    } else if init_func.is_none() {
        panic!("Class has no __init__ method")
    }

    let new_func = new_func.unwrap();
    let init_func = init_func.unwrap();
    
    let mut new_args = vec![Arc::new(PyObject::Class(pyclass.clone())) ];
    new_args.extend(args.clone());
    
    let new_object = call_function(new_func, new_args, arena);
    
    let mut init_args = vec![new_object.clone()];
    init_args.extend(args.clone());
    
    let _init_rtn = call_function(init_func, init_args, arena); // TODO assert its None
    
    new_object
}

fn eval_fun_call(func: &Box<Expr>, args: Vec<Expr>, arena: &mut PyArena) -> Arc<PyObject> {
    let func = eval_expr(&*func, arena);

    let py_args = args.iter().map(|arg| eval_expr(arg, arena)).collect();

    match &*func {
        PyObject::Function(_func) => {
            todo!()
        }
        PyObject::InternalSlot(func) => {
            eval_internal_func(func.clone(), py_args, arena)
        }
        PyObject::Class(pyclass) => {
            eval_obj_init(pyclass.clone(), py_args, arena)
        }
        _ => {
            panic!("{:?} is not a function", func); // TODO Make python error
        }
    }
}

pub(crate) fn eval_internal_func(func: Arc<PyInternalFunction>, args: Vec<Arc<PyObject>>, _py_arena: &mut PyArena) -> Arc<PyObject> {
    match (&*func, args.len()) {
        (PyInternalFunction::NoArgs(func), 0) => {
            func()
        }
        (PyInternalFunction::OneArg(func), 1) => {
            func(args[0].clone())
        }
        (PyInternalFunction::TwoArgs(func), 2) => {
            func(args[0].clone(), args[1].clone())
        }
        (PyInternalFunction::ThreeArgs(func), 3) => {
            func(args[0].clone(), args[1].clone(), args[2].clone())
        }
        (PyInternalFunction::ManyArgs(func), _n) => {
            func(args)
        }
        (internal_function_type, n) => {
            panic!("Trying to call {:?} function type with {} arguments", internal_function_type, n); // TODO Make python error
        }
    }
}

fn call_function_of_pyobj_with_args(func_name: String, pyobj_expr: &Expr, args: Vec<&Expr>, arena: &mut PyArena) -> Arc<PyObject> {
    let pyobj = eval_expr(pyobj_expr, arena);
    let pyargs: Vec<Arc<PyObject>> = args.iter().map(|arg| eval_expr(arg, arena)).collect();
    
    let func = pyobj.get_attribute(func_name.clone()).unwrap_or_else(|| panic!("Object has no attribute {}", func_name)); // TODO Make python error
    let mut func_args = vec![pyobj]; 
    func_args.extend(pyargs);
    
    call_function(func, func_args, arena)
}

fn eval_expr(expr: &Expr, arena: &mut PyArena) -> Arc<PyObject> {
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
        Define::PlusEq(_, _) => {todo!()}
        Define::MinusEq(_, _) => {todo!()}
        Define::DivEq(_, _) => {todo!()}
        Define::MultEq(_, _) => {todo!()}
        Define::VarDefn(name, expr) => { eval_defn_var(name.clone(), expr, arena) },
        Define::FunDefn(_, _, _) => {todo!()}
    }
}

fn eval_for(var: &str, iter: &Expr, code: &CodeBlock, arena: &mut PyArena) -> Option<Arc<PyObject>> {
    let iterable = eval_expr(iter, arena);
    let iter_func = iterable.get_attribute("__iter__".to_string()).unwrap_or_else(|| panic!("Object is not iterable"));  // TODO Make python error
    
    let iterator = call_function(iter_func, vec![iterable], arena); 
    
    let next_func = iterator.get_attribute("__next__".to_string()).unwrap_or_else(|| panic!("Iterator doesn't have __next__ method"));
    
    let mut next_val = call_function(next_func.clone(), vec![iterator.clone()], arena);
    
    while !next_val.is_flag_type(PyFlag::StopIteration) {
        next_val = call_function(next_func.clone(), vec![iterator.clone()], arena);

        arena.set(var.to_string(), next_val.clone());

        let code_result = eval_code_block(code.clone(), arena);

        if let Some(code_rtn) = code_result {
            if code_rtn.is_not_flag() {
                return Some(code_rtn);
            }
            
            match &*code_rtn {
                PyObject::InternalFlag(flag) => {
                    match &**flag {
                        PyFlag::StopIteration => {panic!("StopIteration flag should not be returned")},
                        PyFlag::GeneratorExit => {panic!("GeneratorExit flag should not be returned")},
                        PyFlag::Break => break,
                        PyFlag::Continue => continue,
                    }
                }
                _ => return Some(code_rtn)
            }
        }
    };
    arena.remove(var).unwrap_or_else(|| panic!("Variable {} not found", var)); // TODO Make python error

    None
}

fn eval_code_block(code: CodeBlock, arena: &mut PyArena) -> Option<Arc<PyObject>> {
    for statement in code.statements.iter() {
        let mut rtn_val: Option<Arc<PyObject>> = None;
        match statement {
            Statement::Expr(expr) => {
                let _result = eval_expr(expr, arena);
            }
            Statement::Defn(define) => eval_defn(define, arena),
            Statement::For(iter_var, iter_exp, code) => rtn_val = eval_for(iter_var, iter_exp, code, arena),
            Statement::Return(rtn_expr) => rtn_val = Some(eval_expr(rtn_expr, arena)),
            Statement::Continue => rtn_val = Some(Arc::new(PyObject::InternalFlag(Arc::new(PyFlag::Continue)))),
            Statement::Break => rtn_val = Some(Arc::new(PyObject::InternalFlag(Arc::new(PyFlag::Break)))),
        };

        if rtn_val.is_some() {
            return rtn_val;
        }
    };
    None
}