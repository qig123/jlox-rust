use std::{env, fs};

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
mod environment;
mod expr;
mod interpreter;
mod parser;
mod report;
mod scanner;
mod token;
mod token_type;
fn main() {
    let args: Vec<String> = env::args().collect();
    // let args: Vec<&'static str> = vec!["name", r"./test.lox"]; //这行是测试代码
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
    let tokens = scanner.scan_tokens(source.clone()); // Clone source for parser
    match tokens {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens, source); // Pass source to parser
            match parser.parse() {
                Ok(expr) => match Interpreter::new().interpret(expr) {
                    Ok(_obj) => {}
                    Err(e) => {
                        eprintln!("Runtime error: {} at line {}", e.message, e.line)
                    }
                },
                Err(_e) => {
                    //语法分析出现错误，简单退出进程
                    std::process::exit(65);
                }
            }
        }
        Err(_e) => {
            //词法分析出现错误，简单退出进程
            std::process::exit(65);
        }
    }
}
