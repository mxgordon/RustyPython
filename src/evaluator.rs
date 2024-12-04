use std::any::Any;
use std::sync::Arc;
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

fn eval_val(value: &Value, arena: &PyArena) -> Arc<PyObject> {
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

fn eval_obj_init(pyclass: Arc<PyClass>, args: Vec<Arc<PyObject>>, arena: &mut PyArena) -> Arc<PyObject> {
    let new = pyclass.search_for_attribute("__new__".to_string());
    let init = pyclass.search_for_attribute("__init__".to_string());

    if new.is_none() {
        panic!("Class has no __new__ method"); // TODO Make python error
    } else if init.is_none() {
        panic!("Class has no __init__ method")
    }

    let new = new.unwrap();
    let init = init.unwrap();
    
    let mut new_args = vec![Arc::new(PyObject::Class(pyclass.clone())) ];
    new_args.extend(args.clone());
    
    let new_object = call_function(new, new_args, arena);
    
    let mut init_args = vec![new_object.clone()];
    init_args.extend(args.clone());
    
    let _init_rtn = call_function(init, init_args, arena); // TODO assert its None
    
    new_object
}

fn call_function(func: Arc<PyObject>, args: Vec<Arc<PyObject>>, arena: &mut PyArena) -> Arc<PyObject> {


    match &*func {
        PyObject::Function(func) => {
            todo!()
        }
        PyObject::InternalSlot(func) => {
            println!("{:?}", arena);
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

fn eval_fun_call(func: &Box<Expr>, args: Vec<Expr>, arena: &mut PyArena) -> Arc<PyObject> {
    // let func = arena.get(name);
    let func = eval_expr(&*func, arena);
;
    let py_args = args.iter().map(|arg| eval_expr(arg, arena)).collect();

    match &*func {
        PyObject::Function(func) => {
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

fn eval_internal_func(func: Arc<PyInternalFunction>, args: Vec<Arc<PyObject>>, py_arena: &mut PyArena) -> Arc<PyObject> {
    // let arena_scope = py_arena.clone();

    // let py_args: Vec<Arc<PyObject>> = args.iter().map(|arg| eval_expr(arg, py_arena)).collect();

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
        (PyInternalFunction::ManyArgs(func), n) => {
            func(args)
        }
        (internal_function_type, n) => {
            panic!("Trying to call {:?} function type with {} arguments", internal_function_type.type_id(), n); // TODO Make python error
        }
    }
}



fn eval_expr(expr: &Expr, arena: &mut PyArena) -> Arc<PyObject> {
    match expr {
        Expr::Var(name) => eval_var(name, arena),
        Expr::Val(value) => eval_val(value, arena),
        Expr::Times(_first, _second) => {todo!()}
        Expr::Divide(_first, _second) => {todo!()}
        Expr::Plus(_first, _second) => {todo!()}
        Expr::Minus(_first, _second) => {todo!()}
        Expr::Pow(_first, _second) => {todo!()}
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

fn eval_code_block(code: CodeBlock, arena: &mut PyArena) {
    for statement in code.statements.iter() {
        match statement {
            Statement::Expr(expr) => {
                let result = eval_expr(expr, arena);
                // println!("{:?}", result);
            }
            Statement::Defn(define) => eval_defn(define, arena),
            Statement::For(_, _, _) => todo!()
        }
    }
}