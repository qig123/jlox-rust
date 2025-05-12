use std::{env, fs};

use parser::Parser;
use scanner::Scanner;
mod expr;
mod parser;
mod report;
mod scanner;
mod token;
mod token_type;
fn main() {
    //let args: Vec<String> = env::args().collect();
    let args = vec!["name", r"./test.lox"]; //这行是测试代码
    match args.len() {
        2 => {
            run_file(args[1].to_string());
        }
        _ => {
            panic!("Usage: jlox-rust filename");
        }
    }
}
fn run_file(path: String) {
    let f = fs::read_to_string(path).expect("lox文件读取失败");
    run(f);
}
fn run(source: String) {
    let mut scanner = Scanner::new();
    let tokens = scanner.scan_tokens(source);
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(expr) => println!("Parsed expression: {}", expr),
        Err(e) => eprintln!("Parse error: {:?}", e),
    }
}
