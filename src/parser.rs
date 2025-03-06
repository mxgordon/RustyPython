use peg::*;

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
        rule var() -> Expr = v:id() {Expr::Var(v)}

        rule float() -> f64 = n:$("-"? ['0'..='9']* "." ['0'..='9']+) {n.parse().unwrap()} / n:$("-"? ['0'..='9']+ "." ['0'..='9']*) {n.parse().unwrap()}
        rule integer() -> i64 = n:$("-"? ['0'..='9']+) {n.parse().unwrap()}
        rule string() -> String = "\"" s:$([^('\n' | '"')]*) "\"" {s.to_string()}  // TODO make string match correct
        rule boolean() -> bool = $"True" {true} / $"False" {false}
        rule none() -> Value = "None" {Value::None}

        rule val() -> Value = f:float() {Value::Float(f)} / i:integer() {Value::Integer(i)} / s:string() {Value::String(s)} / b:boolean() {Value::Boolean(b)} / n:none() {n}

        rule expr() -> Expr = precedence!{
            // logical or
            // logical and
            // logical not
            // comparisons ==, !=, >, >=, <, <=, is, is not, in, not in
            l:(@) sp() "==" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::Equal, Box::new(r))}
            l:(@) sp() "!=" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::NotEqual, Box::new(r))}
            l:(@) sp() ">=" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::GreaterThanOrEqual, Box::new(r))}
            l:(@) sp() "<=" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::LessThanOrEqual, Box::new(r))}
            l:(@) sp() ">" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::GreaterThan, Box::new(r))}
            l:(@) sp() "<" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::LessThan, Box::new(r))}
            l:(@) sp() "==" sp() r:@ {Expr::Comparison(Box::new(l), Comparitor::Equal, Box::new(r))}
            l:(@) sp1() "is" sp1() r:@ {Expr::Comparison(Box::new(l), Comparitor::Is, Box::new(r))}
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
            v:val() {Expr::Val(v)}
            f:var() sp() "(" sp() args:(expr() ** (sp() "," sp())) sp() ")" {Expr::FunCall(Box::new(f), args)}
            v:var() {v}
            --
            "(" e:expr() ")" {e}
        }

        rule define() -> Define = //precedence!{
            // "def" sp1() f:id() sp() "(" sp() args:(id() ** (sp() "," sp())) sp() ")" sp() ": \n" sp() e:expr() {Define::FunDefn(f, args, e)} /
            v:id() sp() "=" sp() e:expr() {Define::VarDefn(v, e)}
            / v:id() sp() "+=" sp() e:expr() {Define::PlusEq(v, e)}
            / v:id() sp() "-=" sp() e:expr() {Define::MinusEq(v, e)}
            / v:id() sp() "/=" sp() e:expr() {Define::DivEq(v, e)}
            / v:id() sp() "*=" sp() e:expr() {Define::MultEq(v, e)}
        //}

        rule if_(depth: usize) -> Statement = 
            "if" sp1() cond:expr() sp() ":" next_line() if_code:code(depth + 1)
            elif:(indent(depth) "elif" sp1() elif_cond:expr() sp() ":" next_line() elif_code:code(depth+1) {(elif_cond, elif_code)})* 
            else_code:(indent(depth) "else" sp() ":" next_line() else_code:code(depth+1) {else_code})? {Statement::If(cond, if_code, elif, else_code)}

        rule statement(depth: usize) -> Statement =
            if_statement:if_(depth) {if_statement}
            / "for" sp1() v:id() sp1() "in" sp1() e:expr() sp() ":" next_line() c:code(depth + 1) {Statement::For(v, e, c)}
            / "while" sp1() e:expr() sp() ":" next_line() c:code(depth + 1) {Statement::While(e, c)}
            / "assert" sp1() e1:expr() e2:("," sp() e:expr() {e})?  {Statement::Assert(e1, e2)}
            / "return" sp1() e:expr() {Statement::Return(e)}
            / "return" sp() {Statement::Return(Expr::Val(Value::None))}  // empty "return" statement
            / "continue" {Statement::Continue}
            / "break" {Statement::Break}
            / d:define() {Statement::Defn(d)}
            / e:expr() {Statement::Expr(e)}

        // pub rule code(depth: usize) -> CodeBlock = &" "*<{depth}> spaces:" "*<{depth},> s:(statement(depth) ** nl()) sp() {CodeBlock::Block(s)}
        pub rule code(depth: usize) -> CodeBlock = spaces:" "*<{depth},> statements:(statement(depth) ** (next_line() indent(spaces.len()) nosp())) {CodeBlock{statements, depth: spaces.len()}}


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
        
        pub rule code_traced() -> CodeBlock = traced(<code(0)>)
    }
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
    Comparison(Box<Expr>, Comparitor, Box<Expr>),
}

#[derive(Debug)]
pub enum Define {
    PlusEq(String, Expr),
    MinusEq(String, Expr),
    DivEq(String, Expr),
    MultEq(String, Expr),
    VarDefn(String, Expr),
    FunDefn(String, Vec<String>, CodeBlock),
}

#[derive(Debug)]
pub enum Statement {
    Expr(Expr),
    Defn(Define),
    For(String, Expr, CodeBlock), // TODO allow for variable unpacking
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
