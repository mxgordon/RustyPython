use rustpython_parser::ast::{BoolOp, CmpOp, Expr, ExprContext, Identifier, Mod, ModModule, Operator, Stmt, UnaryOp};
use rustpython_parser::text_size::TextRange;
use crate::builtins::functions::compare::compare_op;
use crate::builtins::functions::math_op::{math_op};
use crate::builtins::structure::magic_methods::PyMagicMethod;
use crate::builtins::structure::pyexception::PyException;
use crate::builtins::structure::pyobject::{PyObject, StatementOperation};
use crate::builtins::types::pybool::convert_pyobj_to_bool;
use crate::builtins::types::str::py_repr;
use crate::pyarena::PyArena;

pub fn evaluate_mod(code: Mod) {
    // let mut arena =  PyArena::new();
    let result = match code {
        Mod::Module(module  ) => {evaluate_module(module)},
        Mod::Interactive(_) => {todo!()}
        Mod::Expression(_) => {todo!()}
        Mod::FunctionType(_) => {todo!()}
    };

    // let code_result = eval_code_block(&code, &mut arena);

    // print!("Exit Code: {:?}", );

    // if let Err(err) = code_result {
    //     println!("{}", err);
    // }
}

fn evaluate_module(module: ModModule) -> Result<(PyArena, i32), PyException> {
    let mut arena =  PyArena::new();

    for statement in module.body {
        let statement_value = eval_stmt(statement);
    }

    Ok((arena, 0))
}

fn eval_stmt(statement: Stmt, arena: &mut PyArena) -> Result<StatementOperation, PyException> {
    Ok(match statement {
        Stmt::Break(..) => StatementOperation::Break,
        Stmt::Continue(..) => StatementOperation::Continue,
        Stmt::Pass(..) => StatementOperation::Pass,
        Stmt::Return(stmt_return) => StatementOperation::Return(eval_return_stmt(stmt_return.value, stmt_return.range, arena)?),
        Stmt::Expr(stmt_expr) => StatementOperation::Expression(eval_expr(*stmt_expr.value, arena)?),
        Stmt::While(_) => {}
        Stmt::If(_) => {}
        Stmt::For(_) => {}
        Stmt::Assign(_) => {}
        Stmt::FunctionDef(_) => {}
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

fn eval_return_stmt(value: Option<Box<Expr>>, _range: TextRange, arena: &mut PyArena) -> Result<Option<PyObject>, PyException> {
    if let Some(expr) = value {
        return Ok(Some(eval_expr(*expr, arena)?));
    }
    Ok(None)
}

fn eval_expr(expr: Expr, arena: &mut PyArena) -> Result<PyObject, PyException> {
    match expr {
        Expr::Name(name) => eval_name(name.id, name.ctx, name.range, arena).cloned(),
        Expr::BoolOp(expr_bool_op) => eval_bool_expr(expr_bool_op.op, expr_bool_op.values, expr_bool_op.range, arena),
        Expr::BinOp(expr_bin_op) => eval_bin_op(expr_bin_op.left, expr_bin_op.op, expr_bin_op.right, expr_bin_op.range, arena),
        Expr::UnaryOp(expr_unary_op) => eval_unary_op(expr_unary_op.operand, expr_unary_op.op, expr_unary_op.range, arena),
        Expr::Compare(expr_comp_op) => eval_compare_op(expr_comp_op.left, expr_comp_op.comparators, expr_comp_op.ops, expr_comp_op.range, arena),
        Expr::Constant(_) => {}
        Expr::Call(_) => {}
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

fn eval_compare_op(left: Box<Expr>, comparators: Vec<Expr>, ops: Vec<CmpOp>, _range: TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    let mut left_value = eval_expr(*left, arena)?;

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

fn eval_unary_op(operand: Box<Expr>, op: UnaryOp, _range: TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    let operand_value = eval_expr(*operand, arena)?;
    match op {
        UnaryOp::Not => {todo!()}
        UnaryOp::Invert => {todo!()}
        UnaryOp::UAdd => {todo!()}
        UnaryOp::USub => {todo!()}
    }
}

fn eval_bin_op(left: Box<Expr>, op: Operator, right: Box<Expr>, _range: TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    let left_value = eval_expr(*left, arena)?;
    let right_value = eval_expr(*right, arena)?;
    
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

fn eval_bool_expr(op: BoolOp, values: Vec<Expr>, _range: TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
    match op {
        BoolOp::And => eval_bool_and(values, _range, arena),
        BoolOp::Or => eval_bool_or(values, _range, arena),
    }
}

fn eval_bool_and(values: Vec<Expr>, _range: TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
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

fn eval_bool_or(values: Vec<Expr>, _range: TextRange, arena: &mut PyArena) -> Result<PyObject, PyException> {
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

fn eval_name(id: Identifier, _ctx: ExprContext, _range: TextRange, arena: &mut PyArena) -> Result<&PyObject, PyException> {
    arena.get(&id).ok_or_else(|| arena.exceptions.name_error.instantiate(format!("name '{}' is not defined", id)))
}