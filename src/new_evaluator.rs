use std::str::FromStr;
use malachite::Integer;
use rustpython_parser::ast::{BoolOp, CmpOp, Constant, Expr, ExprContext, ExprName, Identifier, Keyword, Mod, ModModule, Operator, Stmt, UnaryOp};
use rustpython_parser::text_size::TextRange;
use crate::builtins::function_utils::{call_function, call_function_1_arg_min};
use crate::builtins::functions::compare::compare_op;
use crate::builtins::functions::math_op::{math_op};
use crate::builtins::structure::magic_methods::PyMagicMethod;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{PyObject, StatementOperation};
use crate::builtins::types::pybool::convert_pyobj_to_bool;
use crate::pyarena::PyArena;

pub fn evaluate_mod(code: Mod) {
    // let mut arena =  PyArena::new();
    let result = match code {
        Mod::Module(module  ) => {evaluate_module(module)},
        Mod::Interactive(_) => {todo!()}
        Mod::Expression(_) => {todo!()}
        Mod::FunctionType(_) => {todo!()}
    };

    if let Ok(_arena) = result {
        println!("Program exited successfully");
    } else if let Err(err) = result {
        println!("Program had fatal error\n{}", err);
    }

    // let code_result = eval_code_block(&code, &mut arena);

    // print!("Exit Code: {:?}", );

    // if let Err(err) = code_result {
    //     println!("{}", err);
    // }
}

fn evaluate_module(module: ModModule) -> Result<PyArena, PyException> {
    let mut arena =  PyArena::new();

    eval_block(&module.body, &mut arena)?;

    Ok(arena)
}

fn eval_block(code: &Vec<Stmt>, arena: &mut PyArena) -> Result<StatementOperation, PyException> {
    for statement in code {
        let statement_value = eval_stmt(statement, arena)?;

        match &statement_value {
            StatementOperation::Pass => continue,
            StatementOperation::Normal => continue,
            StatementOperation::Expression(..) => continue,
            StatementOperation::Return(..) => return Ok(statement_value),
            StatementOperation::Continue => return Ok(statement_value),
            StatementOperation::Break => return Ok(statement_value),
        }
    }

    Ok(StatementOperation::Normal)
}

fn eval_stmt(statement: &Stmt, arena: &mut PyArena) -> Result<StatementOperation, PyException> {
    Ok(match statement {
        Stmt::Break(..) => StatementOperation::Break,
        Stmt::Continue(..) => StatementOperation::Continue,
        Stmt::Pass(..) => StatementOperation::Pass,
        Stmt::Return(stmt_return) => StatementOperation::Return(eval_return_stmt(&stmt_return.value, stmt_return.range, arena)?),
        Stmt::Expr(stmt_expr) => StatementOperation::Expression(eval_expr(&stmt_expr.value, arena)?),
        Stmt::While(stmt_while) => eval_stmt_while(&stmt_while.test, &stmt_while.body, &stmt_while.orelse, stmt_while.range, arena)?,
        Stmt::If(stmt_if) => eval_stmt_if(&stmt_if.test, &stmt_if.body, &stmt_if.orelse, stmt_if.range, arena)?,
        Stmt::For(stmt_for) => eval_stmt_for(&stmt_for.target, &stmt_for.iter, &stmt_for.body, &stmt_for.orelse, stmt_for.range, arena)?,
        Stmt::Assign(stmt_assign) => eval_stmt_assign(&stmt_assign.targets, &stmt_assign.value, stmt_assign.range, arena)?,
        Stmt::FunctionDef(_) => {todo!()}
        // not implemented
        Stmt::AsyncFunctionDef(_) => {todo!()}
        Stmt::ClassDef(_) => {todo!()}
        Stmt::Delete(_) => {todo!()}
        Stmt::TypeAlias(_) => {todo!()}
        Stmt::AugAssign(_) => {todo!()}
        Stmt::AnnAssign(_) => {todo!()}
        Stmt::AsyncFor(_) => {todo!()}
        Stmt::With(_) => {todo!()}
        Stmt::AsyncWith(_) => {todo!()}
        Stmt::Match(_) => {todo!()}
        Stmt::Raise(_) => {todo!()}
        Stmt::Try(_) => {todo!()}
        Stmt::TryStar(_) => {todo!()}
        Stmt::Assert(_) => {todo!()}
        Stmt::Import(_) => {todo!()}
        Stmt::ImportFrom(_) => {todo!()}
        Stmt::Global(_) => {todo!()}
        Stmt::Nonlocal(_) => {todo!()}
    })
}

fn eval_stmt_assign(targets: &Vec<Expr>, value: &Box<Expr>, _range: TextRange, arena: &mut PyArena) -> Result<StatementOperation, PyException> {
    let target_name = expect_name(&targets[0], arena)?; // TODO no support for multiple assignment yet

    let value_value = eval_expr(value, arena)?;

    arena.set(target_name.id.to_string(), value_value);

    Ok(StatementOperation::Normal)
}

fn eval_stmt_for(target: &Box<Expr>, iterable: &Box<Expr>, body: &Vec<Stmt>, else_body: &Vec<Stmt>, _range: TextRange, arena: &mut PyArena) -> Result<StatementOperation, PyException> {
    let target_name = expect_name(target.as_ref(), arena)?;
    let iterable_value = eval_expr(iterable, arena)?;
    let iterator_func = iterable_value.get_magic_method(&PyMagicMethod::Iter, arena).unwrap();  // TODO Make python error
    let iterator = call_function_1_arg_min(&iterator_func, &iterable_value, &[], arena)?;

    let next_func = iterator.get_magic_method(&PyMagicMethod::Next, arena).expect("Iterator doesn't have __next__ method");

    let mut next_func_rtn = call_function_1_arg_min(&next_func, &iterator, &[], arena);

    while let Ok(next_func_rtn_ok) = next_func_rtn {
        arena.set(target_name.id.to_string(), next_func_rtn_ok);

        let body_value = eval_block(body, arena)?;

        if let StatementOperation::Return(..) = body_value {
            return Ok(body_value);
        }

        next_func_rtn = call_function_1_arg_min(&next_func, &iterator, &[], arena);

        match body_value {
            StatementOperation::Pass => continue,
            StatementOperation::Normal => continue,
            StatementOperation::Expression(..) => continue,
            StatementOperation::Continue => continue,
            StatementOperation::Break => break,
            StatementOperation::Return(..) => panic!("This branch should not be reached"),
        };

    }

    if let Err(next_func_rtn_err) = next_func_rtn {
        if !next_func_rtn_err.is_same_type(&arena.exceptions.stop_iteration) {
            return Err(next_func_rtn_err);
        }

        let else_value = eval_block(else_body, arena)?;

        return match else_value {
            StatementOperation::Pass => Ok(StatementOperation::Normal),
            StatementOperation::Normal => Ok(StatementOperation::Normal),
            StatementOperation::Expression(..) => Ok(StatementOperation::Normal),
            StatementOperation::Continue => Ok(else_value),
            StatementOperation::Break => Ok(else_value),
            StatementOperation::Return(..) => Ok(else_value),
        }
    }

    Ok(StatementOperation::Normal)
}

fn expect_name<'a>(name_expr: &'a Expr, arena: &mut PyArena) -> Result<&'a ExprName, PyException> {
    match name_expr {
        Expr::Name(expr_name) => Ok(&expr_name),
        _ => Err(arena.exceptions.syntax_error.instantiate(format!("expected name, got {:?}", name_expr)))  // TODO make real python error
    }
}

fn eval_stmt_if(test_cond: &Box<Expr>, body: &Vec<Stmt>, else_body: &Vec<Stmt>, _range: TextRange, arena: &mut PyArena) -> Result<StatementOperation, PyException> {
    if convert_pyobj_to_bool(&eval_expr(test_cond, arena)?, arena)? {
        eval_block(body, arena)
    } else {
        eval_block(else_body, arena)
    }
}

fn eval_stmt_while(test_cond: &Box<Expr>, body: &Vec<Stmt>, else_body: &Vec<Stmt>, _range: TextRange, arena: &mut PyArena) -> Result<StatementOperation, PyException> {
    let mut run_else_block = true;

    while convert_pyobj_to_bool(&eval_expr(test_cond, arena)?, arena)? {
        let body_value = eval_block(body, arena)?;

        match &body_value {
            StatementOperation::Pass => continue,
            StatementOperation::Normal => continue,
            StatementOperation::Expression(..) => continue,
            StatementOperation::Continue => continue,
            StatementOperation::Break => {
                run_else_block = false;
                break; },
            StatementOperation::Return(..) => return Ok(body_value),
        }
    }

    if run_else_block {
        let else_value = eval_block(else_body, arena)?;

        return match else_value {
            StatementOperation::Pass => Ok(StatementOperation::Normal),
            StatementOperation::Normal => Ok(StatementOperation::Normal),
            StatementOperation::Expression(..) => Ok(StatementOperation::Normal),
            StatementOperation::Continue => Ok(else_value),
            StatementOperation::Break => Ok(else_value),
            StatementOperation::Return(..) => Ok(else_value),
        }
    }

    Ok(StatementOperation::Normal)
}


fn eval_return_stmt(value: &Option<Box<Expr>>, _range: TextRange, arena: &mut PyArena) -> Result<Option<PyObject>, PyException> {
    if let Some(expr) = value {
        return Ok(Some(eval_expr(&*expr, arena)?));
    }
    Ok(None)
}

fn eval_expr(expr: &Expr, arena: &mut PyArena) -> Result<PyObject, PyException> {
    match expr {
        Expr::Name(name) => eval_name(&name.id, &name.ctx, &name.range, arena).cloned(),
        Expr::BoolOp(expr_bool_op) => eval_bool_expr(&expr_bool_op.op, &expr_bool_op.values, &expr_bool_op.range, arena),
        Expr::BinOp(expr_bin_op) => eval_bin_op(&expr_bin_op.left, &expr_bin_op.op, &expr_bin_op.right, &expr_bin_op.range, arena),
        Expr::UnaryOp(expr_unary_op) => eval_unary_op(&expr_unary_op.operand, &expr_unary_op.op, &expr_unary_op.range, arena),
        Expr::Compare(expr_comp_op) => eval_compare_op(&expr_comp_op.left, &expr_comp_op.comparators, &expr_comp_op.ops, &expr_comp_op.range, arena),
        Expr::Constant(expr_const) => eval_constant(&expr_const.value, &expr_const.range, arena),
        Expr::Call(expr_call) => eval_func_call(&expr_call.func, &expr_call.args, &expr_call.keywords, &expr_call.range, arena),
        Expr::NamedExpr(_) => {todo!()}//doable in current config but focussing on other things first


        Expr::IfExp(_) => {todo!()}
        Expr::Lambda(_) => {todo!()}
        Expr::Dict(_) => {todo!()}
        Expr::Set(_) => {todo!()}
        Expr::ListComp(_) => {todo!()}
        Expr::SetComp(_) => {todo!()}
        Expr::DictComp(_) => {todo!()}
        Expr::GeneratorExp(_) => {todo!()}
        Expr::Await(_) => {todo!()}
        Expr::Yield(_) => {todo!()}
        Expr::YieldFrom(_) => {todo!()}
        Expr::FormattedValue(_) => {todo!()}
        Expr::JoinedStr(_) => {todo!()}
        Expr::Attribute(_) => {todo!()}
        Expr::Subscript(_) => {todo!()}
        Expr::Starred(_) => {todo!()}
        Expr::List(_) => {todo!()}
        Expr::Tuple(_) => {todo!()}
        Expr::Slice(_) => {todo!()}
    }
}

fn eval_func_call(func: &Box<Expr>, args: &Vec<Expr>, keywords: &Vec<Keyword>, _range: &TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    let func_value = eval_expr(&*func, arena)?;
    let func_args = args.into_iter().map(|arg| eval_expr(arg, arena)).collect::<Result<Vec<PyObject>, PyException>>()?;
    let func_kwargs = keywords.into_iter().map(|kwarg| eval_keyword(&kwarg.arg, &kwarg.value, &kwarg.range, arena)).collect::<Result<Vec<(Option<&Identifier>, PyObject)>, PyException>>()?;

    call_function(func_value, &func_args, arena) // TODO include kwargs
}

fn eval_keyword<'a>(arg_name: &'a Option<Identifier>, value: &Expr, _range: &TextRange, arena: &mut PyArena) -> Result<(Option<&'a Identifier>, PyObject), PyException> {
    let keyword_value = eval_expr(value, arena)?;
    Ok((arg_name.as_ref(), keyword_value))
}

fn eval_constant(constant: &Constant, _range: &TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    match constant {
        Constant::None => Ok(arena.statics.none().clone()),
        Constant::Bool(bool_value) => Ok(arena.statics.get_bool(*bool_value).clone()),
        Constant::Int(int_value) => Ok(PyObject::new_int(Integer::from_str(&int_value.to_string()).unwrap())), // TODO avoid using strings here, also rewrite ast to not have old malachite
        Constant::Float(float_value) => Ok(PyObject::new_float(*float_value)),
        Constant::Str(_) => {todo!()}
        Constant::Bytes(_) => {todo!()}
        Constant::Tuple(_) => {todo!()}
        Constant::Complex { .. } => {todo!()}
        Constant::Ellipsis => {todo!()}
    }
}

fn eval_compare_op(left: &Box<Expr>, comparators: &Vec<Expr>, ops: &Vec<CmpOp>, _range: &TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    let mut left_value = eval_expr(left, arena)?;

    for (comparator, op) in comparators.into_iter().zip(ops.iter()) {
        let right_value = eval_expr(comparator, arena)?;

        let compare_result = compare_op(&left_value, &right_value, op, arena)?;

        if !convert_pyobj_to_bool(&compare_result, arena)? {
            return Ok(arena.statics.get_bool(false).clone());
        }

        left_value = right_value;
    }

    Ok(arena.statics.get_bool(true).clone()) // TODO prob is supposed to return the output of the comparison
}

fn eval_unary_op(operand: &Box<Expr>, op: &UnaryOp, _range:& TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    let operand_value = eval_expr(operand, arena)?;
    match op {
        UnaryOp::Not => {todo!()}
        UnaryOp::Invert => {todo!()}
        UnaryOp::UAdd => {todo!()}
        UnaryOp::USub => {todo!()}
    }
}

fn eval_bin_op(left: &Box<Expr>, op: &Operator, right: &Box<Expr>, _range: &TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    let left_value = eval_expr(&*left, arena)?;
    let right_value = eval_expr(&*right, arena)?;
    
    match op {
        Operator::Add => math_op(left_value, right_value, PyMagicMethod::Add {right: false}, arena),
        Operator::Sub => math_op(left_value, right_value, PyMagicMethod::Sub {right: false}, arena),
        Operator::Mult => math_op(left_value, right_value, PyMagicMethod::Mul {right: false}, arena),
        Operator::Div => math_op(left_value, right_value, PyMagicMethod::TrueDiv {right: false}, arena),
        Operator::Pow => math_op(left_value, right_value, PyMagicMethod::Pow {right: false}, arena),
        Operator::MatMult => {todo!()}
        Operator::Mod => {todo!()}
        Operator::LShift => {todo!()}
        Operator::RShift => {todo!()}
        Operator::BitOr => {todo!()}
        Operator::BitXor => {todo!()}
        Operator::BitAnd => {todo!()}
        Operator::FloorDiv => {todo!()}
    }
}

fn eval_bool_expr(op: &BoolOp, values: &Vec<Expr>, _range: &TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    match op {
        BoolOp::And => eval_bool_and(values, _range, arena),
        BoolOp::Or => eval_bool_or(values, _range, arena),
    }
}

fn eval_bool_and(values: &Vec<Expr>, _range: &TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    let mut value = None;

    for  expr in values {
        let expr_value = eval_expr(expr, arena)?;
        let expr_bool = convert_pyobj_to_bool(&expr_value, arena)?;

        if !expr_bool {
            return Ok(expr_value);
        }
        
        value = Some(expr_value);
    }
    
    Ok(value.expect("Must have been initialized"))
}

fn eval_bool_or(values: &Vec<Expr>, _range: &TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    let mut value = None;

    for  expr in values {
        let expr_value = eval_expr(expr, arena)?;
        let expr_bool = convert_pyobj_to_bool(&expr_value, arena)?;

        if expr_bool {
            return Ok(expr_value);
        }
        
        value = Some(expr_value);
    }
    
    Ok(value.expect("Must have been initialized"))
}

fn eval_name<'a>(id: &'a Identifier, _ctx: &ExprContext, _range: &TextRange, arena: &'a mut PyArena) -> Result<&'a PyObject, PyException> {
    arena.get(&id).ok_or_else(|| arena.exceptions.name_error.instantiate(format!("name '{}' is not defined", id)))
}