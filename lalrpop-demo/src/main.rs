#[macro_use] extern crate lalrpop_util;

pub mod lexer;
pub mod pt;
pub mod func;
pub mod token;

fn main() {
    let result = func::FuncParser::new()
        .parse("((((a))))")
        .unwrap();

    println!("{:?}", result);

    let result = func::FuncParser::new()
        .parse("a()")
        .unwrap();

    println!("{:?}", result);
}