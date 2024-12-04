// use crate::parser::*;
// 
// pub fn verify_indents(code: CodeBlock) {
//     todo!("Not necessary atm");
//     let block_indent_level = code.depth;
// 
//     for statement in code.statements.iter() {
//         match statement {
//             Statement::Expr(_, depth) => {
//                 if *depth != code.depth {
//                     panic!("Indentation error: expected depth {}, got {}", code.depth, depth);
//                 }
//             }
//             Statement::Defn(_, depth) => {
//                 if *depth != code.depth {
//                     panic!("Indentation error: expected depth {}, got {}", code.depth, depth);
//                 }
//             }
//         }
//     }
// }