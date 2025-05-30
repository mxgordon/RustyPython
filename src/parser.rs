use std::cell::RefCell;
use peg::*;
use peg::error::ParseError;
use peg::str::LineCol;
use std::rc::Rc;
use ahash::AHashMap;

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

pub fn parse_code(code: &str) -> (Result<CodeBlock, ParseError<LineCol>>, AHashMap<String, ScopeInformation>) {
    let variables = RefCell::new(AHashMap::new());

    let code_result = python_parser::code_traced(code, &variables);

    (code_result, variables.into_inner())
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
        rule var(variables: &RefCell<AHashMap<String, ScopeInformation>>) -> Rc<Variable> = name:id() {
            name
            // let mut variables = variables.borrow_mut();
            // 
            // let scope_info_option = variables.get(&name).cloned();
            // 
            // let scope_info = scope_info_option.unwrap_or_else(|| {
            //     let scope_info = ScopeInformation {
            //         variable: Rc::new(Variable {
            //             name: name.clone(),
            //             fast_locals_location: None
            //         }),
            //         uses: 0,
            //         has_definition: false
            //     };
            //     
            //     variables.insert(name, scope_info.clone());
            //     
            //     scope_info
            // });
            // 
            // scope_info.variable.clone()
        }

        rule float() -> f64 = n:$("-"? ['0'..='9']* "." ['0'..='9']+) {n.parse().unwrap()} / n:$("-"? ['0'..='9']+ "." ['0'..='9']*) {n.parse().unwrap()}
        rule integer() -> i64 = n:$("-"? ['0'..='9']+) {n.parse().unwrap()}
        rule string() -> String = "\"" s:$([^('\n' | '"')]*) "\"" {s.to_string()}  // TODO make string match correct
        rule boolean() -> bool = $"True" {true} / $"False" {false}
        rule none() -> Value = "None" {Value::None}

        rule val() -> Value = f:float() {Value::Float(f)} / i:integer() {Value::Integer(i)} / s:string() {Value::String(s)} / b:boolean() {Value::Boolean(b)} / n:none() {n}

        rule expr(vars: &RefCell<AHashMap<String, ScopeInformation>>) -> Expr = precedence!{
            // comparisons ==, !=, >, >=, <, <=, is, is not, in, not in
            l:(@) sp() "==" sp() r:@ {Expr::Comparison(Box::new(l), Comparator::Equal, Box::new(r))}
            l:(@) sp() "!=" sp() r:@ {Expr::Comparison(Box::new(l), Comparator::NotEqual, Box::new(r))}
            l:(@) sp() ">=" sp() r:@ {Expr::Comparison(Box::new(l), Comparator::GreaterThanOrEqual, Box::new(r))}
            l:(@) sp() "<=" sp() r:@ {Expr::Comparison(Box::new(l), Comparator::LessThanOrEqual, Box::new(r))}
            l:(@) sp() ">" sp() r:@ {Expr::Comparison(Box::new(l), Comparator::GreaterThan, Box::new(r))}
            l:(@) sp() "<" sp() r:@ {Expr::Comparison(Box::new(l), Comparator::LessThan, Box::new(r))}
            l:(@) sp() "==" sp() r:@ {Expr::Comparison(Box::new(l), Comparator::Equal, Box::new(r))}
            l:(@) sp1() "is" sp1() r:@ {Expr::Comparison(Box::new(l), Comparator::Is, Box::new(r))}
            l:(@) sp1() "is" sp1() "not" sp1() r:@ {Expr::Comparison(Box::new(l), Comparator::IsNot, Box::new(r))}
            l:(@) sp1() "in" sp1() r:@ {Expr::Comparison(Box::new(l), Comparator::In, Box::new(r))}
            l:(@) sp1() "not" sp1() "in" sp1() r:@ {Expr::Comparison(Box::new(l), Comparator::NotIn, Box::new(r))}
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
            "not" sp1() v:expr(vars) {Expr::Not(Box::new(v))}
            --
            v:val() {Expr::Val(v)}
            f:var(vars) sp() "(" sp() args:(expr(vars) ** (sp() "," sp())) sp() ")" {Expr::FunCall(Box::new(Expr::Var(f)), args)} // `f` should be an expression for more robuts parsing (might create loop(?))
            v:var(vars) {Expr::Var(v)}
            --
            "(" e:expr(vars) ")" {e}
        }

        rule function_definition(depth: usize, vars: RefCell<AHashMap<String, ScopeInformation>>) -> Define =
            "def" sp1() f:var(&vars) sp() "(" sp() args:(var(&vars) ** (sp() "," sp())) sp() ")" sp() ":" next_line() c:code(depth+1, &vars) {Define::FunDefn(f, args, c, vars.into_inner())}


        rule define(depth: usize, vars: &RefCell<AHashMap<String, ScopeInformation>>) -> Define =
            func:function_definition(depth, RefCell::new(AHashMap::new())) {func}
            / v:var(vars) sp() "=" sp() e:expr(vars) {Define::VarDefn(v, e)}
            / v:var(vars) sp() "+=" sp() e:expr(vars) {Define::PlusEq(v, e)}
            / v:var(vars) sp() "-=" sp() e:expr(vars) {Define::MinusEq(v, e)}
            / v:var(vars) sp() "/=" sp() e:expr(vars) {Define::DivEq(v, e)}
            / v:var(vars) sp() "*=" sp() e:expr(vars) {Define::MultEq(v, e)}

        rule if_(depth: usize, vars: &RefCell<AHashMap<String, ScopeInformation>>) -> Statement =
            "if" sp1() cond:expr(vars) sp() ":" next_line() if_code:code(depth + 1, vars)
            elif:(next_line() indent(depth) "elif" sp1() elif_cond:expr(vars) sp() ":" next_line() elif_code:code(depth+1, vars) {(elif_cond, elif_code)})*
            else_code:(next_line() indent(depth) "else" sp() ":" next_line() else_code:code(depth+1, vars) {else_code})? {Statement::If(cond, if_code, elif, else_code)}

        rule statement(depth: usize, vars: &RefCell<AHashMap<String, ScopeInformation>>) -> Statement =
            if_statement:if_(depth, vars) {if_statement}
            / "for" sp1() v:var(vars) sp1() "in" sp1() e:expr(vars) sp() ":" next_line() c:code(depth + 1, vars) {Statement::For(v, e, c)}
            / "while" sp1() e:expr(vars) sp() ":" next_line() c:code(depth + 1, vars) {Statement::While(e, c)}
            / "assert" sp1() e1:expr(vars) e2:("," sp() e:expr(vars) {e})?  {Statement::Assert(e1, e2)}
            / "return" sp1() e:expr(vars) {Statement::Return(e)}
            / "return" sp() {Statement::Return(Expr::Val(Value::None))}  // empty "return" statement
            / "continue" {Statement::Continue}
            / "break" {Statement::Break}
            / d:define(depth, vars) {Statement::Defn(d)}
            / e:expr(vars) {Statement::Expr(e)}

        // pub rule code(depth: usize) -> CodeBlock = &" "*<{depth}> spaces:" "*<{depth},> s:(statement(depth) ** nl()) sp() {CodeBlock::Block(s)}
        pub rule code(depth: usize, vars: &RefCell<AHashMap<String, ScopeInformation>>) -> CodeBlock =
            spaces:" "*<{depth},> statements:(statement(depth, vars) ** (next_line() indent(spaces.len()) nosp())) {CodeBlock{statements, depth: spaces.len()}}

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

        pub rule code_traced(vars: &RefCell<AHashMap<String, ScopeInformation>>) -> CodeBlock = traced(<code(0, vars)>)
    }
}

#[derive(Debug)]
pub enum Comparator {
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
    Var(String),
    Val(Value),
    Times(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    FunCall(Box<Expr>, Vec<Expr>),
    Comparison(Box<Expr>, Comparator, Box<Expr>),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub fast_locals_location: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ScopeInformation {
    pub variable: Rc<Variable>,
    pub uses: usize,
    pub has_definition: bool
}

#[derive(Debug)]
pub enum Define {
    PlusEq(Rc<Variable>, Expr),
    MinusEq(Rc<Variable>, Expr),
    DivEq(Rc<Variable>, Expr),
    MultEq(Rc<Variable>, Expr),
    VarDefn(Rc<Variable>, Expr),
    FunDefn(Rc<Variable>, Vec<Rc<Variable>>, CodeBlock, AHashMap<String, ScopeInformation>),
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