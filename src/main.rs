#![feature(fn_traits)]
// #![feature(let_chains)]
#![feature(hash_raw_entry)]
#![feature(get_mut_unchecked)]

mod pyarena;
mod builtins;
mod new_evaluator;

use std::env;
use std::fs::File;
use std::io::Read;
use rustpython_parser::{lexer::{lex}, parse_tokens, Mode};
use crate::new_evaluator::evaluate_mod;

#[macro_use]
extern crate mopa;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    
    let mut contents = String::new();
    
    if args.len() == 1 {
        // contents = "  a=4\n  a + 2".to_string();
        println!("No target file");
        return;

    } else if args.len() > 2 {
        panic!("Expect 1 arg for the test file name, got: {}", args.len() - 1);
    } 
    let filename = &args[1];
    let file_path = "tests/".to_string() + filename;
    let mut file = File::open(file_path.clone()).unwrap_or_else(|_| panic!("file not found: {}", filename));
    
    {
        let _ = file.read_to_string(&mut contents);
    }
    
    let tokens = lex(&contents, Mode::Module);
    let ast =  parse_tokens(tokens, Mode::Module, &file_path);

    if let Ok(ast) = ast {
        
        // println!("{:?}", ast);
        
        evaluate_mod(ast);
        
    } else if let Err(parse_error) = ast {
        // println!("Char: \"{}\"({})\nError: {:?}", contents.chars().nth(parse_tree_err.location.offset).unwrap_or_default(), contents.bytes().nth(parse_tree_err.location.offset).unwrap_or_default(), parse_tree_err);
        println!("{:?}", parse_error);
    }
}
