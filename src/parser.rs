use peg::*;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Var(String),
    Val(Value),
    Times(Box<Expr>,Box<Expr>),
    Divide(Box<Expr>,Box<Expr>),
    Plus(Box<Expr>,Box<Expr>),
    Minus(Box<Expr>,Box<Expr>),
    Pow(Box<Expr>,Box<Expr>),
    FunCall(Box<Expr>, Vec<Expr>),
}

#[derive(Clone, Debug)]
pub enum Define {
    PlusEq(String, Expr),
    MinusEq(String, Expr),
    DivEq(String, Expr),
    MultEq(String, Expr),
    VarDefn(String, Expr),
    FunDefn(String, Vec<String>, CodeBlock),
}


#[derive(Clone, Debug)]
pub enum Statement {
    Expr(Expr),
    Defn(Define),
    For(String, Expr, CodeBlock) // allow for variable unpacking
}

// #[derive(Clone, Debug)]
// pub enum CodeBlock {
//     Block(Vec<Statement>, usize),
// }
#[derive(Clone, Debug)]
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
        pub rule string() -> String = "\"" s:$([_]*) "\"" {s.to_string()}  // TODO make string match correct
        pub rule boolean() -> bool = $"True" {true} / $"False" {false}
        pub rule none() -> Value = "None" {Value::None}

        pub rule val() -> Value = f:float() {Value::Float(f)} / i:integer() {Value::Integer(i)} / s:string() {Value::String(s)} / b:boolean() {Value::Boolean(b)} / n:none() {n}

        pub rule expr() -> Expr = precedence!{
            l:(@) sp() "+" sp() r:@ {Expr::Plus(Box::new(l), Box::new(r))}
            l:(@) sp() "-" sp() r:@ {Expr::Minus(Box::new(l), Box::new(r))}
            --
            l:(@) sp() "*" sp() r:@ {Expr::Times(Box::new(l), Box::new(r))}
            l:(@) sp() "/" sp() r:@ {Expr::Divide(Box::new(l), Box::new(r))}
            --
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
        / nosp() d:define() {Statement::Defn(d)}
        / nosp() e:expr() {Statement::Expr(e)}

        // pub rule code(depth: usize) -> CodeBlock = &" "*<{depth}> spaces:" "*<{depth},> s:(statement(depth) ** nl()) sp() {CodeBlock::Block(s)}
        pub rule code(depth: usize) -> CodeBlock = spaces:" "*<{depth},> statements:(statement(depth) ** (nl() ws() indent(spaces.len()))) {CodeBlock{statements, depth: spaces.len()}}
    }
}

