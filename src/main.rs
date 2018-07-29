#![macro_use]

// defines macros, has to be declared first!
mod parser;

mod eval;

use parser::*;

fn main() {
    let term = ast!((# x -> x x) (# x -> x x)).eval();
    println!("ast!({}) == {:?} ==> {}", term, term, term.clone().eval());
}
