#![feature(fn_traits)]

mod parser;
mod verify_indents;
mod evaluator;
mod pyarena;
mod builtins;

use std::env;
use std::fs::File;
use std::io::Read;

use parser::python_parser;
use crate::evaluator::{evaluate};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut contents = String::new();
    
    if args.len() == 1 {
        contents = "  a=4\n  a + 2".to_string();

    } else if args.len() > 2 {
        panic!("Expect 1 arg for the test file name, got: {}", args.len() - 1);
    } else {
        let filename = &args[1];
        let mut file = File::open("tests/".to_string() + filename).expect(format!("file not found: {}", filename).as_str());

        let _ = file.read_to_string(&mut contents);
    }
    
    let parse_tree = python_parser::code(contents.trim(), 0);
    if let Ok(parse_tree) = parse_tree {
        println!("{:?}", parse_tree);
        
        // let mut arena = pyarena::PyArena::new();
        // arena.load_builtins();
        
        evaluate(parse_tree);
        
        
        
    } else if let Err(parse_tree_err) = parse_tree {
        println!("Char: \"{}\"({})\nError: {:?}", contents.chars().nth(parse_tree_err.location.offset).unwrap_or_default(), contents.bytes().nth(parse_tree_err.location.offset).unwrap_or_default(), parse_tree_err);
    }
    return;
}
