use std::{env, fs};
mod report;
mod scanner;
mod token;
mod token_type;
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            run_file(args[1].clone());
        }
        _ => {
            panic!("Usage: jlox-rust filename");
        }
    }
}
fn run_file(path: String) {
    let x = fs::read_to_string(path).expect("lox文件读取失败");
    run(x);
}
fn run(source: String) {
    println!("{}", source);
}
