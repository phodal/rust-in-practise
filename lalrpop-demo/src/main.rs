pub mod lexer;
pub mod pt;
pub mod func;
pub mod token;

fn main() {
    let _result = func::FuncParser::new()
        .parse("((((a))))")
        .unwrap();

    let _result = func::FuncParser::new()
        .parse("a()")
        .unwrap();

    let result = func::FuncParser::new()
        .parse("!a()")
        .unwrap();

    println!("{:?}", result);
}