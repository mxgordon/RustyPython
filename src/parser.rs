use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::env::var;
use peg::*;
use ahash::AHashMap;
use peg::error::ParseError;
use peg::str::LineCol;

pub fn remove_comments(input: &str) -> String {
    let mut output = String::new();
    let mut in_comment = false;
    for c in input.chars() {
        if in_comment {
            if c == '\n' {
                in_comment = false;
                output.push(c);
            }
        } else if c == '#' {
            in_comment = true;
        } else {
            output.push(c);
        }
    }
    output
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}

pub fn parse_code(code: &str) -> Result<ScopedCodeBlock, ParseError<LineCol>> {
    let variables = RefCell::new(AHashMap::new());
    let fast_local_count = RefCell::new(0);

    python_parser::code_traced(code, &variables, &fast_local_count)
}


fn remove_from_fast_locals(mut variables: RefMut<AHashMap<String, Rc<Variable>>>, mut variable: Rc<Variable>) {
    let removed_number = variable.fast_locals_loc.expect("Variable not in fast locals");

    unsafe {
        Rc::get_mut_unchecked(&mut variable).fast_locals_loc = None;
    }
    
    for (_name, mut variable) in variables.iter_mut() {
        if let Some(fast_locals_loc) = variable.fast_locals_loc {
            if fast_locals_loc > removed_number {
                unsafe {
                    Rc::get_mut_unchecked(&mut variable).fast_locals_loc = Some(fast_locals_loc - 1);
                }
            }
        }
    }
}

fn add_to_fast_locals(mut variable: Rc<Variable>, fast_locals_size: &mut usize) {
    unsafe {
        Rc::get_mut_unchecked(&mut variable).fast_locals_loc = Some(*fast_locals_size);
    }
    
    *fast_locals_size += 1;
}



parser! {
    pub grammar python_parser() for str {
        rule sp() = quiet!{" "*}
        rule sp1() = quiet!{" "+}
        rule nosp() = !" "
        rule nl() = "\r\n" / "\n" / ";"
        rule next_line() = sp() (nl() **<1,> sp())
        // rule ws() = quiet!{("\r\n" / "\n" / " ")*}

        rule indent(min: usize) = quiet!{" "*<{min}>}

        // recognizes a variable name
        rule id() -> String = s:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {s.to_string()}
        // recognizes a variable
        // rule var() -> Expr = v:id() {Expr::Var(v)}
        rule var(variables: &RefCell<AHashMap<String, Rc<Variable>>>, fast_local_count: &RefCell<usize>, is_def: bool) -> Rc<Variable> = name:id() {
            let mut variables = variables.borrow_mut();
            let mut fast_local_count = fast_local_count.borrow_mut();

            let variable = variables.get(&name).cloned();

            if name == "print" {
                println!("print: {is_def}");
            }

            if let Some(variable) = variable {
                // if is_def && variable.fast_locals_loc.is_none() {
                //     add_to_fast_locals(variable.clone(), &mut fast_local_count);
                // } else if !is_def && variable.fast_locals_loc.is_some() {
                //     remove_from_fast_locals(variables, variable.clone());
                // }
                
                return variable.clone()
            } 
            
            if is_def {
                let new_var = Rc::new(Variable {
                    name: name.clone(),
                    fast_locals_loc: Some(*fast_local_count),
                });

                *fast_local_count += 1;

                variables.insert(name, new_var.clone());

                return new_var;
            }

            let new_var = Rc::new(Variable {
                name: name.clone(),
                fast_locals_loc: None,
            });

            variables.insert(name, new_var.clone());

            new_var
        }

        rule float() -> f64 = n:$("-"? ['0'..='9']* "." ['0'..='9']+) {n.parse().unwrap()} / n:$("-"? ['0'..='9']+ "." ['0'..='9']*) {n.parse().unwrap()}
        rule integer() -> i64 = n:$("-"? ['0'..='9']+) {n.parse().unwrap()}
        rule string() -> String = "\"" s:$([^('\n' | '"')]*) "\"" {s.to_string()}  // TODO make string match correct
        rule boolean() -> bool = $"True" {true} / $"False" {false}
        rule none() -> Value = "None" {Value::None}

        rule val() -> Value = f:float() {Value::Float(f)} / i:integer() {Value::Integer(i)} / s:string() {Value::String(s)} / b:boolean() {Value::Boolean(b)} / n:none() {n}

        rule expr(vars: &RefCell<AHashMap<String, Rc<Variable>>>, fl_cnt: &RefCell<usize>) -> Expr = precedence!{
            // comparisons ==, !=, >, >=, <, <=, is, is not, in, not in
            l:(@) sp() "==" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::Equal, Box::new(r))}
            l:(@) sp() "!=" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::NotEqual, Box::new(r))}
            l:(@) sp() ">=" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::GreaterThanOrEqual, Box::new(r))}
            l:(@) sp() "<=" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::LessThanOrEqual, Box::new(r))}
            l:(@) sp() ">" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::GreaterThan, Box::new(r))}
            l:(@) sp() "<" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::LessThan, Box::new(r))}
            l:(@) sp() "==" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::Equal, Box::new(r))}
            l:(@) sp1() "is" sp1() r:@ {Expr::Comparison(Box::new(l), Comparitor::Is, Box::new(r))}
            l:(@) sp1() "is" sp1() "not" sp1() r:@ {Expr::Comparison(Box::new(l), Comparitor::IsNot, Box::new(r))}
            l:(@) sp1() "in" sp1() r:@ {Expr::Comparison(Box::new(l), Comparitor::In, Box::new(r))}
            l:(@) sp1() "not" sp1() "in" sp1() r:@ {Expr::Comparison(Box::new(l), Comparitor::NotIn, Box::new(r))}
            --
            // bitwise |
            // bitwise ^
            // bitwise &
            // Bitwise shifts here
            l:(@) sp() "+" sp() r:@ {Expr::Plus(Box::new(l), Box::new(r))}
            l:(@) sp() "-" sp() r:@ {Expr::Minus(Box::new(l), Box::new(r))}
            --
            l:(@) sp() "*" sp() r:@ {Expr::Times(Box::new(l), Box::new(r))}  // include @, //, and %
            l:(@) sp() "/" sp() r:@ {Expr::Divide(Box::new(l), Box::new(r))}
            -- // Unary ops here
            l:@ sp() "**" sp() r:(@) {Expr::Pow(Box::new(l), Box::new(r))}
            --
            l:(@) sp1() "and" sp1() r:@ {Expr::And(Box::new(l), Box::new(r))}
            l:(@) sp1() "or" sp1() r:@ {Expr::Or(Box::new(l), Box::new(r))}
            --
            "not" sp1() v:expr(vars, fl_cnt) {Expr::Not(Box::new(v))}
            --
            v:val() {Expr::Val(v)}
            f:var(vars, fl_cnt, false) sp() "(" sp() args:(expr(vars, fl_cnt) ** (sp() "," sp())) sp() ")" {Expr::FunCall(Box::new(Expr::Var(f)), args)} // `f` should be an expression for more robuts parsing (might create loop(?))
            v:var(vars, fl_cnt, false) {Expr::Var(v)}
            --
            "(" e:expr(vars, fl_cnt) ")" {e}
        }

        rule function_definition(depth: usize, vars: RefCell<AHashMap<String, Rc<Variable>>>, fl_cnt: RefCell<usize>) -> Define =
            "def" sp1() f:var(&vars, &fl_cnt, true) sp() "(" sp() args:(var(&vars, &fl_cnt, true) ** (sp() "," sp())) sp() ")" sp() ":" next_line() c:scoped_code(depth+1, &vars, &fl_cnt) {Define::FunDefn(f, args, c)}


        rule define(depth: usize, vars: &RefCell<AHashMap<String, Rc<Variable>>>, fl_cnt: &RefCell<usize>) -> Define =
            func:function_definition(depth, RefCell::new(AHashMap::new()), RefCell::new(0)) {func}
            / v:var(vars, fl_cnt, true) sp() "=" sp() e:expr(vars, fl_cnt) {Define::VarDefn(v, e)}
            / v:var(vars, fl_cnt, true) sp() "+=" sp() e:expr(vars, fl_cnt) {Define::PlusEq(v, e)}
            / v:var(vars, fl_cnt, true) sp() "-=" sp() e:expr(vars, fl_cnt) {Define::MinusEq(v, e)}
            / v:var(vars, fl_cnt, true) sp() "/=" sp() e:expr(vars, fl_cnt) {Define::DivEq(v, e)}
            / v:var(vars, fl_cnt, true) sp() "*=" sp() e:expr(vars, fl_cnt) {Define::MultEq(v, e)}

        rule if_(depth: usize, vars: &RefCell<AHashMap<String, Rc<Variable>>>, fl_cnt: &RefCell<usize>) -> Statement =
            "if" sp1() cond:expr(vars, fl_cnt) sp() ":" next_line() if_code:code(depth + 1, vars, fl_cnt)
            elif:(next_line() indent(depth) "elif" sp1() elif_cond:expr(vars, fl_cnt) sp() ":" next_line() elif_code:code(depth+1, vars, fl_cnt) {(elif_cond, elif_code)})*
            else_code:(next_line() indent(depth) "else" sp() ":" next_line() else_code:code(depth+1, vars, fl_cnt) {else_code})? {Statement::If(cond, if_code, elif, else_code)}

        rule statement(depth: usize, vars: &RefCell<AHashMap<String, Rc<Variable>>>, fl_cnt: &RefCell<usize>) -> Statement =
            if_statement:if_(depth, vars, fl_cnt) {if_statement}
            / "for" sp1() v:var(vars, fl_cnt, true) sp1() "in" sp1() e:expr(vars, fl_cnt) sp() ":" next_line() c:code(depth + 1, vars, fl_cnt) {Statement::For(v, e, c)}
            / "while" sp1() e:expr(vars, fl_cnt) sp() ":" next_line() c:code(depth + 1, vars, fl_cnt) {Statement::While(e, c)}
            / "assert" sp1() e1:expr(vars, fl_cnt) e2:("," sp() e:expr(vars, fl_cnt) {e})?  {Statement::Assert(e1, e2)}
            / "return" sp1() e:expr(vars, fl_cnt) {Statement::Return(e)}
            / "return" sp() {Statement::Return(Expr::Val(Value::None))}  // empty "return" statement
            / "continue" {Statement::Continue}
            / "break" {Statement::Break}
            / d:define(depth, vars, fl_cnt) {Statement::Defn(d)}
            / e:expr(vars, fl_cnt) {Statement::Expr(e)}

        // pub rule code(depth: usize) -> CodeBlock = &" "*<{depth}> spaces:" "*<{depth},> s:(statement(depth) ** nl()) sp() {CodeBlock::Block(s)}
        pub rule code(depth: usize, vars: &RefCell<AHashMap<String, Rc<Variable>>>, fl_cnt: &RefCell<usize>) -> CodeBlock =
            spaces:" "*<{depth},> statements:(statement(depth, vars, fl_cnt) ** (next_line() indent(spaces.len()) nosp())) {CodeBlock{statements, depth: spaces.len()}}

        pub rule scoped_code(depth: usize, vars: &RefCell<AHashMap<String, Rc<Variable>>>, fl_cnt: &RefCell<usize>) -> ScopedCodeBlock = c:code(depth, vars, fl_cnt) {ScopedCodeBlock{code: c, fast_local_size: *fl_cnt.borrow()}}

        rule traced<T>(e: rule<T>) -> T =
            &(input:$([_]*) {
                #[cfg(feature = "trace")]
                println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", input);
            })
            e:e()? {?
                #[cfg(feature = "trace")]
                println!("[PEG_TRACE_STOP]");
                e.ok_or("")
            }

        pub rule code_traced(vars: &RefCell<AHashMap<String, Rc<Variable>>>, fl_cnt: &RefCell<usize>) -> ScopedCodeBlock = traced(<scoped_code(0, vars, fl_cnt)>)
    }
}

#[derive(Debug)]
pub enum Comparitor {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Is,
    IsNot,
    In,
    NotIn,
}

#[derive(Debug)]
pub enum Expr {
    Var(Rc<Variable>),
    Val(Value),
    Times(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    FunCall(Box<Expr>, Vec<Expr>),
    Comparison(Box<Expr>, Comparitor, Box<Expr>),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub fast_locals_loc: Option<usize>,
}

#[derive(Debug)]
pub enum Define {
    PlusEq(Rc<Variable>, Expr),
    MinusEq(Rc<Variable>, Expr),
    DivEq(Rc<Variable>, Expr),
    MultEq(Rc<Variable>, Expr),
    VarDefn(Rc<Variable>, Expr),
    FunDefn(Rc<Variable>, Vec<Rc<Variable>>, ScopedCodeBlock),
}

#[derive(Debug)]
pub enum Statement {
    Expr(Expr),
    Defn(Define),
    For(Rc<Variable>, Expr, CodeBlock), // TODO allow for variable unpacking
    While(Expr, CodeBlock),       // TODO allow for else block
    If(Expr, CodeBlock, Vec<(Expr, CodeBlock)>, Option<CodeBlock>), // IfCond, Code, (ElIfCond, Code), ElseCode
    Return(Expr),
    Assert(Expr, Option<Expr>),
    Continue,
    Break,
}

#[derive(Debug)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
    pub depth: usize,
}

#[derive(Debug)]
pub struct ScopedCodeBlock {
    pub code: CodeBlock,
    pub fast_local_size: usize
}