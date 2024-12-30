use crate::builtins::function_utils::{call_function, eval_internal_func, eval_obj_init};
use crate::builtins::structure::magic_methods::PyMagicMethod;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{FuncReturnType, PyIteratorFlag, PyObject, PyPointer};
use crate::parser::*;
use crate::pyarena::PyArena;

pub fn evaluate(code: CodeBlock) {
    let mut arena =  PyArena::new();
    
    eval_code_block(&code, &mut arena);
}

fn eval_var(name: &str, arena: &PyArena) -> FuncReturnType {
    arena.get(name).ok_or_else(|| arena.exceptions.name_error.instantiate(format!("name '{name}' is not defined")))
}

fn eval_val(value: &Value) -> PyPointer<PyObject> {
    match value {
        Value::Integer(value) => {
            PyPointer::new(PyObject::Int(*value))
        }
        Value::Float(value) => {
            PyPointer::new(PyObject::Float(*value))
        }
        Value::String(value) => {
            PyPointer::new(PyObject::Str(value.clone()))
        }
        Value::Boolean(value) => {
            PyPointer::new(PyObject::Bool(*value))
        }
        Value::None => {
            PyPointer::new(PyObject::None)
        }
    }
}

fn eval_fun_call(func: &Box<Expr>, args: &Vec<Expr>, arena: &mut PyArena) -> FuncReturnType {
    let func = eval_expr(&*func, arena)?;
    
    let mut py_args = vec![];
    
    for arg in args {
        py_args.push(eval_expr(arg, arena)?);
    }

    match *func.clone().borrow() {
        PyObject::Function(ref _func) => {
            todo!()
        }
        PyObject::InternalSlot(ref func) => {
            eval_internal_func(func.clone(), py_args, arena)
        }
        PyObject::Class(ref pyclass) => {
            eval_obj_init(pyclass.clone(), py_args, arena)
        }
        _ => {
            panic!("{:?} is not a function", func); // TODO Make python error
        }
    }
}

fn call_method_of_pyobj_with_args(func_name: String, pyobj_expr: &Expr, args: Vec<&Expr>, arena: &mut PyArena) -> FuncReturnType {
    let magic_method = PyMagicMethod::from_string(func_name.as_str());

    if let Some(magic_method) = magic_method {
        return call_magic_method_of_pyobj_with_args(magic_method, pyobj_expr, args, arena);
    }
    
    let pyobj = eval_expr(pyobj_expr, arena)?;
    let mut pyargs = vec![];
    
    for arg in args {
        pyargs.push(eval_expr(arg, arena)?);
    }

    let func = pyobj.borrow().get_attribute(func_name.clone().as_str(), arena);
    let mut func_args = vec![pyobj];
    func_args.extend(pyargs);

    call_function(func, func_args, arena)
}

fn call_magic_method_of_pyobj_with_args(py_magic_method: PyMagicMethod, pyobj_expr: &Expr, args: Vec<&Expr>, arena: &mut PyArena) -> FuncReturnType {
    let pyobj = eval_expr(pyobj_expr, arena)?;
    let mut pyargs = vec![];
    
    for arg in args {
        pyargs.push(eval_expr(arg, arena)?);
    }

    let func = pyobj.borrow().get_magic_method(py_magic_method, arena).ok_or_else(|| { 
        let message = format!("cannot find method '{py_magic_method}' in '{}' object", pyobj.borrow().get_class(arena).get_name());
        arena.exceptions.type_error.instantiate(message) 
    })?;
    
    let mut func_args = vec![pyobj];
    func_args.extend(pyargs);

    call_function(func, func_args, arena)
}

fn eval_expr(expr: &Expr, arena: &mut PyArena) -> FuncReturnType {
    match expr {
        Expr::Var(name) => eval_var(name, arena),
        Expr::Val(value) => Ok(eval_val(value)),
        Expr::Times(first, second) => {call_magic_method_of_pyobj_with_args(PyMagicMethod::Mul, first, vec![second], arena)}
        Expr::Divide(first, second) => {call_magic_method_of_pyobj_with_args(PyMagicMethod::TrueDiv, first, vec![second], arena)} // TODO implement __div__ (prob not)
        Expr::Plus(first, second) => call_magic_method_of_pyobj_with_args(PyMagicMethod::Add, first, vec![second], arena),
        Expr::Minus(first, second) => {call_magic_method_of_pyobj_with_args(PyMagicMethod::Sub, first, vec![second], arena)}
        Expr::Comparison(_first, _comp, _second) => {todo!()}
        Expr::Pow(first, second) => call_magic_method_of_pyobj_with_args(PyMagicMethod::Pow, first, vec![second], arena),
        Expr::FunCall(name, args) => eval_fun_call(name, args, arena)
    }
}

fn eval_defn_var(name: String, expr: &Expr, arena: &mut PyArena) -> Result<(), PyException> {
    let result = eval_expr(expr, arena)?;
    arena.set(name, result);
    
    Ok(())
}

fn eval_defn(define: &Define, arena: &mut PyArena) -> Result<(), PyException> {
    match define {
        Define::PlusEq(var_name, expr) => {
            let other = eval_expr(expr, arena)?;
            let variable = eval_var(var_name, arena)?;

            let add_func = variable.borrow().get_magic_method(PyMagicMethod::Add, arena).unwrap_or_else(|| panic!("Object has no __add__ method"));  // TODO Make python error

            let result = call_function(add_func, vec![variable, other], arena)?;

            arena.set(var_name.clone(), result);
            
            Ok(())
        }
        Define::MinusEq(_, _) => {todo!()}
        Define::DivEq(_, _) => {todo!()}
        Define::MultEq(_, _) => {todo!()}
        Define::VarDefn(name, expr) => { eval_defn_var(name.clone(), expr, arena) },
        Define::FunDefn(_, _, _) => {todo!()}
    }
}

fn eval_for(var: &str, iter: &Expr, code: &CodeBlock, arena: &mut PyArena) -> CodeBlockReturn {
    let iterable = eval_expr(iter, arena)?;
    let iter_func = iterable.borrow().get_magic_method(PyMagicMethod::Iter, arena).unwrap();  // TODO Make python error
    
    let iterator = call_function(iter_func, vec![iterable], arena)?; 
    
    let next_func = iterator.borrow().get_magic_method(PyMagicMethod::Next, arena).unwrap_or_else(|| panic!("Iterator doesn't have __next__ method"));
    
    let mut next_func_rtn = call_function(next_func.clone(), vec![iterator.clone()], arena);
    let var_name = var.to_string();


    while let Ok(ref mut next_val) = next_func_rtn {
        arena.set(var_name.clone(), next_val.clone());
        
        let code_result = eval_code_block(code, arena)?;

        if let Some(code_rtn) = code_result {
            match *code_rtn.borrow() {
                PyObject::IteratorFlag(ref flag_type) => {
                    match flag_type {
                        PyIteratorFlag::Break => break,
                        PyIteratorFlag::Continue => continue
                    }
                }
                _ => return Ok(Some(code_rtn.clone()))
            }
        }

        next_func_rtn = call_function(next_func.clone(), vec![iterator.clone()], arena);
        
    };
    arena.remove(var);  // clear iterator variable from scope
    
    let err_val = next_func_rtn.expect_err("next_func_rtn needs to end with a type of error");
    
    if !err_val.is_same_type(&*arena.exceptions.stop_iteration) {
        return Err(err_val);
    } 

    Ok(None)
}

fn eval_code_block(code: &CodeBlock, arena: &mut PyArena) -> CodeBlockReturn {
    for statement in code.statements.iter() {
        let mut rtn_val: Option<PyPointer<PyObject>> = None;
        match statement {
            Statement::Expr(expr) => {
                let _result = eval_expr(expr, arena);
            }
            Statement::Defn(define) => eval_defn(define, arena)?,
            Statement::For(iter_var, iter_exp, code) => rtn_val = eval_for(iter_var, iter_exp, code, arena)?,
            Statement::Return(rtn_expr) => rtn_val = Some(eval_expr(rtn_expr, arena)?),
            Statement::Continue => rtn_val = Some(PyPointer::new(PyObject::IteratorFlag(PyIteratorFlag::Continue))),
            Statement::Break => rtn_val = Some(PyPointer::new(PyObject::IteratorFlag(PyIteratorFlag::Break))),
        };

        if rtn_val.is_some() {
            return Ok(rtn_val);
        }
    };
    Ok(None)
}

type CodeBlockReturn = Result<Option<PyPointer<PyObject>>, PyException>;