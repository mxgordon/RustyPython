use std::env::var;
use std::rc::Rc;
use ahash::AHashMap;
use crate::parser::{CodeBlock, Define, Expr, ScopeInformation, Statement, Variable};

fn add_var_access(variable: &Rc<Variable>, scope: &mut AHashMap<String, ScopeInformation>) {
    let scope_info = scope.get_mut(&variable.name).expect("should already be in scope map");

    scope_info.uses += 1;
}

fn add_var_def(variable: &Rc<Variable>, scope: &mut AHashMap<String, ScopeInformation>) {
    if let Some(scope_info) = scope.get_mut(&variable.name) {
        scope_info.has_definition = true;
    } else {
        scope.insert(variable.name.clone(), ScopeInformation {variable: variable.clone(), uses: 0, has_definition: true});
    }
}
pub fn preprocess_code(code_block: &mut CodeBlock, scope: &mut AHashMap<String, ScopeInformation>) {
    for statement in code_block.statements.iter_mut() {
        match statement {
            Statement::Expr(expr) => { preprocess_expr(expr, scope) },
            Statement::Defn(define) => { preprocess_defn(define, scope) },
            Statement::If(cond, if_code, elif, else_code) => {
                preprocess_expr(cond, scope);
                preprocess_code(if_code, scope);

                for (elif_cond, elif_code) in elif {
                    preprocess_expr(elif_cond, scope);
                    preprocess_code(elif_code, scope);
                }

                    
                
            },
            Statement::For(iter_var, iter_exp, code) => {},
            Statement::While(condition, code) => {},
            Statement::Return(rtn_expr) => {},
            Statement::Assert(expr1, expr2) => {},
            Statement::Continue => {},
            Statement::Break => {},
        };
    };
}

fn preprocess_expr(expr: &mut Expr, scope: &mut AHashMap<String, ScopeInformation>) {
    match expr {
        Expr::Var(var) => add_var_access(var, scope),
        Expr::Val(_val) => {}
        Expr::Times(expr1, expr2) => {
            preprocess_expr(&mut **expr1, scope);
            preprocess_expr(&mut **expr2, scope);
        }
        Expr::Divide(expr1, expr2) => {
            preprocess_expr(&mut **expr1, scope);
            preprocess_expr(&mut **expr2, scope);
        }
        Expr::Plus(expr1, expr2) => {
            preprocess_expr(&mut **expr1, scope);
            preprocess_expr(&mut **expr2, scope);
        }
        Expr::Minus(expr1, expr2) => {
            preprocess_expr(&mut **expr1, scope);
            preprocess_expr(&mut **expr2, scope);
        }
        Expr::Pow(expr1, expr2) => {
            preprocess_expr(&mut **expr1, scope);
            preprocess_expr(&mut **expr2, scope);
        }
        Expr::FunCall(expr1, args) => {
            preprocess_expr(&mut **expr1, scope);
            for arg in args {
                preprocess_expr(arg, scope);
            }
        }
        Expr::Comparison(expr1, _comp, expr2) => {
            preprocess_expr(&mut **expr1, scope);
            preprocess_expr(&mut **expr2, scope);
        }
        Expr::Not(expr) => {preprocess_expr(&mut **expr, scope)}
        Expr::And(expr1, expr2) => {
            preprocess_expr(&mut **expr1, scope);
            preprocess_expr(&mut **expr2, scope);
        }
        Expr::Or(expr1, expr2) => {
            preprocess_expr(&mut **expr1, scope);
            preprocess_expr(&mut **expr2, scope);
        }
    }
}


fn preprocess_defn(defn: &mut Define, scope: &mut AHashMap<String, ScopeInformation>) {
    match defn {
        Define::PlusEq(var, expr) => {
            add_var_def(var, scope);
            preprocess_expr(expr, scope);
        }
        Define::MinusEq(var, expr) => {
            add_var_def(var, scope);
            preprocess_expr(expr, scope);
        }
        Define::DivEq(var, expr) => {
            add_var_def(var, scope);
            preprocess_expr(expr, scope);
        }
        Define::MultEq(var, expr) => {
            add_var_def(var, scope);
            preprocess_expr(expr, scope);
        }
        Define::VarDefn(var, expr) => {
            add_var_def(var, scope);
            preprocess_expr(expr, scope);
        }
        Define::FunDefn(func, parameters, code, new_scope) => {
            add_var_def(func, scope);
            
            for param in parameters {
                add_var_def(param, new_scope);
            }
            
            preprocess_code(code, new_scope);
        }
    }
}