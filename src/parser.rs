use peg::*;

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

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Val(Value),
    Times(Box<Expr>,Box<Expr>),
    Divide(Box<Expr>,Box<Expr>),
    Plus(Box<Expr>,Box<Expr>),
    Minus(Box<Expr>,Box<Expr>),
    Pow(Box<Expr>,Box<Expr>),
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
    Return(Expr),
    Continue,
    Break,
}

#[derive(Debug)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
    pub depth: usize,
}

peg::parser!{
    pub grammar python_parser() for str {
        pub rule sp() = quiet!{" "*}
        pub rule sp1() = quiet!{" "+}
        pub rule nosp() = !" "
        pub rule nl() = quiet!{("\r\n" / "\n")+}
        pub rule ws() = quiet!{("\r\n" / "\n" / " ")*}

        pub rule indent(min: usize) = quiet!{" "*<{min}>}

        // recognizes a variable name
        pub rule id() -> String = s:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {s.to_string()}
        // recognizes a variable
        pub rule var() -> Expr = v:id() {Expr::Var(v)}

        pub rule float() -> f64 = n:$("-"? ['0'..='9']* "." ['0'..='9']+) {n.parse().unwrap()} / n:$("-"? ['0'..='9']+ "." ['0'..='9']*) {n.parse().unwrap()}
        pub rule integer() -> i64 = n:$("-"? ['0'..='9']+) {n.parse().unwrap()}
        pub rule string() -> String = "\"" s:$([^('\n' | '"')]*) "\"" {s.to_string()}  // TODO make string match correct
        pub rule boolean() -> bool = $"True" {true} / $"False" {false}
        pub rule none() -> Value = "None" {Value::None}

        pub rule val() -> Value = f:float() {Value::Float(f)} / i:integer() {Value::Integer(i)} / s:string() {Value::String(s)} / b:boolean() {Value::Boolean(b)} / n:none() {n}

        pub rule expr() -> Expr = precedence!{
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

        pub rule define() -> Define = //precedence!{
            // "def" sp1() f:id() sp() "(" sp() args:(id() ** (sp() "," sp())) sp() ")" sp() ": \n" sp() e:expr() {Define::FunDefn(f, args, e)} /
            v:id() sp() "=" sp() e:expr() {Define::VarDefn(v, e)}
            / v:id() sp() "+=" sp() e:expr() {Define::PlusEq(v, e)}
            / v:id() sp() "-=" sp() e:expr() {Define::MinusEq(v, e)}
            / v:id() sp() "/=" sp() e:expr() {Define::DivEq(v, e)}
            / v:id() sp() "*=" sp() e:expr() {Define::MultEq(v, e)}
        //}

        // pub rule statement(depth: usize) -> Statement = spaces:" "* d:define() {Statement::Defn(d, depth + spaces.len())}
        // / spaces:" "* e:expr() {Statement::Expr(e, depth + spaces.len())}

        pub rule statement(depth: usize) -> Statement =
        nosp() "for" sp1() v:id() sp1() "in" sp1() e:expr() sp() ":" sp() nl() c:code(depth + 1) {Statement::For(v, e, c)}
        / nosp() "return" sp() e:expr() {Statement::Return(e)}
        / nosp() "return" sp() {Statement::Return(Expr::Val(Value::None))}  // empty "return" statement
        / nosp() "continue" {Statement::Continue}
        / nosp() "break" {Statement::Break}
        / nosp() d:define() {Statement::Defn(d)}
        / nosp() e:expr() {Statement::Expr(e)}

        // pub rule code(depth: usize) -> CodeBlock = &" "*<{depth}> spaces:" "*<{depth},> s:(statement(depth) ** nl()) sp() {CodeBlock::Block(s)}
        pub rule code(depth: usize) -> CodeBlock = spaces:" "*<{depth},> statements:(statement(depth) ** (nl() ws() indent(spaces.len()))) {CodeBlock{statements, depth: spaces.len()}}
    }
}

