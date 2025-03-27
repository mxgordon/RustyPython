#![feature(fn_traits)]
// #![feature(let_chains)]
#![feature(hash_raw_entry)]
#![feature(get_mut_unchecked)]

mod parser;
mod evaluator;
mod pyarena;
mod builtins;
mod preprocessor;

use std::env;
use std::fs::File;
use std::io::Read;
use crate::evaluator::{evaluate};
use crate::parser::{parse_code, remove_comments};

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
    } else {
        let filename = &args[1];
        let mut file = File::open("tests/".to_string() + filename).unwrap_or_else(|_| panic!("file not found: {}", filename));

        let _ = file.read_to_string(&mut contents);
    }
    let contents = remove_comments(&contents);
    let contents = contents.trim();
    
    let parse_tree = parse_code(contents);
    if let Ok(parse_tree) = parse_tree {
        
        println!("{:?}", parse_tree);
        
        evaluate(parse_tree);
        
    } else if let Err(parse_tree_err) = parse_tree {
        println!("Char: \"{}\"({})\nError: {:?}", contents.chars().nth(parse_tree_err.location.offset).unwrap_or_default(), contents.bytes().nth(parse_tree_err.location.offset).unwrap_or_default(), parse_tree_err);
    }
}
