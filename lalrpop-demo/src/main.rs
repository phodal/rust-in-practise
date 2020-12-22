#[macro_use] extern crate lalrpop_util;

pub mod lexer;
pub mod pt;
pub mod func;
pub mod token;

fn main() {
    let _parser = func::FuncParser::new();
}